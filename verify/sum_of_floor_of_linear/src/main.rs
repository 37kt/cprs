// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear

use floor_sum::floor_sum;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
            m: u64,
            a: i64,
            b: i64,
        }
        println!("{}", floor_sum(n, m, a, b));
    }
}
