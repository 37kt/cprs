// verification-helper: PROBLEM https://yukicoder.me/problems/no/119

use multivalued_optimization::MultivaluedOptimization;
use proconio::{fastout, input};

const INF: i64 = 1 << 20;

#[fastout]
fn main() {
    input! {
        n: usize,
        bc: [(i64, i64); n],
        m: usize,
        de: [(usize, usize); m],
    }

    let mut opt = MultivaluedOptimization::new(vec![3; n]);
    for i in 0..n {
        let (b, c) = bc[i];
        opt.add_1(i, |mi| [-c, 0, -b][mi]);
    }
    for &(i, j) in &de {
        opt.add_2(i, j, |mi, mj| if mi == 2 && mj == 0 { INF } else { 0 });
    }

    let (cost, choice) = opt.solve();

    let mut sum = 0;
    for i in 0..n {
        let (b, c) = bc[i];
        sum += [-c, 0, -b][choice[i]];
    }
    assert_eq!(cost, sum);
    for &(i, j) in &de {
        assert!(!(choice[i] == 2 && choice[j] == 0));
    }

    println!("{}", -cost);
}
