use std::ops::{Bound, RangeBounds};

use algebraic::Monoid;
use dual_segment_tree::DualSegmentTree;

/// 双対 Range Tree
///
/// 2 次元平面上に配置されている作用素を管理するデータ構造。
///
/// # 計算量
///
/// - 構築: O(n log n)
/// - get: O(log n)
/// - apply: O(log n)
/// - apply_range: O(log n)
///
/// ここで、n は管理する点の数。
pub struct DualRangeTree<I, M>
where
    I: Ord + Copy,
    M: Monoid,
    M::S: Clone,
{
    n: usize,
    seg: Vec<DualSegmentTree<M>>,
    ps: Vec<(I, I)>,
    ys: Vec<Vec<I>>,
}

impl<I, M> DualRangeTree<I, M>
where
    I: Ord + Copy,
    M: Monoid,
    M::S: Clone,
{
    /// 双対 Range Tree を構築する。
    ///
    /// # 引数
    ///
    /// * `ps` - get クエリの引数として与えられる点の集合
    pub fn new(mut ps: Vec<(I, I)>) -> Self {
        ps.sort();
        ps.dedup();
        let n = ps.len();
        let mut seg: Vec<_> = (0..n * 2).map(|_| DualSegmentTree::new(0)).collect();
        let mut ys = vec![vec![]; n * 2];
        for i in 0..n {
            ys[n + i].push(ps[i].1);
            seg[n + i] = DualSegmentTree::new(1);
        }
        for i in (1..n).rev() {
            ys[i] = merge(&ys[i << 1], &ys[i << 1 | 1]);
            seg[i] = DualSegmentTree::new(ys[i].len());
        }
        Self { n, seg, ps, ys }
    }

    /// 座標 (x, y) に配置されている作用素を取得する。
    ///
    /// # 引数
    ///
    /// * `(x, y)` - 取得したい作用素の座標
    ///
    /// # パニック
    ///
    /// 指定された座標が構築時に与えられた点集合に含まれていない場合、パニックする。
    ///
    /// # 計算量
    ///
    /// O(log^2 n)
    pub fn get(&self, (x, y): (I, I)) -> M::S {
        let i = self.ps.partition_point(|&p| p < (x, y));
        assert!(self.ps[i] == (x, y));
        let mut k = i + self.n;
        let mut res = M::e();
        while k > 0 {
            let j = self.ys[k].partition_point(|&t| t < y);
            res = M::op(&res, &self.seg[k].get(j));
            k >>= 1;
        }
        res
    }

    /// 座標 (x, y) に配置されている作用素に f を合成する。
    ///
    /// # 引数
    ///
    /// * `(x, y)` - 作用素を適用する座標
    /// * `f` - 適用する作用素
    ///
    /// # パニック
    ///
    /// 指定された座標が構築時に与えられた点集合に含まれていない場合、パニックする。
    ///
    /// # 計算量
    ///
    /// O(log^2 n)
    pub fn apply(&mut self, (x, y): (I, I), f: M::S) {
        let mut i = self.ps.partition_point(|&p| p < (x, y));
        assert!(self.ps[i] == (x, y));
        i += self.n;
        while i > 0 {
            let j = self.ys[i].partition_point(|&t| t < y);
            self.seg[i].apply(j, f.clone());
            i >>= 1;
        }
    }

    /// x ∈ range_x かつ y ∈ range_y を満たす座標 (x, y) に配置されている作用素に f を合成する。
    ///
    /// # 引数
    ///
    /// * `range_x` - x 座標の範囲
    /// * `range_y` - y 座標の範囲
    /// * `f` - 適用する作用素
    ///
    /// # 計算量
    ///
    /// O(log^2 n)
    pub fn apply_range(
        &mut self,
        range_x: impl RangeBounds<I>,
        range_y: impl RangeBounds<I>,
        f: M::S,
    ) {
        let mut l = match range_x.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&l) => self.ps.partition_point(|&(x, _)| x < l),
            Bound::Excluded(&l) => self.ps.partition_point(|&(x, _)| x <= l),
        } + self.n;
        let mut r = match range_x.end_bound() {
            Bound::Unbounded => self.n,
            Bound::Excluded(&r) => self.ps.partition_point(|&(x, _)| x < r),
            Bound::Included(&r) => self.ps.partition_point(|&(x, _)| x <= r),
        } + self.n;
        while l < r {
            if l & 1 != 0 {
                let a = match range_y.start_bound() {
                    Bound::Unbounded => 0,
                    Bound::Included(&a) => self.ys[l].partition_point(|&y| y < a),
                    Bound::Excluded(&a) => self.ys[l].partition_point(|&y| y <= a),
                };
                let b = match range_y.end_bound() {
                    Bound::Unbounded => self.ys[l].len(),
                    Bound::Excluded(&b) => self.ys[l].partition_point(|&y| y < b),
                    Bound::Included(&b) => self.ys[l].partition_point(|&y| y <= b),
                };
                self.seg[l].apply_range(a..b, f.clone());
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                let a = match range_y.start_bound() {
                    Bound::Unbounded => 0,
                    Bound::Included(&a) => self.ys[r].partition_point(|&y| y < a),
                    Bound::Excluded(&a) => self.ys[r].partition_point(|&y| y <= a),
                };
                let b = match range_y.end_bound() {
                    Bound::Unbounded => self.ys[r].len(),
                    Bound::Excluded(&b) => self.ys[r].partition_point(|&y| y < b),
                    Bound::Included(&b) => self.ys[r].partition_point(|&y| y <= b),
                };
                self.seg[r].apply_range(a..b, f.clone());
            }
            l >>= 1;
            r >>= 1;
        }
    }
}

/// 2つのソート済み配列をマージする。
///
/// # 引数
///
/// * `a` - ソート済み配列
/// * `b` - ソート済み配列
///
/// # 戻り値
///
/// マージされたソート済み配列
fn merge<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Ord + Copy,
{
    let n = a.len();
    let m = b.len();
    let mut i = 0;
    let mut j = 0;
    let mut c = vec![];
    while i < n || j < m {
        let p = if j == m {
            a[i]
        } else if i == n {
            b[j]
        } else {
            a[i].min(b[j])
        };
        c.push(p);
        while i < n && a[i] == p {
            i += 1;
        }
        while j < m && b[j] == p {
            j += 1;
        }
    }
    c
}
