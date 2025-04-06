// verification-helper: PROBLEM https://yukicoder.me/problems/no/119

use multivalued_optimization::MultivaluedOptimization;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        bc: [(i64, i64); n],
        m: usize,
        de: [(usize, usize); m],
    }

    let mut opt = MultivaluedOptimization::new(vec![3; n]);
    for (i, &(b, c)) in bc.iter().enumerate() {
        opt.add_unary(i, |mi| Some([-c, 0, -b][mi]));
    }
    for &(i, j) in &de {
        opt.add_binary(
            i,
            j,
            |mi, mj| {
                if mi == 2 && mj == 0 {
                    None
                } else {
                    Some(0)
                }
            },
        );
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
