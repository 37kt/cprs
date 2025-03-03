// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use mo::Mo;
use proconio::fastout;
use proconio::input;

struct Solver {
    a: Vec<usize>,
    cnt: Vec<usize>,
}

impl Mo for Solver {
    type Arg = usize;
    type Output = usize;

    fn add(&mut self, i: usize) {
        self.cnt[self.a[i]] += 1;
    }

    fn remove(&mut self, i: usize) {
        self.cnt[self.a[i]] -= 1;
    }

    fn query(&mut self, &query: &Self::Arg) -> Self::Output {
        self.cnt[query]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        mut lrx: [(usize, usize, usize); q],
    }

    let mut xs = lrx.iter().map(|&(_, _, x)| x).collect::<Vec<_>>();
    xs.sort_unstable();
    xs.dedup();
    let m = xs.len() + 1;
    for x in &mut a {
        *x = xs.binary_search(x).unwrap_or(m - 1);
    }
    for (_, _, x) in &mut lrx {
        *x = xs.binary_search(x).unwrap();
    }

    let mut solver = Solver { a, cnt: vec![0; m] };
    let res = solver.solve(&lrx);
    for x in res {
        println!("{}", x);
    }
}
