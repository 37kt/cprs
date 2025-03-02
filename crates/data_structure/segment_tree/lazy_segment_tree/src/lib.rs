use std::ops::{Deref, DerefMut, RangeBounds};

use algebraic_traits::{Act, Algebraic, Magma, Monoid, Unital};
use into_half_open_range::IntoHalfOpenRange;
use numeric_traits::Integer;

pub struct LazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    n: usize,
    sz: usize,
    lg: usize,
    v: Vec<<A::Operand as Algebraic>::Value>,
    lz: Vec<<A::Operator as Algebraic>::Value>,
}

impl<A> FromIterator<<A::Operand as Algebraic>::Value> for LazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    fn from_iter<T: IntoIterator<Item = <A::Operand as Algebraic>::Value>>(iter: T) -> Self {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let lg = n.checked_ceil_log2().unwrap_or(0);
        let sz = 1 << lg;
        let v = (0..sz)
            .map(|_| A::Operand::unit())
            .chain(a)
            .chain((0..sz - n).map(|_| A::Operand::unit()))
            .collect();
        let lz = vec![A::Operator::unit(); sz];
        let mut seg = Self { n, sz, lg, v, lz };
        for i in (1..sz).rev() {
            seg.update(i);
        }
        seg
    }
}

impl<A> LazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    pub fn new(n: usize) -> Self {
        let lg = n.checked_ceil_log2().unwrap_or(0);
        let sz = 1 << lg;
        let v = vec![A::Operand::unit(); sz * 2];
        let lz = vec![A::Operator::unit(); sz];
        Self { n, sz, lg, v, lz }
    }

    pub fn from_fn(n: usize, f: impl FnMut(usize) -> <A::Operand as Algebraic>::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn set(&mut self, mut i: usize, x: <A::Operand as Algebraic>::Value) {
        assert!(i < self.n);
        i += self.sz;
        for h in (1..=self.lg).rev() {
            self.push(i >> h);
        }
        self.v[i] = x;
        for h in 1..=self.lg {
            self.update(i >> h);
        }
    }

    pub fn add(&mut self, mut i: usize, x: <A::Operand as Algebraic>::Value) {
        assert!(i < self.n);
        i += self.sz;
        for h in (1..=self.lg).rev() {
            self.push(i >> h);
        }
        self.v[i] = A::Operand::op(&self.v[i], &x);
        for h in 1..=self.lg {
            self.update(i >> h);
        }
    }

    pub fn get(&mut self, mut i: usize) -> <A::Operand as Algebraic>::Value {
        assert!(i < self.n);
        i += self.sz;
        for h in (1..=self.lg).rev() {
            self.push(i >> h);
        }
        self.v[i].clone()
    }

    pub fn get_mut(&mut self, mut i: usize) -> GetMut<A> {
        assert!(i < self.n);
        i += self.sz;
        for h in (1..=self.lg).rev() {
            self.push(i >> h);
        }
        GetMut { seg: self, i }
    }

    pub fn as_slice(&mut self) -> &[<A::Operand as Algebraic>::Value] {
        for i in 1..self.sz {
            self.push(i);
        }
        &self.v[self.sz..self.sz + self.n]
    }

    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> <A::Operand as Algebraic>::Value {
        let (mut l, mut r) = range.into_half_open_range(0, self.n);
        if l == r {
            return A::Operand::unit();
        }
        l += self.sz;
        r += self.sz;
        for h in (1..=self.lg).rev() {
            if l >> h << h != l {
                self.push(l >> h);
            }
            if r >> h << h != r {
                self.push(r - 1 >> h);
            }
        }
        let mut sl = A::Operand::unit();
        let mut sr = A::Operand::unit();
        while l < r {
            if l & 1 != 0 {
                sl = A::Operand::op(&sl, &self.v[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                sr = A::Operand::op(&self.v[r], &sr);
            }
            l >>= 1;
            r >>= 1;
        }
        A::Operand::op(&sl, &sr)
    }

    pub fn fold_all(&self) -> <A::Operand as Algebraic>::Value {
        self.v[1].clone()
    }

    pub fn apply_range(
        &mut self,
        range: impl RangeBounds<usize>,
        f: <A::Operator as Algebraic>::Value,
    ) {
        let (mut l, mut r) = range.into_half_open_range(0, self.n);
        if l == r {
            return;
        }
        l += self.sz;
        r += self.sz;
        for h in (1..=self.lg).rev() {
            if l >> h << h != l {
                self.push(l >> h);
            }
            if r >> h << h != r {
                self.push(r - 1 >> h);
            }
        }
        let l2 = l;
        let r2 = r;
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
        l = l2;
        r = r2;
        for h in 1..=self.lg {
            if l >> h << h != l {
                self.update(l >> h);
            }
            if r >> h << h != r {
                self.update(r - 1 >> h);
            }
        }
    }

    pub fn apply(&mut self, i: usize, f: <A::Operator as Algebraic>::Value) {
        self.apply_range(i..i + 1, f);
    }

    pub fn max_right(
        &mut self,
        l: usize,
        mut pred: impl FnMut(&<A::Operand as Algebraic>::Value) -> bool,
    ) -> usize {
        assert!(l <= self.n);
        assert!(pred(&A::Operand::unit()));
        if l == self.n {
            return self.n;
        }
        let mut r = l + self.sz;
        for h in (1..=self.lg).rev() {
            self.push(r >> h);
        }
        let mut s = A::Operand::unit();
        loop {
            r >>= r.lsb_index();
            let t = A::Operand::op(&s, &self.v[r]);
            if !pred(&t) {
                while r < self.sz {
                    self.push(r);
                    r <<= 1;
                    let t = A::Operand::op(&s, &self.v[r]);
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

    pub fn min_left(
        &mut self,
        r: usize,
        mut pred: impl FnMut(&<A::Operand as Algebraic>::Value) -> bool,
    ) -> usize {
        assert!(r <= self.n);
        assert!(pred(&A::Operand::unit()));
        if r == 0 {
            return 0;
        }
        let mut l = r + self.sz;
        for h in (1..=self.lg).rev() {
            self.push(l - 1 >> h);
        }
        let mut s = A::Operand::unit();
        loop {
            l -= 1;
            l >>= (!l).lsb_index();
            l = l.max(1);
            let t = A::Operand::op(&self.v[l], &s);
            if !pred(&t) {
                while l < self.sz {
                    self.push(l);
                    l = l << 1 | 1;
                    let t = A::Operand::op(&self.v[l], &s);
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
        self.v[i] = A::Operand::op(&self.v[i << 1], &self.v[i << 1 | 1]);
    }

    fn apply_at(&mut self, i: usize, f: &<A::Operator as Algebraic>::Value) {
        self.v[i] = A::act(&self.v[i], f);
        if i < self.sz {
            self.lz[i] = A::Operator::op(&self.lz[i], f);
        }
    }

    fn push(&mut self, i: usize) {
        if self.lz[i] == A::Operator::unit() {
            return;
        }
        let f = std::mem::replace(&mut self.lz[i], A::Operator::unit());
        self.apply_at(i << 1, &f);
        self.apply_at(i << 1 | 1, &f);
    }
}

pub struct GetMut<'a, A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    seg: &'a mut LazySegmentTree<A>,
    i: usize,
}

impl<'a, A> Deref for GetMut<'a, A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    type Target = <A::Operand as Algebraic>::Value;

    fn deref(&self) -> &Self::Target {
        &self.seg.v[self.i]
    }
}

impl<'a, A> DerefMut for GetMut<'a, A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seg.v[self.i]
    }
}

impl<'a, A> Drop for GetMut<'a, A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    fn drop(&mut self) {
        for h in 1..=self.seg.lg {
            self.seg.update(self.i >> h);
        }
    }
}

impl<A> Clone for LazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone + Eq,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            sz: self.sz,
            lg: self.lg,
            v: self.v.clone(),
            lz: self.lz.clone(),
        }
    }
}
