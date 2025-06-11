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
        let mut n = 0;
        let mut v = vec![];
        let mut s = vec![];
        for x in iter.into_iter() {
            assert!(x == 0 || x == 1);
            if n % 64 == 0 {
                v.push(0);
                s.push(0);
            }
            let y = x as i32;
            v[n / 64] |= (y as u64) << (n % 64);
            s[n / 64] += y;
            n += 1;
        }
        if n % 64 == 0 {
            v.push(0);
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
            v: vec![0; (n + 1).ceil_div(64)],
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
        res += (self.v[i / 64] & ((1 << (i % 64)) - 1)).count_ones() as usize;
        res
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> usize {
        let (l, r) = range.as_half_open_range(0, self.n);
        let mut res = self.ft.fold(l / 64..r / 64) as usize;
        res += (self.v[r / 64] & ((1 << (r % 64)) - 1)).count_ones() as usize;
        res -= (self.v[l / 64] & ((1 << (l % 64)) - 1)).count_ones() as usize;
        res
    }
}
