use std::ops::{Bound, RangeBounds};

use algebraic::Monoid;

/// 静的モノイド列の区間クエリを処理する
pub struct DisjointSparseTable<M>
where
    M: Monoid,
    M::S: Clone,
{
    t: Vec<Vec<M::S>>,
}

impl<M> DisjointSparseTable<M>
where
    M: Monoid,
    M::S: Clone,
{
    /// Disjoint Sparse Table を構築する
    pub fn new(a: &[M::S]) -> Self {
        let n = a.len() + 2;
        let h = 64 - (n - 1).leading_zeros() as usize;
        let mut t = vec![vec![M::e(); n]; h];
        for k in 1..h {
            let w = 1 << k;
            for i in (w..n).step_by(w * 2) {
                for j in (i + 1 - w..i).rev() {
                    t[k][j - 1] = M::op(&a[j - 1], &t[k][j]);
                }
                let m = (i + w - 1).min(n - 1);
                for j in i..m {
                    t[k][j + 1] = M::op(&t[k][j], &a[j - 1]);
                }
            }
        }
        Self { t }
    }

    /// a の長さを取得する
    pub fn len(&self) -> usize {
        self.t[0].len() - 2
    }

    /// a\[k\] を取得する
    pub fn get(&self, k: usize) -> M::S {
        assert!(k < self.len());
        self.prod(k..k + 1)
    }

    /// a\[range\] の総積を取得する
    pub fn prod(&self, range: impl RangeBounds<usize>) -> M::S {
        let (l, r) = self.range_to_pair(range);
        assert!(l <= r && r <= self.len());
        let r = r + 1;
        let s = &self.t[63 - (l ^ r).leading_zeros() as usize];
        M::op(&s[l], &s[r])
    }

    fn range_to_pair<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let n = self.len();
        let l = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&r) => (r + 1).min(n),
            Bound::Excluded(&r) => r.min(n),
            Bound::Unbounded => n,
        };
        (l, r)
    }
}
