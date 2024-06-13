use std::ops::{Bound, RangeBounds};

use union_find::UnionFind;

pub struct RangeUnionFind {
    n: usize,
    uf: Vec<UnionFind>,
}

impl RangeUnionFind {
    pub fn new(n: usize) -> Self {
        let mut log = 1;
        while 1 << log < n {
            log += 1;
        }
        let uf = (0..log).map(|i| UnionFind::new(n - (1 << i) + 1)).collect();
        Self { n, uf }
    }

    pub fn leader(&self, x: usize) -> usize {
        self.uf[0].leader(x)
    }

    pub fn size(&self, x: usize) -> usize {
        self.uf[0].size(x)
    }

    pub fn merge_range(
        &mut self,
        xs: impl RangeBounds<usize>,
        ys: impl RangeBounds<usize>,
    ) -> Vec<(usize, usize)> {
        let (a, b) = self.range_to_pair(xs);
        let (c, d) = self.range_to_pair(ys);
        assert!(b - a == d - c);
        let mut res = vec![];
        if a == c || b - a == 0 {
            return res;
        }
        let s = 63 - (b - a).leading_zeros() as usize;
        self.merge_range_(a, c, s, &mut res);
        self.merge_range_(b - (1 << s), d - (1 << s), s, &mut res);
        res
    }

    pub fn merge(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        self.merge_range(x..x + 1, y..y + 1).pop()
    }

    fn merge_range_(&mut self, l: usize, r: usize, k: usize, res: &mut Vec<(usize, usize)>) {
        let x = self.uf[k].leader(l);
        let y = self.uf[k].leader(r);
        if self.uf[k].merge(l, r) {
            if k == 0 {
                let z = self.uf[k].leader(l);
                res.push((z, x ^ y ^ z));
            } else {
                self.merge_range_(l, r, k - 1, res);
                self.merge_range_(l + (1 << k - 1), r + (1 << k - 1), k - 1, res);
            }
        }
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
