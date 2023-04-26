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

    pub fn get(&self, k: usize) -> M::S {
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

    pub fn max_right<P>(&self, mut l: usize, pred: P) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(pred(&M::e()));
        if pred(&self.prod(l..)) {
            return self.n;
        }
        l += self.n;
        let mut s = M::e();
        loop {
            while l & 1 == 0 && self.is_good_node(l >> 1) {
                l >>= 1;
            }
            if !pred(&M::op(&s, &self.v[l])) {
                while l < self.n {
                    l <<= 1;
                    let t = M::op(&s, &self.v[l]);
                    if pred(&t) {
                        s = t;
                        l += 1;
                    }
                }
                return l - self.n;
            }
            s = M::op(&s, &self.v[l]);
            l += 1;
        }
    }

    pub fn min_left<P>(&self, mut r: usize, pred: P) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(pred(&M::e()));
        if pred(&self.prod(..r)) {
            return 0;
        }
        r += self.n;
        let mut s = M::e();
        loop {
            r -= 1;
            while !self.is_good_node(r) {
                r = r * 2 + 1;
            }
            while r & 1 != 0 && self.is_good_node(r >> 1) {
                r >>= 1;
            }
            if !pred(&M::op(&self.v[r], &s)) {
                while r < self.n {
                    r = r * 2 + 1;
                    let t = M::op(&self.v[r], &s);
                    if pred(&t) {
                        s = t;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }
            s = M::op(&self.v[r], &s);
        }
    }

    #[inline]
    fn is_good_node(&self, k: usize) -> bool {
        if k >= self.n {
            true
        } else {
            let d = k.leading_zeros() - self.n.leading_zeros();
            self.n >> d != k || self.n >> d << d == self.n
        }
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
