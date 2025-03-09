use std::ops::{Range, RangeBounds};

use bit_vector::BitVector;
use coordinate_compression::CoordinateCompression;
use into_half_open_range::IntoHalfOpenRange;
use numeric_traits::{Cast, Inf, Integer, NegInf};

mod bit_vector;

pub struct WaveletMatrixImpl<
    X,
    Y,
    const X_SMALL: bool = false,
    const Y_SMALL: bool = false,
    const INVERTIVE: bool = false,
> where
    X: Integer + Inf + NegInf + Cast<usize>,
    Y: Integer + Inf + NegInf + Cast<usize>,
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
    WaveletMatrixImpl<X, Y, X_SMALL, Y_SMALL, INVERTIVE>
where
    X: Integer + Inf + NegInf + Cast<usize>,
    Y: Integer + Inf + NegInf + Cast<usize>,
{
    pub fn new<Container>(
        ps: impl IntoIterator<Item = (X, Y)>,
        mut build_container: impl FnMut(&[usize]) -> Container,
    ) -> (Self, Vec<Container>) {
        let ps = ps.into_iter().collect::<Vec<_>>();
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

    pub fn count_with(
        &self,
        x_range: impl RangeBounds<X>,
        y_range: impl RangeBounds<Y>,
        f: impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        let (xl, xr) = x_range.into_half_open_range(X::neg_inf(), X::inf());
        let (yl, yr) = y_range.into_half_open_range(Y::neg_inf(), Y::inf());
        let xl = self.ccx.encode(xl);
        let xr = self.ccx.encode(xr);
        let yl = self.ccy.encode(yl);
        let yr = self.ccy.encode(yr);

        self.count_with_(xl, xr, yl, yr, f)
    }

    fn count_with_(
        &self,
        xl: usize,
        xr: usize,
        yl: usize,
        yr: usize,
        mut f: impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        if INVERTIVE {
            self.count_prefix_with_(xl, xr, yr, &mut f)
                - self.count_prefix_with_(xl, xr, yl, &mut |d, x, inv| f(d, x, !inv))
        } else {
            self.dfs_count_with_(self.lg, xl, xr, yl, yr, 0, 1 << self.lg, &mut f)
        }
    }

    fn dfs_count_with_(
        &self,
        d: usize,
        xl: usize,
        xr: usize,
        yl: usize,
        yr: usize,
        a: usize,
        b: usize,
        f: &mut impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        assert_eq!(1 << d, b - a);
        if yr <= a || b <= yl {
            return 0;
        } else if yl <= a && b <= yr {
            f(d, xl..xr, false);
            return xr - xl;
        }
        let d = d - 1;
        let c = (a + b) >> 1;
        let l0 = self.bv[d].count_prefix(xl, 0);
        let r0 = self.bv[d].count_prefix(xr, 0);
        let l1 = xl + self.mid[d] - l0;
        let r1 = xr + self.mid[d] - r0;
        self.dfs_count_with_(d, l0, r0, yl, yr, a, c, f)
            + self.dfs_count_with_(d, l1, r1, yl, yr, c, b, f)
    }

    fn count_prefix_with_(
        &self,
        mut xl: usize,
        mut xr: usize,
        y: usize,
        f: &mut impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        if xl == xr || y == 0 {
            return 0;
        } else if y == self.m {
            f(self.lg, xl..xr, false);
            return xr - xl;
        }

        let mut cnt = 0;
        for d in (0..self.lg).rev() {
            let l0 = self.bv[d].count_prefix(xl, 0);
            let r0 = self.bv[d].count_prefix(xr, 0);
            let l1 = xl + self.mid[d] - l0;
            let r1 = xr + self.mid[d] - r0;
            if y >> d & 1 == 0 {
                xl = l0;
                xr = r0;
            } else {
                f(d, l0..r0, false);
                cnt += r0 - l0;
                xl = l1;
                xr = r1;
            }
        }
        cnt
    }
}
