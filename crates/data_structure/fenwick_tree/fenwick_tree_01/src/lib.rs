// TODO: 二分探索

use std::ops::RangeBounds;

use algebraic_structure::magma::AddOperator;
use as_half_open_range::AsHalfOpenRange;
use fenwick_tree::FenwickTree;
use numeric_traits::Integer;

#[derive(Clone)]
pub struct FenwickTree01 {
    n: usize,
    ft: FenwickTree<AddOperator<i32>>,
    v: Vec<u64>,
}

impl FromIterator<usize> for FenwickTree01 {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let m = n.ceil_div(64);
        let mut v = vec![0; m];
        let mut s = vec![0; m];
        for (i, &x) in a.iter().enumerate() {
            assert!(x == 0 || x == 1);
            let y = x as i32;
            v[i / 64] |= (y as u64) << (i % 64);
            s[i / 64] += y;
        }
        Self {
            n,
            ft: FenwickTree::<AddOperator<i32>>::from_iter(s),
            v,
        }
    }
}

impl FenwickTree01 {
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> usize) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn new(n: usize) -> Self {
        Self {
            n,
            ft: FenwickTree::<AddOperator<i32>>::new(n.ceil_div(64)),
            v: vec![0; n.ceil_div(64)],
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn get(&self, i: usize) -> usize {
        assert!(i < self.n);
        (self.v[i / 64] >> (i % 64) & 1) as usize
    }

    pub fn set(&mut self, i: usize, x: usize) {
        assert!(i < self.n);
        assert!(x == 0 || x == 1);
        if self.get(i) == x {
            return;
        }
        if x == 1 {
            self.v[i / 64] |= 1 << (i % 64);
            self.ft.add(i / 64, 1);
        } else {
            self.v[i / 64] &= !(1 << (i % 64));
            self.ft.add(i / 64, -1);
        }
    }

    pub fn fold_prefix(&self, i: usize) -> usize {
        assert!(i <= self.n);
        let mut res = self.ft.fold_prefix(i / 64) as usize;
        if i % 64 > 0 {
            res += (self.v[i / 64] & ((1 << (i % 64)) - 1)).count_ones() as usize;
        }
        res
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> usize {
        let (l, r) = range.as_half_open_range(0, self.n);
        self.fold_prefix(r) - self.fold_prefix(l)
    }
}
