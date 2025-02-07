use std::ops::{Bound, RangeBounds};

use algebraic::{Act, Monoid};

/// 遅延セグメント木。
/// 区間作用、区間積取得ができる。
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
    /// Vec で初期化する。
    ///
    /// # 引数
    ///
    /// - `a`: 初期値
    ///
    /// # 計算量
    ///
    /// O(N)
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
    /// a\[k\] を x に更新する。
    ///
    /// # 計算量
    ///
    /// O(log N)
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

    /// a\[k\] を取得する。
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn get(&mut self, mut k: usize) -> M::S {
        assert!(k < self.n);
        k += self.n;
        let h = 63 - k.leading_zeros() as usize;
        for i in (1..=h).rev() {
            self.push(k >> i);
        }
        self.v[k].clone()
    }

    /// a\[range\] の総積を取得する。
    ///
    /// # 計算量
    ///
    /// O(log N)
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
            if l >> i << i != l {
                self.push(l >> i);
            }
            if r >> i << i != r {
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

    /// a\[k\] に f を左から作用させる。
    ///
    /// # 計算量
    ///
    /// O(log N)
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

    /// a\[range\] に f を左から作用させる。
    ///
    /// # 計算量
    ///
    /// O(log N)
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
            if l >> i << i != l {
                self.push(l >> i);
            }
            if r >> i << i != r {
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
            if l >> i << i != l {
                self.update(l >> i);
            }
            if r >> i << i != r {
                self.update(r - 1 >> i);
            }
        }
    }

    /// l を左端とする区間のうち、条件を満たす最大の区間の右端を取得する。
    ///
    /// # 引数
    ///
    /// - `l`: 左端
    /// - `pred`: a\[range\] が条件を満たすかを判定する関数
    ///
    /// # 戻り値
    ///
    /// - 条件を満たす最大の区間の右端
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn max_right<P>(&mut self, mut l: usize, pred: P) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(pred(&M::e()));
        if pred(&self.prod(l..)) {
            return self.n;
        }
        l += self.n;
        let h = 63 - l.leading_zeros() as usize;
        for i in (1..=h).rev() {
            self.push(l >> i);
        }
        let mut s = M::e();
        loop {
            while l & 1 == 0 && self.is_good_node(l >> 1) {
                l >>= 1;
            }
            if !pred(&M::op(&s, &self.v[l])) {
                while l < self.n {
                    self.push(l);
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

    /// r を右端とする区間のうち、条件を満たす最小の区間の左端を取得する。
    ///
    /// # 引数
    ///
    /// - `r`: 右端
    /// - `pred`: a\[range\] が条件を満たすかを判定する関数
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn min_left<P>(&mut self, mut r: usize, pred: P) -> usize
    where
        P: Fn(&M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(pred(&M::e()));
        if pred(&self.prod(..r)) {
            return 0;
        }
        r += self.n;
        let h = 63 - (r - 1).leading_zeros() as usize;
        for i in (1..=h).rev() {
            self.push(r - 1 >> i);
        }
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
                    self.push(r);
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

    /// 子ノードの値を親ノードに反映させる。
    fn update(&mut self, k: usize) {
        self.v[k] = M::op(&self.v[k * 2], &self.v[k * 2 + 1]);
    }

    /// ノードに作用させる。子への作用は遅延させる。
    fn all_apply(&mut self, k: usize, f: F::S) {
        self.v[k] = F::act(&f, &self.v[k]);
        if k < self.n {
            self.lz[k] = F::op(&f, &self.lz[k]);
        }
    }

    /// 遅延させていた作用を子ノードに反映させる。
    fn push(&mut self, k: usize) {
        self.all_apply(k * 2, self.lz[k].clone());
        self.all_apply(k * 2 + 1, self.lz[k].clone());
        self.lz[k] = F::e();
    }

    /// セグ木のサイズを 2 冪にしていない都合上、無効なノードもある。  
    /// 有効なノードかどうかを判定する。
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
