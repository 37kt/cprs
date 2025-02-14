// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use algebraic::{algebra, group};
use fenwick_tree::FenwickTree;
use itertools::Itertools;
use mo::Mo;
use proconio::input;
use superslice::Ext;

algebra!(G, i64);
group!(G, 0, |x, y| x + y, |x: &_| -x);

struct Solver {
    a: Vec<usize>,
    tr: FenwickTree<G>,
    res: i64,
}

impl Mo for Solver {
    type Output = i64;
    fn add_left(&mut self, i: usize) {
        self.res += self.tr.accum(self.a[i]);
        self.tr.add(self.a[i], 1);
    }
    fn remove_left(&mut self, i: usize) {
        self.res -= self.tr.accum(self.a[i]);
        self.tr.add(self.a[i], -1);
    }
    fn add_right(&mut self, i: usize) {
        self.res += self.tr.sum(self.a[i] + 1..);
        self.tr.add(self.a[i], 1);
    }
    fn remove_right(&mut self, i: usize) {
        self.res -= self.tr.sum(self.a[i] + 1..);
        self.tr.add(self.a[i], -1);
    }
    fn query(&self) -> Self::Output {
        self.res
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let mut z = a.clone();
    z.sort();
    z.dedup();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = z.lower_bound(&a[i]);
    }
    let mut solver = Solver {
        a: b,
        tr: FenwickTree::new(z.len()),
        res: 0,
    };
    let res = solver.solve(&lr);
    println!("{}", res.iter().join("\n"));
}
