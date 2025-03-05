// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test

use prime_factorization::is_prime;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            n: usize,
        }
        if is_prime(n) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
