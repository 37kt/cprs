use std::ops::{Bound, RangeBounds};

use algebraic::Monoid;

#[derive(Clone)]
pub struct DualSegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    n: usize,
    v: Vec<M::S>,
}

impl<M> DualSegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            v: vec![M::e(); n * 2],
        }
    }

    pub fn apply_range(&mut self, range: impl RangeBounds<usize>, f: M::S) {
        let mut l = match range.start_bound() {
            Bound::Excluded(&l) => l + 1,
            Bound::Included(&l) => l,
            Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            Bound::Excluded(&r) => r,
            Bound::Included(&r) => r + 1,
            Bound::Unbounded => self.n,
        };
        assert!(l <= r);
        assert!(r <= self.n);
        l += self.n;
        r += self.n;
        self.propagate(l);
        self.propagate(r - 1);
        while l < r {
            if l & 1 != 0 {
                self.v[l] = M::op(&self.v[l], &f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.v[r] = M::op(&self.v[r], &f);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    pub fn apply(&mut self, k: usize, f: M::S) {
        assert!(k < self.n);
        self.apply_range(k..=k, f);
    }

    pub fn get(&self, mut k: usize) -> M::S {
        assert!(k < self.n);
        k += self.n;
        let mut res = self.v[k].clone();
        while k > 1 {
            k >>= 1;
            res = M::op(&res, &self.v[k]);
        }
        res
    }

    fn push(&mut self, i: usize) {
        if i < self.n {
            self.v[i * 2] = M::op(&self.v[i * 2], &self.v[i]);
            self.v[i * 2 + 1] = M::op(&self.v[i * 2 + 1], &self.v[i]);
            self.v[i] = M::e();
        }
    }

    fn propagate(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        let crz = i.trailing_zeros() as usize;
        for h in (crz + 1..63 - i.leading_zeros() as usize).rev() {
            self.push(i >> h);
        }
    }
}
