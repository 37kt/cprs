use std::ops::{Bound, RangeBounds};

use algebraic::Monoid;
use segment_tree::SegmentTree;

pub struct RangeTree<I, M>
where
    I: Ord + Copy,
    M: Monoid,
    M::S: Clone,
{
    n: usize,
    seg: Vec<SegmentTree<M>>,
    ps: Vec<(I, I)>,
    ys: Vec<Vec<I>>,
}

impl<I, M> RangeTree<I, M>
where
    I: Ord + Copy,
    M: Monoid,
    M::S: Clone,
{
    pub fn new(mut ps: Vec<(I, I)>) -> Self {
        ps.sort();
        ps.dedup();
        let n = ps.len();
        let mut seg: Vec<_> = (0..n * 2).map(|_| SegmentTree::new(0)).collect();
        let mut ys = vec![vec![]; n * 2];
        for i in 0..n {
            ys[n + i].push(ps[i].1);
            seg[n + i] = SegmentTree::new(1);
        }
        for i in (1..n).rev() {
            ys[i] = merge(&ys[i << 1], &ys[i << 1 | 1]);
            seg[i] = SegmentTree::new(ys[i].len());
        }
        Self { n, seg, ps, ys }
    }

    pub fn get(&self, (x, y): (I, I)) -> M::S {
        let i = self.ps.partition_point(|&p| p < (x, y));
        assert!(self.ps[i] == (x, y));
        self.seg[self.n + i].get(0)
    }

    pub fn add(&mut self, (x, y): (I, I), v: M::S) {
        let mut i = self.ps.partition_point(|&p| p < (x, y));
        assert!(self.ps[i] == (x, y));
        i += self.n;
        while i > 0 {
            let j = self.ys[i].partition_point(|&t| t < y);
            let t = self.seg[i].get(j);
            self.seg[i].set(j, M::op(&t, &v));
            i >>= 1;
        }
    }

    pub fn prod(&self, range_x: impl RangeBounds<I>, range_y: impl RangeBounds<I>) -> M::S {
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
        let mut pl = M::e();
        let mut pr = M::e();
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
                pl = M::op(&pl, &self.seg[l].prod(a..b));
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
                pr = M::op(&self.seg[r].prod(a..b), &pr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&pl, &pr)
    }
}

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
