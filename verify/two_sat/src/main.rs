// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_sat

use proconio::input;
use two_satisfiability::TwoSatisfiability;

#[proconio::fastout]
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
            a: i64,
            b: i64,
            _: i64,
        }
        let a = if a < 0 { a } else { a - 1 } as usize;
        let b = if b < 0 { b } else { b - 1 } as usize;
        ts.add(a, b);
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
