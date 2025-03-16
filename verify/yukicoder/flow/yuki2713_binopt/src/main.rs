// verification-helper: PROBLEM https://yukicoder.me/problems/no/2713

use binary_optimization::BinaryOptimization;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
        c: [[Usize1]; m],
    }

    let mut opt = BinaryOptimization::new(n);
    for i in 0..n {
        opt.add_1(i, |bi| [0, a[i]][bi]);
    }
    for i in 0..m {
        opt.add_all1(&c[i], -b[i]);
    }

    let (cost, choice) = opt.solve();
    let mut sum = 0;
    for i in 0..n {
        if choice[i] == 1 {
            sum += a[i];
        }
    }
    for i in 0..m {
        if c[i].iter().all(|&j| choice[j] == 1) {
            sum -= b[i];
        }
    }
    assert_eq!(cost, sum);

    println!("{}", -cost);
}
