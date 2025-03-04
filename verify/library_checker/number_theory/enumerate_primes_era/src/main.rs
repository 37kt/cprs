// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes

use eratosthenes::Eratosthenes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let sieve = Eratosthenes::new(n);
    let primes = sieve.primes().collect::<Vec<_>>();
    let pi = primes.len();
    let res = primes
        .iter()
        .skip(b)
        .step_by(a)
        .copied()
        .collect::<Vec<_>>();
    println!("{} {}", pi, res.len());
    for i in 0..res.len() {
        print!("{}", res[i]);
        if i < res.len() - 1 {
            print!(" ");
        } else {
            println!();
        }
    }
}
