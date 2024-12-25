use std::{fmt::Debug, mem::swap};

/// Double-ended priority queue.
#[derive(Clone)]
pub struct IntervalHeap<T: Clone + Ord>(Vec<T>);

impl<T: Clone + Ord + Debug> Debug for IntervalHeap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Vec から IntervalHeap を構築する。
///
/// # 計算量
///
/// O(N)
impl<T: Clone + Ord> From<Vec<T>> for IntervalHeap<T> {
    fn from(value: Vec<T>) -> Self {
        let mut a = Self(value);
        a.make_heap();
        a
    }
}

impl<T: Clone + Ord> IntervalHeap<T> {
    /// 空の IntervalHeap を構築する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn new() -> Self {
        Self(vec![])
    }

    /// 要素数を取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 空かどうかを判定する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 要素を追加する。
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn push(&mut self, x: T) {
        let k = self.len();
        self.0.push(x);
        self.up(k, 1);
    }

    /// 2 つの IntervalHeap をマージする。
    ///
    /// # 計算量
    ///
    /// O(N)
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }
        let n = self.len();
        self.0.append(&mut other.0);
        for i in (n..self.len()).rev() {
            self.up(i, 1);
        }
    }

    /// 要素を全て削除する。
    ///
    /// # 計算量
    ///
    /// O(N)
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// 最小値を取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn peek_min(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else if self.len() == 1 {
            Some(&self.0[0])
        } else {
            Some(&self.0[1])
        }
    }

    /// 最大値を取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn peek_max(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.0[0])
        }
    }

    /// 最小値を削除する。
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn pop_min(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.len() < 3 {
            self.0.pop()
        } else {
            let n = self.len();
            self.0.swap(1, n - 1);
            let res = self.0.pop();
            let k = self.down(1);
            self.up(k, 1);
            res
        }
    }

    /// 最大値を削除する。
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn pop_max(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.len() < 2 {
            self.0.pop()
        } else {
            let n = self.len();
            self.0.swap(0, n - 1);
            let res = self.0.pop();
            let k = self.down(0);
            self.up(k, 1);
            res
        }
    }

    fn make_heap(&mut self) {
        for i in (0..self.0.len()).rev() {
            if i % 2 == 1 && self.0[i - 1] < self.0[i] {
                self.0.swap(i - 1, i);
            }
            let k = self.down(i);
            self.up(k, i);
        }
    }

    fn parent(k: usize) -> usize {
        (k >> 1).wrapping_sub(1) & !1
    }

    fn down(&mut self, mut k: usize) -> usize {
        let n = self.0.len();
        if k % 2 == 1 {
            while k * 2 + 1 < n {
                let mut c = k * 2 + 3;
                if n <= c || self.0[c - 2] < self.0[c] {
                    c -= 2;
                }
                if c < n && self.0[c] < self.0[k] {
                    self.0.swap(k, c);
                    k = c;
                } else {
                    break;
                }
            }
        } else {
            while k * 2 + 2 < n {
                let mut c = k * 2 + 4;
                if n <= c || self.0[c] < self.0[c - 2] {
                    c -= 2;
                }
                if c < n && self.0[k] < self.0[c] {
                    self.0.swap(k, c);
                    k = c;
                } else {
                    break;
                }
            }
        }
        k
    }

    fn up(&mut self, mut k: usize, root: usize) -> usize {
        if k | 1 < self.0.len() && self.0[k & !1] < self.0[k | 1] {
            self.0.swap(k & !1, k | 1);
            k ^= 1;
        }
        let mut p;
        while root < k {
            p = Self::parent(k);
            if self.0[p] >= self.0[k] {
                break;
            }
            self.0.swap(p, k);
            k = p;
        }
        while root < k {
            p = Self::parent(k) | 1;
            if self.0[k] >= self.0[p] {
                break;
            }
            self.0.swap(p, k);
            k = p;
        }
        k
    }
}
