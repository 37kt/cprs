use std::ops::{Bound, RangeBounds};

const W: usize = 64;

#[derive(Clone)]
struct BitVector {
    bit: Vec<usize>,
    sum: Vec<usize>,
}

pub struct WaveletMatrix {
    n: usize,
    mat: Vec<BitVector>,
    mid: Vec<usize>,
}

impl WaveletMatrix {
    pub fn new(mut a: Vec<usize>) -> Self {
        let n = a.len();
        let max = a.iter().max().max(Some(&2)).unwrap();
        let m = 64 - max.leading_zeros() as usize;
        let mut mat = vec![BitVector::new(n + 1); m];
        let mut mid = vec![0; m];
        for d in (0..m).rev() {
            let mut l = vec![];
            let mut r = vec![];
            for i in 0..n {
                if a[i] >> d & 1 == 1 {
                    mat[d].set(i);
                    r.push(a[i]);
                } else {
                    l.push(a[i]);
                }
            }
            mid[d] = l.len();
            mat[d].build();
            a = l;
            a.append(&mut r);
        }
        Self { n, mat, mid }
    }

    pub fn access(&self, mut k: usize) -> usize {
        let mut res = 0;
        for d in (0..self.mat.len()).rev() {
            let f = self.mat[d].access(k);
            if f {
                res |= 1 << d;
                k = self.mat[d].rank1(k) + self.mid[d];
            } else {
                k = self.mat[d].rank0(k);
            }
        }
        res
    }

    pub fn kth_smallest<R: RangeBounds<usize>>(&self, range: R, mut k: usize) -> usize {
        let (mut l, mut r) = range_to_pair(range, self.n);
        assert!(k < r - l);
        let mut res = 0;
        for d in (0..self.mat.len()).rev() {
            let cnt = self.mat[d].rank0(r) - self.mat[d].rank0(l);
            if cnt <= k {
                res |= 1 << d;
                k -= cnt;
                let (ll, rr) = self.succ1(l..r, d);
                l = ll;
                r = rr;
            } else {
                let (ll, rr) = self.succ0(l..r, d);
                l = ll;
                r = rr;
            }
        }
        res
    }

    pub fn kth_largest<R: RangeBounds<usize>>(&self, range: R, k: usize) -> usize {
        let (l, r) = range_to_pair(range, self.n);
        self.kth_smallest(l..r, r - l - k - 1)
    }

    pub fn range_freq<IR, VR>(&self, index_range: IR, value_range: VR) -> usize
    where
        IR: RangeBounds<usize>,
        VR: RangeBounds<usize>,
    {
        let (il, ir) = range_to_pair(index_range, self.n);
        let (vl, vr) = range_to_pair(value_range, 1 << self.mat.len());
        self.range_freq_(il, ir, vr) - self.range_freq_(il, ir, vl)
    }

    pub fn prev_value<R: RangeBounds<usize>>(&self, range: R, upper: usize) -> Option<usize> {
        let (l, r) = range_to_pair(range, self.n);
        let cnt = self.range_freq_(l, r, upper);
        if cnt == 0 {
            None
        } else {
            Some(self.kth_smallest(l..r, cnt - 1))
        }
    }

    pub fn next_value<R: RangeBounds<usize>>(&self, range: R, lower: usize) -> Option<usize> {
        let (l, r) = range_to_pair(range, self.n);
        let cnt = self.range_freq_(l, r, lower);
        if cnt == r - l {
            None
        } else {
            Some(self.kth_smallest(l..r, cnt))
        }
    }

    fn succ1<R: RangeBounds<usize>>(&self, range: R, d: usize) -> (usize, usize) {
        let (l, r) = range_to_pair(range, self.n);
        (
            self.mat[d].rank1(l) + self.mid[d],
            self.mat[d].rank1(r) + self.mid[d],
        )
    }

    fn succ0<R: RangeBounds<usize>>(&self, range: R, d: usize) -> (usize, usize) {
        let (l, r) = range_to_pair(range, self.n);
        (self.mat[d].rank0(l), self.mat[d].rank0(r))
    }

    fn range_freq_(&self, mut l: usize, mut r: usize, upper: usize) -> usize {
        if upper >= 1 << self.mat.len() {
            return r - l;
        }
        let mut res = 0;
        for d in (0..self.mat.len()).rev() {
            if upper >> d & 1 == 1 {
                res += self.mat[d].rank0(r) - self.mat[d].rank0(l);
                let (ll, rr) = self.succ1(l..r, d);
                l = ll;
                r = rr;
                // (l, r) = self.succ1(l..r, d); はRust 1.14.0ではできない
            } else {
                let (ll, rr) = self.succ0(l..r, d);
                l = ll;
                r = rr;
            }
        }
        res
    }
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

fn range_to_pair<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    let l = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Excluded(&l) => (l + 1).min(n),
        Bound::Included(&l) => l.min(n),
    };
    let r = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Excluded(&r) => r.min(n),
        Bound::Included(&r) => (r + 1).min(n),
    };
    assert!(l <= r);
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let wm = WaveletMatrix::new(vec![]);
        assert_eq!(wm.range_freq(0..0, 1..=1), 0);
        assert_eq!(wm.range_freq(0..0, 0..=0), 0);
        assert_eq!(wm.range_freq(0..0, 3..=3), 0);
        assert_eq!(wm.range_freq(0..0, 1_000_000_000..=1_000_000_000), 0);

        let wm = WaveletMatrix::new(vec![3, 7, 1, 2, 1]);
        assert_eq!(wm.range_freq(1..5, 1..=1), 2);
        assert_eq!(wm.range_freq(3..3, 0..=0), 0);
        assert_eq!(wm.range_freq(0..4, 3..=3), 1);
    }
}
