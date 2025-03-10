use std::{
    borrow::Borrow,
    ops::{Range, RangeBounds},
};

use bit_vector::BitVector;
use coordinate_compression::CoordinateCompression;
use into_half_open_range::IntoHalfOpenRange;
use numeric_traits::{Cast, Inf, Integer, NegInf};

mod bit_vector;
mod internal;

pub type WaveletMatrix<Y, const Y_SMALL: bool = false, const INVERTIVE: bool = false> =
    WaveletMatrix2D<usize, Y, true, Y_SMALL, INVERTIVE>;

impl<Y, const Y_SMALL: bool, const INVERTIVE: bool> WaveletMatrix<Y, Y_SMALL, INVERTIVE>
where
    Y: Integer + Inf + NegInf + Cast<usize>,
    u32: Cast<Y>,
{
    pub fn new<I>(a: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Y>,
    {
        Self::new_with_containers(a, |_| ()).0
    }

    pub fn new_with_containers<Container, I>(
        a: I,
        build_container: impl FnMut(&[usize]) -> Container,
    ) -> (Self, Vec<Container>)
    where
        I: IntoIterator,
        I::Item: Borrow<Y>,
    {
        WaveletMatrix2D::<usize, Y, true, Y_SMALL, INVERTIVE>::new_2d_with_containers(
            a.into_iter().enumerate().map(|(i, y)| (i, *y.borrow())),
            build_container,
        )
    }
}

pub struct WaveletMatrix2D<
    X,
    Y,
    const X_SMALL: bool = false,
    const Y_SMALL: bool = false,
    const INVERTIVE: bool = false,
> where
    X: Integer + Inf + NegInf + Cast<usize>,
    Y: Integer + Inf + NegInf + Cast<usize>,
    u32: Cast<X>,
    u32: Cast<Y>,
{
    n: usize,
    m: usize,
    lg: usize,
    ccx: CoordinateCompression<X, X_SMALL, true>,
    ccy: CoordinateCompression<Y, Y_SMALL, false>,
    pos: Vec<usize>,
    mid: Vec<usize>,
    bv: Vec<BitVector>,
}

impl<X, Y, const X_SMALL: bool, const Y_SMALL: bool, const INVERTIVE: bool>
    WaveletMatrix2D<X, Y, X_SMALL, Y_SMALL, INVERTIVE>
where
    X: Integer + Inf + NegInf + Cast<usize>,
    Y: Integer + Inf + NegInf + Cast<usize>,
    u32: Cast<X>,
    u32: Cast<Y>,
{
    pub fn new_2d<I>(ps: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<(X, Y)>,
    {
        Self::new_2d_with_containers(ps, |_| ()).0
    }

    pub fn new_2d_with_containers<Container, I>(
        ps: I,
        mut build_container: impl FnMut(&[usize]) -> Container,
    ) -> (Self, Vec<Container>)
    where
        I: IntoIterator,
        I::Item: Borrow<(X, Y)>,
    {
        let ps = ps.into_iter().map(|p| *p.borrow()).collect::<Vec<_>>();
        let n = ps.len();

        let (ccx, pos) = CoordinateCompression::<X, X_SMALL, true>::new(ps.iter().map(|&(x, _)| x));
        let (ccy, _) = CoordinateCompression::<Y, Y_SMALL, false>::new(ps.iter().map(|&(_, y)| y));
        let m = ccy.len();
        let lg = m.checked_ceil_log2().unwrap_or(0);

        let mut a = vec![0; n]; // インデックス
        let mut b = vec![0; n]; // エンコードされた値
        for i in 0..n {
            a[pos[i]] = i;
            b[pos[i]] = ccy.encode(ps[i].1);
        }

        let mut mid = vec![0; lg];
        let mut bv = vec![BitVector::new(n); lg];
        let mut containers = Vec::with_capacity(lg + 1);
        containers.push(build_container(&a));
        let mut a0 = vec![0; n];
        let mut a1 = vec![0; n];
        let mut b0 = vec![0; n];
        let mut b1 = vec![0; n];
        for d in (0..lg).rev() {
            let mut p0 = 0;
            let mut p1 = 0;
            for i in 0..n {
                if b[i] >> d & 1 == 0 {
                    a0[p0] = a[i];
                    b0[p0] = b[i];
                    p0 += 1;
                } else {
                    bv[d].set(i);
                    a1[p1] = a[i];
                    b1[p1] = b[i];
                    p1 += 1;
                }
            }
            a[..p0].copy_from_slice(&a0[..p0]);
            a[p0..].copy_from_slice(&a1[..p1]);
            b[..p0].copy_from_slice(&b0[..p0]);
            b[p0..].copy_from_slice(&b1[..p1]);
            mid[d] = p0;
            bv[d].build();
            containers.push(build_container(&a));
        }
        containers.reverse();

        (
            Self {
                n,
                m,
                lg,
                ccx,
                ccy,
                pos,
                mid,
                bv,
            },
            containers,
        )
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn map_index(&self, i: usize) -> usize {
        self.pos[i]
    }

    pub fn count(&self, x_range: impl RangeBounds<X>, y_range: impl RangeBounds<Y>) -> usize {
        self.count_with(x_range, y_range, |_, _, _| {})
    }

    pub fn count_with(
        &self,
        x_range: impl RangeBounds<X>,
        y_range: impl RangeBounds<Y>,
        mut f: impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        let (xl, xr) = x_range.into_half_open_range(X::neg_inf(), X::inf());
        let (yl, yr) = y_range.into_half_open_range(Y::neg_inf(), Y::inf());
        let xl = self.ccx.encode(xl);
        let xr = self.ccx.encode(xr);
        let yl = self.ccy.encode(yl);
        let yr = self.ccy.encode(yr);

        self.count_with_(xl, xr, yl, yr, &mut f)
    }

    pub fn access(&self, i: usize) -> Y {
        self.access_with(i, |_, _| {})
    }

    pub fn access_with(&self, i: usize, mut f: impl FnMut(usize, usize)) -> Y {
        let mut y = 0;
        let mut i = self.map_index(i);
        f(self.lg, i);
        for d in (0..self.lg).rev() {
            if self.bv[d].get(i) {
                y |= 1 << d;
                i = self.bv[d].count_prefix(i, true) + self.mid[d];
            } else {
                i = self.bv[d].count_prefix(i, false);
            }
            f(d, i);
        }
        self.ccy.decode(y)
    }

    pub fn kth_smallest(&self, x_range: impl RangeBounds<X>, mut k: usize) -> Option<Y> {
        let (xl, xr) = x_range.into_half_open_range(X::neg_inf(), X::inf());
        let mut xl = self.ccx.encode(xl);
        let mut xr = self.ccx.encode(xr);
        if k >= xr - xl {
            return None;
        }

        let mut y = 0;
        for d in (0..self.lg).rev() {
            let l0 = self.bv[d].count_prefix(xl, false);
            let r0 = self.bv[d].count_prefix(xr, false);
            let l1 = xl + self.mid[d] - l0;
            let r1 = xr + self.mid[d] - r0;
            if k < r0 - l0 {
                (xl, xr) = (l0, r0);
            } else {
                k -= r0 - l0;
                y |= 1 << d;
                (xl, xr) = (l1, r1);
            }
        }
        Some(self.ccy.decode(y))
    }

    pub fn kth_largest(&self, x_range: impl RangeBounds<X>, k: usize) -> Option<Y> {
        let (xl, xr) = x_range.into_half_open_range(X::neg_inf(), X::inf());
        let xl = self.ccx.encode(xl);
        let xr = self.ccx.encode(xr);
        if k >= xr - xl {
            return None;
        }

        self.kth_smallest(x_range, xr - xl - 1 - k)
    }
}
