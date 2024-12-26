// verify: https://atcoder.jp/contests/abc217/tasks/abc217_h

use std::{cmp::Reverse, collections::BinaryHeap, mem::swap};

/// 区分線形凸関数を扱う。
pub struct SlopeTrick {
    add_l: i64,
    add_r: i64,
    min_y: i64,
    lq: BinaryHeap<(i64, i64)>,
    rq: BinaryHeap<Reverse<(i64, i64)>>,
}

impl SlopeTrick {
    /// 区分線形凸関数を f(x) = 0 で初期化する。
    pub fn new() -> Self {
        Self {
            add_l: 0,
            add_r: 0,
            min_y: 0,
            lq: Default::default(),
            rq: Default::default(),
        }
    }

    /// 最小値とその時の x を取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn min(&self) -> (i64, i64) {
        (
            self.min_y,
            self.get_l().or(self.get_r()).map(|(x, _)| x).unwrap_or(0),
        )
    }

    /// f(x) <- f(x) + a
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn add_a(&mut self, a: i64) {
        self.min_y += a;
    }

    /// f(x) <- f(x - a)
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn shift(&mut self, a: i64) {
        self.add_l += a;
        self.add_r += a;
    }

    /// f(x) <- min f(y) (x-b <= y <= x-a)
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn slide_min(&mut self, a: i64, b: i64) {
        assert!(a <= b);
        self.add_l += a;
        self.add_r += b;
    }

    /// f(x) <- f(x) + (x-a).max(0)
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn add_x_minus_a(&mut self, a: i64, c: i64) {
        let mut used = 0;
        while used < c && !self.lq.is_empty() {
            let (x, d) = self.get_l().unwrap();
            if x <= a {
                break;
            }
            self.pop_l();
            let t = d.min(c - used);
            self.push_r(x, t);
            if d != t {
                self.push_l(x, d - t);
            }
            self.min_y += (x - a) * t;
            used += t;
        }
        if used != 0 {
            self.push_l(a, used);
        }
        if c - used != 0 {
            self.push_r(a, c - used);
        }
    }

    /// f(x) <- f(x) + (a-x).max(0)
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn add_a_minus_x(&mut self, a: i64, c: i64) {
        let mut used = 0;
        while used < c && !self.rq.is_empty() {
            let (x, d) = self.get_r().unwrap();
            if x >= a {
                break;
            }
            self.pop_r();
            let t = d.min(c - used);
            self.push_l(x, t);
            if d != t {
                self.push_r(x, d - t);
            }
            self.min_y += (a - x) * t;
            used += t;
        }
        if used != 0 {
            self.push_r(a, used);
        }
        if c - used != 0 {
            self.push_l(a, c - used);
        }
    }

    /// f(x) <- f(x) + |x-a|
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn add_abs_x_minus_a(&mut self, a: i64, c: i64) {
        self.add_x_minus_a(a, c);
        self.add_a_minus_x(a, c);
    }

    /// 右方向に累積minをとる。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn chmin_right(&mut self) {
        self.rq.clear();
    }

    /// 左方向に累積minをとる。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn chmin_left(&mut self) {
        self.lq.clear();
    }

    /// 2つの区分線形凸関数をマージする。
    ///
    /// # 計算量
    ///
    /// O(n)
    pub fn merge(&mut self, mut other: Self) {
        if self.lq.len() + self.rq.len() < other.lq.len() + other.rq.len() {
            swap(self, &mut other);
        }
        while let Some((x, c)) = other.lq.pop() {
            self.add_a_minus_x(x, c);
        }
        while let Some(Reverse((x, c))) = other.rq.pop() {
            self.add_x_minus_a(x, c);
        }
        self.add_a(other.min_y);
    }

    fn push_l(&mut self, x: i64, c: i64) {
        self.lq.push((x - self.add_l, c));
    }

    fn push_r(&mut self, x: i64, c: i64) {
        self.rq.push(Reverse((x - self.add_r, c)));
    }

    fn get_l(&self) -> Option<(i64, i64)> {
        self.lq.peek().map(|&(x, c)| (x + self.add_l, c))
    }

    fn get_r(&self) -> Option<(i64, i64)> {
        self.rq.peek().map(|&Reverse((x, c))| (x + self.add_r, c))
    }

    fn pop_l(&mut self) {
        self.lq.pop();
    }

    fn pop_r(&mut self) {
        self.rq.pop();
    }
}
