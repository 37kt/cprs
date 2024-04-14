use std::{
    mem::swap,
    ops::{Bound, RangeBounds},
};

use algebraic::{algebra, monoid};
use modint61::ModInt61;
use random::Pcg64Fast;
use segment_tree::SegmentTree;

algebra!(M, (ModInt61, ModInt61));
monoid!(
    M,
    (0.into(), 1.into()),
    |&(a, b): &(ModInt61, ModInt61), &(c, d): &(ModInt61, ModInt61)| (a * d + c, b * d)
);

pub struct RangeUnionFind {
    n: usize,
    base: ModInt61,
    seg: SegmentTree<M>,
    data: Vec<i32>,
    next: Vec<usize>,
}

impl RangeUnionFind {
    pub fn new(n: usize) -> Self {
        let base = base();
        let seg = SegmentTree::from((0..n).map(|i| (i.into(), base)).collect::<Vec<_>>());
        Self {
            n,
            base,
            seg,
            data: vec![-1; n],
            next: vec![!0; n],
        }
    }

    pub fn leader(&self, x: usize) -> usize {
        if self.data[x] < 0 {
            x
        } else {
            self.data[x] as usize
        }
    }

    pub fn size(&self, x: usize) -> usize {
        -self.data[self.leader(x)] as usize
    }

    pub fn connected_components(&self, x: usize) -> Vec<usize> {
        let mut x = self.leader(x);
        let mut res = vec![x];
        while self.next[x] != !0 {
            x = self.next[x];
            res.push(x);
        }
        res
    }

    pub fn merge_range(&mut self, xs: impl RangeBounds<usize>, ys: impl RangeBounds<usize>) {
        let (mut a, b) = self.range_to_pair(xs);
        let (mut c, d) = self.range_to_pair(ys);
        assert!(b - a == d - c);
        loop {
            let mut ok = 0;
            let mut ng = b - a + 1;
            while ok + 1 < ng {
                let md = (ok + ng) / 2;
                if self.seg.prod(a..a + md) == self.seg.prod(c..c + md) {
                    ok = md;
                } else {
                    ng = md;
                }
            }
            if ok == b - a {
                break;
            }
            a += ok;
            c += ok;
            let mut x = self.leader(a);
            let mut y = self.leader(c);
            assert!(x != y);
            if self.size(x) < self.size(y) {
                swap(&mut x, &mut y);
            }
            while self.next[y] != !0 {
                let v = self.next[y];
                self.next[y] = self.next[v];
                self.seg.set(v, (x.into(), self.base));
                self.data[v] = x as i32;
                self.data[x] -= 1;
                self.next[v] = self.next[x];
                self.next[x] = v;
            }
            self.seg.set(y, (x.into(), self.base));
            self.data[y] = x as i32;
            self.data[x] -= 1;
            self.next[y] = self.next[x];
            self.next[x] = y;
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        self.merge_range(x..x + 1, y..y + 1);
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

fn base() -> ModInt61 {
    fn gen() -> ModInt61 {
        const FACTORS: [usize; 12] = [2, 3, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321];
        let mut rng = Pcg64Fast::default();
        loop {
            let x = ModInt61::new(rng.u64());
            if FACTORS
                .iter()
                .all(|&f| x.pow((ModInt61::modulus() as usize - 1) / f).val() > 1)
            {
                return x;
            }
        }
    }

    thread_local! {
        static BASE: ModInt61 = gen();
    }
    BASE.with(|base| *base)
}
