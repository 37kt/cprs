use std::ops::RangeBounds;

use algebraic_traits::Monoid;
use into_half_open_range::IntoHalfOpenRange;
use numeric_traits::Integer;

pub struct DualSegmentTree<M>
where
    M: Monoid,
    M::Value: Clone + Eq,
{
    n: usize,
    sz: usize,
    lg: usize,
    lz: Vec<M::Value>,
}

impl<M> FromIterator<M::Value> for DualSegmentTree<M>
where
    M: Monoid,
    M::Value: Clone + Eq,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let lg = n.checked_ceil_log2().unwrap_or(0);
        let sz = 1 << lg;
        let lz = (0..sz)
            .map(|_| M::unit())
            .chain(a)
            .chain((0..sz - n).map(|_| M::unit()))
            .collect();
        Self { n, sz, lg, lz }
    }
}

impl<M> DualSegmentTree<M>
where
    M: Monoid,
    M::Value: Clone + Eq,
{
    pub fn new(n: usize) -> Self {
        let lg = n.checked_ceil_log2().unwrap_or(0);
        let sz = 1 << lg;
        let lz = vec![M::unit(); sz * 2];
        Self { n, sz, lg, lz }
    }

    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn set(&mut self, mut i: usize, x: M::Value) {
        assert!(i < self.n);
        i += self.sz;
        for h in (1..=self.lg).rev() {
            self.push(i >> h);
        }
        self.lz[i] = x;
    }

    pub fn get(&mut self, mut i: usize) -> M::Value {
        assert!(i < self.n);
        i += self.sz;
        for h in (1..=self.lg).rev() {
            self.push(i >> h);
        }
        self.lz[i].clone()
    }

    pub fn apply_range(&mut self, range: impl RangeBounds<usize>, f: M::Value) {
        let (mut l, mut r) = range.into_half_open_range(0, self.n);
        if l == r {
            return;
        }
        l += self.sz;
        r += self.sz;

        // if !COMMUTATIVE {
        for h in (1..=self.lg).rev() {
            if l >> h << h != l {
                self.push(l >> h);
            }
            if r >> h << h != r {
                self.push((r - 1) >> h);
            }
        }
        // }

        while l < r {
            if l & 1 != 0 {
                self.apply_at(l, &f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.apply_at(r, &f);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    fn push(&mut self, i: usize) {
        if self.lz[i] == M::unit() {
            return;
        }
        let f = std::mem::replace(&mut self.lz[i], M::unit());
        self.lz[i << 1] = M::op(&self.lz[i << 1], &f);
        self.lz[i << 1 | 1] = M::op(&self.lz[i << 1 | 1], &f);
    }

    fn apply_at(&mut self, i: usize, f: &M::Value) {
        self.lz[i] = M::op(&self.lz[i], f);
    }
}
