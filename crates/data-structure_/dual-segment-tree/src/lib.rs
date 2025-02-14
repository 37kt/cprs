use std::ops::{Bound, RangeBounds};

use algebraic::Monoid;

/// 双対セグメント木
///
/// 動的な作用素の列を管理するデータ構造。
///
/// # 計算量
///
/// - 構築: O(n)
/// - apply_range: O(log n)
/// - apply: O(log n)
/// - get: O(log n)
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
    /// 単位元で初期化した双対セグメント木を構築する。
    pub fn new(n: usize) -> Self {
        Self {
            n,
            v: vec![M::e(); n * 2],
        }
    }

    /// a\[range\] に作用素 f を適用する。
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
        self.propagate(r);
        while l < r {
            if l & 1 != 0 {
                self.v[l] = M::op(&f, &self.v[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.v[r] = M::op(&f, &self.v[r]);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    /// a\[k\] に作用素 f を適用する。
    pub fn apply(&mut self, k: usize, f: M::S) {
        assert!(k < self.n);
        self.apply_range(k..=k, f);
    }

    /// a\[k\] を取得する。
    pub fn get(&self, mut k: usize) -> M::S {
        assert!(k < self.n);
        k += self.n;
        let mut res = self.v[k].clone();
        while k > 1 {
            k >>= 1;
            res = M::op(&self.v[k], &res);
        }
        res
    }

    fn push(&mut self, i: usize) {
        self.v[i * 2] = M::op(&self.v[i], &self.v[i * 2]);
        self.v[i * 2 + 1] = M::op(&self.v[i], &self.v[i * 2 + 1]);
        self.v[i] = M::e();
    }

    fn propagate(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        let crz = i.trailing_zeros() as usize;
        for h in (crz + 1..64 - i.leading_zeros() as usize).rev() {
            self.push(i >> h);
        }
    }
}

impl<M> From<Vec<M::S>> for DualSegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    /// Vec から双対セグメント木を構築する。
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
