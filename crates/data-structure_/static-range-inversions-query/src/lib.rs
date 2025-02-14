use std::{
    cmp::Ordering,
    ops::{Bound, RangeBounds},
};

use div::{div_ceil, div_floor};

/// 静的な列の区間転倒数クエリ

#[derive(Clone)]
pub struct StaticRangeInversionsQuery<T>
where
    T: Copy + PartialOrd,
{
    n: usize,
    block_size: usize,
    block_num: usize,
    sorted: Vec<Vec<(T, usize)>>,
    inv: Vec<Vec<usize>>,
    inv_rev: Vec<Vec<usize>>,
}

impl<T> StaticRangeInversionsQuery<T>
where
    T: Copy + PartialOrd + Ord + Default,
{
    /// 静的な列の区間転倒数クエリを構築する。
    ///
    /// # 計算量
    ///
    /// O(n √n)
    pub fn new(a: &[T]) -> Self {
        let n = a.len();
        let block_size = 1.max((n as f64).sqrt().ceil() as usize * 2);
        let block_num = 1.max(div_ceil(n, block_size));
        let mut a = a.to_vec();
        let max = a.iter().max().copied().unwrap_or_else(T::default);
        a.extend(std::iter::repeat(max).take(block_num * block_size - a.len()));

        let mut sorted = vec![vec![]; block_num];
        for block_i in 0..block_num {
            let l = block_i * block_size;
            let r = (block_i + 1) * block_size;
            sorted[block_i] = a[l..r]
                .iter()
                .copied()
                .enumerate()
                .map(|(i, x)| (x, l + i))
                .collect::<Vec<_>>();
            sorted[block_i].sort_unstable();
        }

        let mut res = Self {
            n,
            block_size,
            block_num,
            sorted,
            inv: vec![],
            inv_rev: vec![],
        };

        res.inv = res.build(&a, |x, y| x.cmp(y));
        a.reverse();
        res.inv_rev = res.build(&a, |x, y| y.cmp(x));
        res.inv_rev.reverse();
        res
    }

    /// 区間 range の転倒数を取得する。
    ///
    /// # 計算量
    ///
    /// O(√n)
    pub fn inversions(&self, range: impl RangeBounds<usize>) -> usize {
        let (l, r) = self.range_to_pair(range);
        assert!(l <= r && r <= self.n);

        let bl = div_floor(l, self.block_size);
        let br = div_ceil(r, self.block_size);
        if br - bl == 1 {
            let mut res = self.inv[bl][r - bl * self.block_size]
                + self.inv_rev[bl + 1][(bl + 1) * self.block_size - l]
                - self.inv[bl][self.block_size];
            let mut cnt = 0;
            for k in (0..self.block_size).rev() {
                if self.sorted[bl][k].1 < l {
                    cnt += 1;
                }
                if self.sorted[bl][k].1 >= r {
                    res += cnt;
                }
            }
            res
        } else {
            let ml = (bl + 1) * self.block_size;
            let mr = (br - 1) * self.block_size;
            let mut res =
                self.inv[bl + 1][r - ml] + self.inv_rev[br - 1][mr - l] - self.inv[bl + 1][mr - ml];
            let mut cnt = 0;
            let mut j = self.block_size;
            for k in (0..self.block_size).rev() {
                if self.sorted[br - 1][k].1 >= r {
                    continue;
                }
                while j > 0 && self.sorted[bl][j - 1].0 > self.sorted[br - 1][k].0 {
                    j -= 1;
                    if self.sorted[bl][j].1 >= l {
                        cnt += 1;
                    }
                }
                res += cnt;
            }
            res
        }
    }

    fn build(&mut self, a: &[T], cmp: impl Fn(&T, &T) -> Ordering) -> Vec<Vec<usize>> {
        let mut inv = vec![vec![]; self.block_num + 1];

        let mut suf: Vec<(T, usize)> = vec![];
        for block_i in (0..self.block_num).rev() {
            let l = block_i * self.block_size;
            let r = (block_i + 1) * self.block_size;
            inv[block_i] = vec![0; self.block_size * (self.block_num - block_i) + 1];
            let mut pre = a[l..r]
                .iter()
                .copied()
                .enumerate()
                .map(|(i, x)| (x, l + i))
                .collect::<Vec<_>>();
            pre.sort_unstable_by(|x, y| cmp(&x.0, &y.0));
            for i in 0..self.block_size {
                inv[block_i][i + 1] = inv[block_i][i];
                for j in 0..i {
                    if cmp(&a[l + i], &a[l + j]) == Ordering::Less {
                        inv[block_i][i + 1] += 1;
                    }
                }
            }

            let mut j = self.block_size;
            for i in (0..self.block_size * (self.block_num - 1 - block_i)).rev() {
                while j > 0 && cmp(&suf[i].0, &pre[j - 1].0) == Ordering::Less {
                    j -= 1;
                }
                inv[block_i][suf[i].1 - l + 1] += self.block_size - j;
            }
            for i in self.block_size..self.block_size * (self.block_num - block_i) {
                inv[block_i][i + 1] += inv[block_i][i] + inv[block_i + 1][i + 1 - self.block_size]
                    - inv[block_i + 1][i - self.block_size];
            }

            suf = Self::merge(&suf, &pre, |x, y| cmp(&x.0, &y.0));
        }

        inv
    }

    fn merge<U: Copy>(a: &[U], b: &[U], cmp: impl Fn(&U, &U) -> Ordering) -> Vec<U> {
        let mut res = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            if cmp(&a[i], &b[j]) != Ordering::Greater {
                res.push(a[i]);
                i += 1;
            } else {
                res.push(b[j]);
                j += 1;
            }
        }
        res.extend_from_slice(&a[i..]);
        res.extend_from_slice(&b[j..]);
        res
    }

    fn range_to_pair(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
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
