// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use prime_factorization::prime_factors;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            n: u64,
        }
        let ps = prime_factors(n);
        print!("{}", ps.len());
        for p in ps {
            print!(" {}", p);
        }
        println!();
    }
}
