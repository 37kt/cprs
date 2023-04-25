use std::ops::{Bound, RangeBounds};

use algebraic::{Group, Monoid};

pub struct FenwickTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    v: Vec<M::S>,
}

impl<M> FenwickTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new(n: usize) -> Self {
        FenwickTree { v: vec![M::e(); n] }
    }

    pub fn add(&mut self, mut k: usize, x: M::S) {
        assert!(k <= self.v.len());
        k += 1;
        while k <= self.v.len() {
            self.v[k - 1] = M::op(&self.v[k - 1], &x);
            k += k & k.wrapping_neg();
        }
    }

    pub fn accum(&self, mut k: usize) -> M::S {
        let mut res = M::e();
        while k > 0 {
            res = M::op(&res, &self.v[k - 1]);
            k &= k - 1;
        }
        res
    }
}

impl<M> FenwickTree<M>
where
    M: Group,
    M::S: Clone,
{
    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> M::S {
        let r = match range.end_bound() {
            Bound::Included(&r) => r + 1,
            Bound::Excluded(&r) => r,
            Bound::Unbounded => self.v.len(),
        };
        let l = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l + 1,
            Bound::Unbounded => return self.accum(r),
        };
        M::op(&M::inv(&self.accum(l)), &self.accum(r))
    }
}
