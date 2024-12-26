use std::ops::{Bound, RangeBounds};

type T = i64;
const INF: T = std::i64::MAX / 2;

/// Segment Tree Beats!
/// 区間加算、区間chmin、区間chmax、区間最大値、区間最小値、区間和
#[derive(Clone)]
pub struct SegtreeBeats {
    n: usize,
    max_v: Vec<T>,
    smax_v: Vec<T>,
    max_c: Vec<T>,
    min_v: Vec<T>,
    smin_v: Vec<T>,
    min_c: Vec<T>,
    sum: Vec<T>,
    len: Vec<T>,
    ladd: Vec<T>,
    lval: Vec<T>,
}

impl From<&[T]> for SegtreeBeats {
    /// 長さ n の列を a で初期化する。
    ///
    /// # 引数
    ///
    /// - `a`: 初期化する列
    ///
    /// # 計算量
    ///
    /// O(N)
    fn from(a: &[T]) -> Self {
        let mut n = 1;
        while n < a.len() {
            n *= 2;
        }

        let mut max_v = vec![0; n * 2];
        let mut smax_v = vec![0; n * 2];
        let mut max_c = vec![0; n * 2];
        let mut min_v = vec![0; n * 2];
        let mut smin_v = vec![0; n * 2];
        let mut min_c = vec![0; n * 2];
        let mut sum = vec![0; n * 2];
        let mut len = vec![0; n * 2];
        let ladd = vec![0; n * 2];
        let lval = vec![INF; n * 2];

        len[0] = n as i64;
        for i in 0..n - 1 {
            len[i * 2 + 1] = len[i] / 2;
            len[i * 2 + 2] = len[i] / 2;
        }

        for i in 0..a.len() {
            max_v[n - 1 + i] = a[i];
            min_v[n - 1 + i] = a[i];
            sum[n - 1 + i] = a[i];
            smax_v[n - 1 + i] = -INF;
            smin_v[n - 1 + i] = INF;
            max_c[n - 1 + i] = 1;
            min_c[n - 1 + i] = 1;
        }

        for i in a.len()..n {
            max_v[n - 1 + i] = -INF;
            smax_v[n - 1 + i] = -INF;
            min_v[n - 1 + i] = INF;
            smin_v[n - 1 + i] = INF;
        }

        let mut seg = SegtreeBeats {
            n,
            max_v,
            smax_v,
            max_c,
            min_v,
            smin_v,
            min_c,
            sum,
            len,
            ladd,
            lval,
        };

        for i in (0..n - 1).rev() {
            seg.update(i);
        }

        seg
    }
}

impl SegtreeBeats {
    /// 長さ n の列を 0 で初期化する。
    ///
    /// # 引数
    ///
    /// - `n`: 列の長さ
    ///
    /// # 計算量
    ///
    /// O(N)
    pub fn new(n: usize) -> Self {
        Self::from(vec![0; n].as_slice())
    }

