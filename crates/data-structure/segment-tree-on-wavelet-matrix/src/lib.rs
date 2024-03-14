use std::mem::swap;

use algebraic::Monoid;
use segment_tree::SegmentTree;

pub struct SegmentTreeOnWaveletMatrix<I, M>
where
    I: Ord + Copy,
    M: Monoid,
    M::S: Clone,
{
    n: usize,
    lg: usize,
    bv: Vec<BitVector>,
    seg: Vec<SegmentTree<M>>,
    mid: Vec<usize>,
    ps: Vec<(I, I)>,
    ys: Vec<I>,
}

impl<I, M> SegmentTreeOnWaveletMatrix<I, M>
where
    I: Ord + Copy,
    M: Monoid,
    M::S: Clone,
{
    pub fn new(mut ps: Vec<(I, I)>) -> Self {
        ps.sort();
        ps.dedup();
        let n = ps.len();
        let mut ys = ps.iter().map(|&(_, y)| y).collect::<Vec<_>>();
        ys.sort();
        ys.dedup();
        let mut cur = vec![0; n];
        let mut nxt = vec![0; n];
        for i in 0..n {
            cur[i] = ys.binary_search(&ps[i].1).unwrap();
        }
        let lg = 64 - n.max(1).leading_zeros() as usize + 1;
        let mut bv = vec![BitVector::new(n + 1); lg];
        let seg = (0..lg).map(|_| SegmentTree::<M>::new(n)).collect();
        let mut mid = vec![0; lg];
        for h in (0..lg).rev() {
            for i in 0..n {
                if cur[i] >> h & 1 == 1 {
                    bv[h].set(i);
                }
            }
            bv[h].build();
            let mut it = [0, bv[h].rank0(n)];
            mid[h] = it[1];
            for i in 0..n {
                let t = bv[h].access(i) as usize;
                nxt[it[t]] = cur[i];
                it[t] += 1;
            }
            swap(&mut cur, &mut nxt);
        }
        Self {
            n,
            lg,
            bv,
            seg,
            mid,
            ps,
            ys,
        }
    }

    pub fn set(&mut self, (x, y): (I, I), v: M::S) {
        let mut i = self.ps.binary_search(&(x, y)).unwrap();
        for h in (0..self.lg).rev() {
            let i0 = self.bv[h].rank0(i);
            if self.bv[h].access(i) {
                i += self.bv[h].rank0(self.n) - i0;
            } else {
                i = i0;
            }
            self.seg[h].set(i, v.clone());
        }
    }

    pub fn add(&mut self, (x, y): (I, I), v: M::S) {
        let mut i = self.ps.binary_search(&(x, y)).unwrap();
        for h in (0..self.lg).rev() {
            let i0 = self.bv[h].rank0(i);
            if self.bv[h].access(i) {
                i += self.bv[h].rank0(self.n) - i0;
            } else {
                i = i0;
            }
            let t = self.seg[h].get(i);
            self.seg[h].set(i, M::op(&t, &v));
        }
    }

    pub fn prod(
        &self,
        range_x: impl std::ops::RangeBounds<I>,
        range_y: impl std::ops::RangeBounds<I>,
    ) -> M::S {
        let l = match range_x.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&l) => self.ps.partition_point(|&(x, _)| x < l),
            std::ops::Bound::Excluded(&l) => self.ps.partition_point(|&(x, _)| x <= l),
        };
        let r = match range_x.end_bound() {
            std::ops::Bound::Unbounded => self.ps.len(),
            std::ops::Bound::Included(&r) => self.ps.partition_point(|&(x, _)| x <= r),
            std::ops::Bound::Excluded(&r) => self.ps.partition_point(|&(x, _)| x < r),
        };
        let d = match range_y.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&d) => self.ys.partition_point(|&y| y < d),
            std::ops::Bound::Excluded(&d) => self.ys.partition_point(|&y| y <= d),
        };
        let u = match range_y.end_bound() {
            std::ops::Bound::Unbounded => self.ys.len(),
            std::ops::Bound::Included(&u) => self.ys.partition_point(|&y| y <= u),
            std::ops::Bound::Excluded(&u) => self.ys.partition_point(|&y| y < u),
        };
        self.prod_(l, r, d, u, self.lg)
    }

    fn prod_(&self, l: usize, r: usize, d: usize, u: usize, h: usize) -> M::S {
        let u = u.min(1 << h);
        if l >= r || d >= u {
            return M::e();
        }
        if d == 0 && u == 1 << h {
            return self.seg[h].prod(l..r);
        }
        let h = h - 1;
        let l0 = self.bv[h].rank0(l);
        let r0 = self.bv[h].rank0(r);
        M::op(
            &self.prod_(l0, r0, d, u, h),
            &self.prod_(
                l + self.mid[h] - l0,
                r + self.mid[h] - r0,
                d.saturating_sub(1 << h),
                u.saturating_sub(1 << h),
                h,
            ),
        )
    }
}

const W: usize = 64;

#[derive(Clone)]
struct BitVector {
    bit: Vec<usize>,
    sum: Vec<usize>,
}

impl BitVector {
    fn new(n: usize) -> Self {
        Self {
            bit: vec![0; (n + 63) / W],
            sum: vec![0; (n + 63) / W],
        }
    }

    fn set(&mut self, k: usize) {
        self.bit[k / W] |= 1 << k % W;
    }

    fn build(&mut self) {
        self.sum[0] = 0;
        for i in 1..self.sum.len() {
            self.sum[i] = self.sum[i - 1] + self.bit[i - 1].count_ones() as usize;
        }
    }

    fn access(&self, k: usize) -> bool {
        self.bit[k / W] >> k % W & 1 == 1
    }

    fn rank1(&self, k: usize) -> usize {
        self.sum[k / W] + (self.bit[k / W] & (1 << k % W) - 1).count_ones() as usize
    }

    fn rank0(&self, k: usize) -> usize {
        k - self.rank1(k)
    }
}
