// TODO: 二分探索

use std::ops::{Deref, DerefMut, RangeBounds};

use algebraic_traits::{AbelianGroup, CommutativeMonoid};
use into_half_open_range::IntoHalfOpenRange;
use numeric_traits::Integer;

pub struct FenwickTree<M>
where
    M: CommutativeMonoid,
    M::Value: Clone,
{
    n: usize,
    v: Vec<M::Value>,
}

impl<M> FromIterator<M::Value> for FenwickTree<M>
where
    M: CommutativeMonoid,
    M::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        let mut v = std::iter::once(M::unit()).chain(iter).collect::<Vec<_>>();
        let n = v.len() - 1;
        for i in 1..=n {
            let j = i + i.lsb();
            if j <= n {
                v[j] = M::op(&v[j], &v[i]);
            }
        }
        Self { n, v }
    }
}

impl<M> FenwickTree<M>
where
    M: CommutativeMonoid,
    M::Value: Clone,
{
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn new(n: usize) -> Self {
        let v = (0..n + 1).map(|_| M::unit()).collect();
        Self { n, v }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn add(&mut self, mut i: usize, x: M::Value) {
        assert!(i < self.n);
        i += 1;
        while i <= self.n {
            self.v[i] = M::op(&self.v[i], &x);
            i += i.lsb();
        }
    }

    pub fn fold_prefix(&self, mut i: usize) -> M::Value {
        let mut s = M::unit();
        while i > 0 {
            s = M::op(&s, &self.v[i]);
            i -= i.lsb();
        }
        s
    }
}

impl<G> FenwickTree<G>
where
    G: AbelianGroup,
    G::Value: Clone,
{
    pub fn fold(&self, range: impl RangeBounds<usize>) -> G::Value {
        let (mut l, mut r) = range.into_half_open_range(0, self.n);
        let mut s = G::unit();
        while r > l {
            s = G::op(&s, &self.v[r]);
            r -= r.lsb();
        }
        while l > r {
            s = G::op(&s, &G::inv(&self.v[l]));
            l -= l.lsb();
        }
        s
    }

    pub fn get(&self, i: usize) -> G::Value {
        self.fold(i..i + 1)
    }

    pub fn set(&mut self, i: usize, x: G::Value) {
        self.add(i, G::op(&x, &G::inv(&self.get(i))));
    }

    pub fn get_mut(&mut self, i: usize) -> GetMut<G> {
        GetMut {
            x: self.get(i),
            ft: self,
            i,
        }
    }
}

pub struct GetMut<'a, G>
where
    G: AbelianGroup,
    G::Value: Clone,
{
    ft: &'a mut FenwickTree<G>,
    i: usize,
    x: G::Value,
}

impl<'a, G> Deref for GetMut<'a, G>
where
    G: AbelianGroup,
    G::Value: Clone,
{
    type Target = G::Value;

    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

impl<'a, G> DerefMut for GetMut<'a, G>
where
    G: AbelianGroup,
    G::Value: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.x
    }
}

impl<'a, G> Drop for GetMut<'a, G>
where
    G: AbelianGroup,
    G::Value: Clone,
{
    fn drop(&mut self) {
        let Self { ft, i, x } = self;
        ft.set(*i, x.clone());
    }
}
