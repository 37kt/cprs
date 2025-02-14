use std::ops::{Bound, RangeBounds};

use algebraic::{Group, Monoid};

/// Fenwick Tree
///
/// prefix sum を管理するデータ構造。
/// 群に対しては区間和クエリも提供する。
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
    /// サイズ n の Fenwick Tree を構築する。  
    /// 単位元で初期化する。
    ///
    /// # 計算量
    ///
    /// O(n)
    pub fn new(n: usize) -> Self {
        FenwickTree { v: vec![M::e(); n] }
    }

    /// a[k] を op(a[k], x) に更新する。
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn add(&mut self, mut k: usize, x: M::S) {
        assert!(k <= self.v.len());
        k += 1;
        while k <= self.v.len() {
            self.v[k - 1] = M::op(&self.v[k - 1], &x);
            k += k & k.wrapping_neg();
        }
    }

    /// a[..k] の総積を取得する。
    ///
    /// # 計算量
    ///
    /// O(log n)
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
    /// a[range] の総積を取得する。
    ///
    /// # 計算量
    ///
    /// O(log n)
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
