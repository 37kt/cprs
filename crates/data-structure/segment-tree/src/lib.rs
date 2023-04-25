use std::ops::{Bound, RangeBounds};

use algebraic::Monoid;

#[derive(Clone)]

pub struct SegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    n: usize,
    v: Vec<M::S>,
}

impl<M> SegmentTree<M>
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

    pub fn set(&mut self, mut k: usize, x: M::S) {
        k += self.n;
        self.v[k] = x;
        while k > 1 {
            k >>= 1;
            self.v[k] = M::op(&self.v[k * 2], &self.v[k * 2 + 1]);
        }
    }

    pub fn get(&mut self, k: usize) -> M::S {
        assert!(k < self.n);
        self.v[k + self.n].clone()
    }

    pub fn prod<R>(&self, range: R) -> M::S
    where
        R: RangeBounds<usize>,
    {
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
        let mut sl = M::e();
        let mut sr = M::e();
        while l < r {
            if l & 1 != 0 {
                sl = M::op(&sl, &self.v[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                sr = M::op(&self.v[r], &sr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&sl, &sr)
    }
}

impl<M> From<Vec<M::S>> for SegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    fn from(mut a: Vec<M::S>) -> Self {
        let n = a.len();
        let mut v = vec![M::e(); n];
        v.append(&mut a);
        for i in (1..n).rev() {
            v[i] = M::op(&v[i * 2], &v[i * 2 + 1]);
        }
        Self { n, v }
    }
}
