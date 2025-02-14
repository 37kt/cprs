// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_sat

use proconio::{fastout, input};
use two_satisfiability::TwoSatisfiability;

#[fastout]
fn main() {
    input! {
        _: String,
        _: String,
        n: usize,
        m: usize,
    }
    let mut ts = TwoSatisfiability::new(n);
    for _ in 0..m {
        input! {
            i: i64,
            j: i64,
            _: i64,
        }
        let (i, f) = if i < 0 {
            ((-i - 1) as usize, false)
        } else {
            ((i - 1) as usize, true)
        };
        let (j, g) = if j < 0 {
            ((-j - 1) as usize, false)
        } else {
            ((j - 1) as usize, true)
        };
        ts.or(i, f, j, g);
    }
    if let Some(res) = ts.solve() {
        println!("s SATISFIABLE");
        print!("v ");
        for i in 0..n {
            if !res[i] {
                print!("-");
            }
            print!("{} ", i + 1);
        }
        println!("0");
    } else {
        println!("s UNSATISFIABLE");
    }
}
