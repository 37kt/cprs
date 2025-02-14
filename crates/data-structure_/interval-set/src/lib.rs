use std::collections::BTreeMap;

/// 区間集合を管理するデータ構造。
#[derive(Debug, Clone)]
pub struct IntervalSet<T: Clone + Ord>(BTreeMap<T, T>);

impl<T: Clone + Ord> IntervalSet<T> {
    /// 空の IntervalSet を構築する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// 区間 [l, r) を追加する。
    ///
    /// # 引数
    ///
    /// - `l`: 区間の始点
    /// - `r`: 区間の終点
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn insert(&mut self, mut l: T, mut r: T) {
        assert!(l <= r);
        if l == r {
            return;
        }

        if let Some((a, b)) = self.0.range(..=&l).next_back() {
            if &l <= b {
                l = a.clone();
            }
        }
        for (a, b) in self
            .0
            .range(&l..=&r)
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect::<Vec<_>>()
        {
            self.0.remove(&a);
            r = r.max(b);
        }
        self.0.insert(l, r);
    }

    /// lower 以上の値のうち区間に含まれない最小のものを取得する。
    ///
    /// # 引数
    ///
    /// - `lower`: 下限
    ///
    /// # 戻り値
    ///
    /// - lower 以上の値のうち区間に含まれない最小のもの
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn mex(&self, lower: T) -> T {
        if let Some((_, r)) = self.0.range(..=&lower).next_back() {
            lower.max(r.clone())
        } else {
            lower
        }
    }

    /// x が区間に含まれるかどうかを判定する。
    ///
    /// # 引数
    ///
    /// - `x`: 判定する値
    ///
    /// # 戻り値
    ///
    /// - x が区間に含まれるかどうか
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn contains(&self, x: T) -> bool {
        if let Some((_, r)) = self.0.range(..=&x).next_back() {
            &x < r
        } else {
            false
        }
    }

    /// 区間集合を取得する。
    ///
    /// # 戻り値
    ///
    /// - 区間集合
    ///
    /// # 計算量
    ///
    /// O(N)
    pub fn intervals(&self) -> impl Iterator<Item = (T, T)> {
        self.0.clone().into_iter()
    }
}
