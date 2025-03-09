// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: u64,
            b: u64,
        }
        println!("{}", a + b);
    }
}