    /// 区間 chmin
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    /// - `x`: 更新する値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn chmin<R>(&mut self, range: R, x: T)
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.chmin_(x, a, b, 0, 0, self.n);
    }

    /// 区間 chmax
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    /// - `x`: 更新する値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn chmax<R>(&mut self, range: R, x: T)
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.chmax_(x, a, b, 0, 0, self.n);
    }

    /// 区間加算
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    /// - `x`: 加算する値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn add<R>(&mut self, range: R, x: T)
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.add_(x, a, b, 0, 0, self.n);
    }

    /// 区間更新
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    /// - `x`: 更新する値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn set<R>(&mut self, range: R, x: T)
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.set_(x, a, b, 0, 0, self.n);
    }

    /// 区間最大値
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn max<R>(&mut self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.max_(a, b, 0, 0, self.n)
    }

    /// 区間最小値
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn min<R>(&mut self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.min_(a, b, 0, 0, self.n)
    }

    /// 区間和
    ///
    /// # 引数
    ///
    /// - `range`: 区間
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn sum<R>(&mut self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let (a, b) = self.range_to_pair(range);
        self.sum_(a, b, 0, 0, self.n)
    }

    fn update_node_max(&mut self, k: usize, x: T) {
        self.sum[k] += (x - self.max_v[k]) * self.max_c[k];

        if self.max_v[k] == self.min_v[k] {
            self.max_v[k] = x;
            self.min_v[k] = x;
        } else if self.max_v[k] == self.smin_v[k] {
            self.max_v[k] = x;
            self.smin_v[k] = x;
        } else {
            self.max_v[k] = x;
        }

        if self.lval[k] != INF && x < self.lval[k] {
            self.lval[k] = x;
        }
    }

    fn update_node_min(&mut self, k: usize, x: T) {
        self.sum[k] += (x - self.min_v[k]) * self.min_c[k];

        if self.max_v[k] == self.min_v[k] {
            self.max_v[k] = x;
            self.min_v[k] = x;
        } else if self.smax_v[k] == self.min_v[k] {
            self.min_v[k] = x;
            self.smax_v[k] = x;
        } else {
            self.min_v[k] = x;
        }

        if self.lval[k] != INF && self.lval[k] < x {
            self.lval[k] = x;
        }
    }

    fn add_all(&mut self, k: usize, x: T) {
        self.max_v[k] += x;
        if self.smax_v[k] != -INF {
            self.smax_v[k] += x;
        }
        self.min_v[k] += x;
        if self.smin_v[k] != INF {
            self.smin_v[k] += x;
        }

        self.sum[k] += self.len[k] * x;
        if self.lval[k] != INF {
            self.lval[k] += x;
        } else {
            self.ladd[k] += x;
        }
    }

    fn set_all(&mut self, k: usize, x: T) {
        self.max_v[k] = x;
        self.smax_v[k] = -INF;
        self.min_v[k] = x;
        self.smin_v[k] = INF;
        self.max_c[k] = self.len[k];
        self.min_c[k] = self.len[k];

        self.sum[k] = x * self.len[k];
        self.lval[k] = x;
        self.ladd[k] = 0;
    }

    fn push(&mut self, k: usize) {
        if self.n - 1 <= k {
            return;
        }

        if self.lval[k] != INF {
            self.set_all(k * 2 + 1, self.lval[k]);
            self.set_all(k * 2 + 2, self.lval[k]);
            self.lval[k] = INF;
            return;
        }

        if self.ladd[k] != 0 {
            self.add_all(k * 2 + 1, self.ladd[k]);
            self.add_all(k * 2 + 2, self.ladd[k]);
            self.ladd[k] = 0;
        }

        if self.max_v[k] < self.max_v[k * 2 + 1] {
            self.update_node_max(k * 2 + 1, self.max_v[k]);
        }
        if self.min_v[k * 2 + 1] < self.min_v[k] {
            self.update_node_min(k * 2 + 1, self.min_v[k]);
        }

        if self.max_v[k] < self.max_v[k * 2 + 2] {
            self.update_node_max(k * 2 + 2, self.max_v[k]);
        }
        if self.min_v[k * 2 + 2] < self.min_v[k] {
            self.update_node_min(k * 2 + 2, self.min_v[k]);
        }
    }

    fn update(&mut self, k: usize) {
        self.sum[k] = self.sum[k * 2 + 1] + self.sum[k * 2 + 2];

        if self.max_v[k * 2 + 1] < self.max_v[k * 2 + 2] {
            self.max_v[k] = self.max_v[k * 2 + 2];
            self.max_c[k] = self.max_c[k * 2 + 2];
            self.smax_v[k] = self.max_v[k * 2 + 1].max(self.smax_v[k * 2 + 2]);
        } else if self.max_v[k * 2 + 1] > self.max_v[k * 2 + 2] {
            self.max_v[k] = self.max_v[k * 2 + 1];
            self.max_c[k] = self.max_c[k * 2 + 1];
            self.smax_v[k] = self.smax_v[k * 2 + 1].max(self.max_v[k * 2 + 2]);
        } else {
            self.max_v[k] = self.max_v[k * 2 + 1];
            self.max_c[k] = self.max_c[k * 2 + 1] + self.max_c[k * 2 + 2];
            self.smax_v[k] = self.smax_v[k * 2 + 1].max(self.smax_v[k * 2 + 2]);
        }

        if self.min_v[k * 2 + 1] < self.min_v[k * 2 + 2] {
            self.min_v[k] = self.min_v[k * 2 + 1];
            self.min_c[k] = self.min_c[k * 2 + 1];
            self.smin_v[k] = self.smin_v[k * 2 + 1].min(self.min_v[k * 2 + 2]);
        } else if self.min_v[k * 2 + 1] > self.min_v[k * 2 + 2] {
            self.min_v[k] = self.min_v[k * 2 + 2];
            self.min_c[k] = self.min_c[k * 2 + 2];
            self.smin_v[k] = self.min_v[k * 2 + 1].min(self.smin_v[k * 2 + 2]);
        } else {
            self.min_v[k] = self.min_v[k * 2 + 1];
            self.min_c[k] = self.min_c[k * 2 + 1] + self.min_c[k * 2 + 2];
            self.smin_v[k] = self.smin_v[k * 2 + 1].min(self.smin_v[k * 2 + 2]);
        }
    }

    fn chmin_(&mut self, x: T, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a || self.max_v[k] <= x {
            return;
        }
        if a <= l && r <= b && self.smax_v[k] < x {
            self.update_node_max(k, x);
            return;
        }

        self.push(k);
        self.chmin_(x, a, b, k * 2 + 1, l, (l + r) / 2);
        self.chmin_(x, a, b, k * 2 + 2, (l + r) / 2, r);
        self.update(k);
    }

    fn chmax_(&mut self, x: T, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a || x <= self.min_v[k] {
            return;
        }
        if a <= l && r <= b && x < self.smin_v[k] {
            self.update_node_min(k, x);
            return;
        }

        self.push(k);
        self.chmax_(x, a, b, k * 2 + 1, l, (l + r) / 2);
        self.chmax_(x, a, b, k * 2 + 2, (l + r) / 2, r);
        self.update(k);
    }

    fn add_(&mut self, x: T, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.add_all(k, x);
            return;
        }

        self.push(k);
        self.add_(x, a, b, k * 2 + 1, l, (l + r) / 2);
        self.add_(x, a, b, k * 2 + 2, (l + r) / 2, r);
        self.update(k);
    }

    fn set_(&mut self, x: T, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.set_all(k, x);
            return;
        }

        self.push(k);
        self.set_(x, a, b, k * 2 + 1, l, (l + r) / 2);
        self.set_(x, a, b, k * 2 + 2, (l + r) / 2, r);
        self.update(k);
    }

    fn max_(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if b <= l || r <= a {
            return -INF;
        }
        if a <= l && r <= b {
            return self.max_v[k];
        }

        self.push(k);
        let lv = self.max_(a, b, k * 2 + 1, l, (l + r) / 2);
        let rv = self.max_(a, b, k * 2 + 2, (l + r) / 2, r);
        lv.max(rv)
    }

    fn min_(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if b <= l || r <= a {
            return INF;
        }
        if a <= l && r <= b {
            return self.min_v[k];
        }

        self.push(k);
        let lv = self.min_(a, b, k * 2 + 1, l, (l + r) / 2);
        let rv = self.min_(a, b, k * 2 + 2, (l + r) / 2, r);
        lv.min(rv)
    }

    fn sum_(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if b <= l || r <= a {
            return 0;
        }
        if a <= l && r <= b {
            return self.sum[k];
        }

        self.push(k);
        let lv = self.sum_(a, b, k * 2 + 1, l, (l + r) / 2);
        let rv = self.sum_(a, b, k * 2 + 2, (l + r) / 2, r);
        lv + rv
    }

    fn range_to_pair<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&r) => r + 1,
            Bound::Excluded(&r) => r,
            Bound::Unbounded => self.n,
        };
        (l, r)
    }
}
