use std::ops::{Bound, RangeBounds};

use algebraic::{Act, Monoid};

#[derive(Clone)]
pub struct LazySegmentTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Act<X = M::S> + Monoid,
    F::S: Clone,
{
    n: usize,
    v: Vec<M::S>,
    lz: Vec<F::S>,
}

impl<M, F> From<Vec<M::S>> for LazySegmentTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Act<X = M::S> + Monoid,
    F::S: Clone,
{
    fn from(mut a: Vec<M::S>) -> Self {
        let n = a.len();
        let mut v = vec![M::e(); n];
        v.append(&mut a);
        let lz = vec![F::e(); n];
        let mut seg = LazySegmentTree { n, v, lz };
        for k in (1..n).rev() {
            seg.update(k);
        }
        seg
    }
}

impl<M, F> LazySegmentTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Act<X = M::S> + Monoid,
    F::S: Clone,
{
    pub fn set(&mut self, k: usize, x: M::S) {
        assert!(k < self.n);
        let k = k + self.n;
        let h = 63 - k.leading_zeros() as usize;
        for i in (1..=h).rev() {
            self.push(k >> i);
        }
        self.v[k] = x;
        for i in 1..=h {
            self.update(k >> i);
        }
    }

    pub fn get(&mut self, mut k: usize) -> M::S {
        assert!(k < self.n);
        k += self.n;
        let h = 63 - k.leading_zeros() as usize;
        for i in (1..=h).rev() {
            self.push(k >> i);
        }
        self.v[k].clone()
    }

    pub fn prod<R>(&mut self, range: R) -> M::S
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
        if l == r {
            return M::e();
        }

        l += self.n;
        r += self.n;
        let h = 63 - self.n.leading_zeros() as usize;
        for i in (1..=h).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if (r >> i) << i != r {
                self.push(r >> i);
            }
        }

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

    pub fn apply(&mut self, k: usize, f: F::S) {
        assert!(k < self.n);
        let k = k + self.n;
        let h = 63 - k.leading_zeros() as usize;
        for i in (1..=h).rev() {
            self.push(k >> i);
        }
        self.v[k] = F::act(&f, &self.v[k]);
        for i in 1..=h {
            self.update(k >> i);
        }
    }

    pub fn apply_range<R>(&mut self, range: R, f: F::S)
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
        let h = 63 - self.n.leading_zeros() as usize;
        for i in (1..=h).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if (r >> i) << i != r {
                self.push(r - 1 >> i);
            }
        }

        let l2 = l;
        let r2 = r;
        while l < r {
            if l & 1 != 0 {
                self.all_apply(l, f.clone());
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.all_apply(r, f.clone());
            }
            l >>= 1;
            r >>= 1;
        }
        l = l2;
        r = r2;

        for i in 1..=h {
            if (l >> i) << i != l {
                self.update(l >> i);
            }
            if (r >> i) << i != r {
                self.update(r - 1 >> i);
            }
        }
    }

    fn update(&mut self, k: usize) {
        self.v[k] = M::op(&self.v[k * 2], &self.v[k * 2 + 1]);
    }

    fn all_apply(&mut self, k: usize, f: F::S) {
        self.v[k] = F::act(&f, &self.v[k]);
        if k < self.n {
            self.lz[k] = F::op(&f, &self.lz[k]);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(k * 2, self.lz[k].clone());
        self.all_apply(k * 2 + 1, self.lz[k].clone());
        self.lz[k] = F::e();
    }
}
