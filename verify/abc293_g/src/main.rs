// verification-helper: PROBLEM https://atcoder.jp/contests/abc293/tasks/abc293_g

use itertools::Itertools;
use mo::Mo;
use proconio::{input, marker::Usize1};

fn f(x: usize) -> usize {
    if x < 2 {
        0
    } else {
        x * (x - 1) * (x - 2) / 6
    }
}

struct Solver {
    a: Vec<usize>,
    cnt: Vec<usize>,
    sum: usize,
}

impl Mo for Solver {
    type Output = usize;
    fn add(&mut self, i: usize) {
        self.sum -= f(self.cnt[self.a[i]]);
        self.cnt[self.a[i]] += 1;
        self.sum += f(self.cnt[self.a[i]]);
    }
    fn remove(&mut self, i: usize) {
        self.sum -= f(self.cnt[self.a[i]]);
        self.cnt[self.a[i]] -= 1;
        self.sum += f(self.cnt[self.a[i]]);
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
        a: [Usize1; n],
        lr: [(Usize1, usize); q],
    }
    let mut solver = Solver {
        a,
        cnt: vec![0; 200_000],
        sum: 0,
    };
    println!("{}", solver.solve(&lr).iter().join("\n"));
}
