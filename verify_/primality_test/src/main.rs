// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test

use fast_factorize::is_prime;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            n: u64,
        }
        println!("{}", if is_prime(n) { "Yes" } else { "No" });
    }
}
