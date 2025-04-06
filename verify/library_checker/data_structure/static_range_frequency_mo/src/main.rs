// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use coordinate_compression::CoordinateCompression;
use mo::Mo;
use proconio::fastout;
use proconio::input;

struct Solver<'a> {
    a: &'a [usize],
    cnt: Vec<usize>,
}

impl<'a> Mo for Solver<'a> {
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

    let xs = a
        .iter()
        .copied()
        .chain(lrx.iter().map(|&(_, _, x)| x))
        .collect::<Vec<_>>();
    let (cc, xs) = CoordinateCompression::<_>::new(xs);
    let (a, x) = xs.split_at(n);
    for i in 0..q {
        lrx[i].2 = x[i];
    }

    let m = cc.len();

    let mut solver = Solver { a, cnt: vec![0; m] };
    let res = solver.solve(&lrx);
    for x in res {
        println!("{}", x);
    }
}
