use std::ops::{Deref, DerefMut, RangeBounds};

use algebraic_traits::Monoid;
use into_half_open_range::IntoHalfOpenRange;
use numeric_traits::Integer;

pub struct SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    n: usize,
    sz: usize,
    v: Vec<M::Value>,
}

impl<M> FromIterator<M::Value> for SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let sz = n.checked_ceil_pow2().unwrap_or(1);
        let v: Vec<_> = (0..sz)
            .map(|_| M::unit())
            .chain(a)
            .chain((0..sz - n).map(|_| M::unit()))
            .collect();
        let mut seg = Self { n, sz, v };
        for i in (1..sz).rev() {
            seg.update(i);
        }
        seg
    }
}

impl<M> SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    pub fn new(n: usize) -> Self {
        let sz = n.checked_ceil_pow2().unwrap_or(1);
        let v = vec![M::unit(); sz * 2];
        Self { n, sz, v }
    }

    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn set(&mut self, i: usize, x: M::Value) {
        assert!(i < self.n);
        let mut i = i + self.sz;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.update(i);
        }
    }

    pub fn add(&mut self, i: usize, x: M::Value) {
        assert!(i < self.n);
        let mut i = i + self.sz;
        self.v[i] = M::op(&self.v[i], &x);
        while i > 1 {
            i >>= 1;
            self.update(i);
        }
    }

    pub fn get(&self, i: usize) -> M::Value {
        assert!(i < self.n);
        self.v[i + self.sz].clone()
    }

    pub fn get_mut(&mut self, i: usize) -> GetMut<M> {
        assert!(i < self.n);
        GetMut {
            i: i + self.sz,
            seg: self,
        }
    }

    pub fn as_slice(&self) -> &[M::Value] {
        &self.v[self.sz..self.sz + self.n]
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {
        let (mut l, mut r) = range.into_half_open_range(0, self.n);
        l += self.sz;
        r += self.sz;
        let mut sl = M::unit();
        let mut sr = M::unit();
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

    pub fn fold_all(&self) -> M::Value {
        self.v[1].clone()
    }

    pub fn max_right(&self, l: usize, mut pred: impl FnMut(&M::Value) -> bool) -> usize {
        assert!(l <= self.n);
        assert!(pred(&M::unit()));
        if l == self.n {
            return self.n;
        }
        let mut r = l + self.sz;
        let mut s = M::unit();
        loop {
            r >>= r.lsb_index();
            let t = M::op(&s, &self.v[r]);
            if !pred(&t) {
                while r < self.sz {
                    r <<= 1;
                    let t = M::op(&s, &self.v[r]);
                    if pred(&t) {
                        s = t;
                        r += 1;
                    }
                }
                return r - self.sz;
            }
            s = t;
            r += 1;
            if r == r.lsb() {
                break;
            }
        }
        self.n
    }

    pub fn min_left(&self, r: usize, mut pred: impl FnMut(&M::Value) -> bool) -> usize {
        assert!(r <= self.n);
        assert!(pred(&M::unit()));
        if r == 0 {
            return 0;
        }
        let mut l = r + self.sz;
        let mut s = M::unit();
        loop {
            l -= 1;
            l >>= (!l).lsb_index();
            l = l.max(1);
            let t = M::op(&self.v[l], &s);
            if !pred(&t) {
                while l < self.sz {
                    l = l << 1 | 1;
                    let t = M::op(&self.v[l], &s);
                    if pred(&t) {
                        s = t;
                        l -= 1;
                    }
                }
                return l + 1 - self.sz;
            }
            s = t;
            if l == l.lsb() {
                break;
            }
        }
        0
    }

    fn update(&mut self, i: usize) {
        self.v[i] = M::op(&self.v[i << 1], &self.v[i << 1 | 1]);
    }
}

pub struct GetMut<'a, M>
where
    M: Monoid,
    M::Value: Clone,
{
    seg: &'a mut SegmentTree<M>,
    i: usize,
}

impl<'a, M> Deref for GetMut<'a, M>
where
    M: Monoid,
    M::Value: Clone,
{
    type Target = M::Value;

    fn deref(&self) -> &Self::Target {
        &self.seg.v[self.i]
    }
}

impl<'a, M> DerefMut for GetMut<'a, M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seg.v[self.i]
    }
}

impl<'a, M> Drop for GetMut<'a, M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn drop(&mut self) {
        let mut i = self.i;
        while i > 1 {
            i >>= 1;
            self.seg.update(i);
        }
    }
}

impl<M> Clone for SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            sz: self.sz,
            v: self.v.clone(),
        }
    }
}
