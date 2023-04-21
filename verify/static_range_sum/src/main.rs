// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use itertools::Itertools;
use mo::Mo;
use proconio::input;

struct Solver {
    a: Vec<i64>,
    sum: i64,
}

impl Mo for Solver {
    type Output = i64;
    fn add(&mut self, i: usize) {
        self.sum += self.a[i];
    }
    fn remove(&mut self, i: usize) {
        self.sum -= self.a[i];
    }
    fn query(&self) -> Self::Output {
        self.sum
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
    let mut solver = Solver { a, sum: 0 };
    println!("{}", solver.solve(&lr).iter().join("\n"));
}
