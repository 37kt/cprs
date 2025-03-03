// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use binomial::BinomialPrime;
use dynamic_modint::DefaultDynamicModInt;
use proconio::{fastout, input};

type Mint = DefaultDynamicModInt;

#[fastout]
fn main() {
    input! {
        t: usize,
        m: u32,
    }
    Mint::set_modulus(m);
    let mut binom = BinomialPrime::new();
    binom.expand(10_000_000);
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }
        let res: Mint = binom.nck(n, k);
        println!("{}", res);
    }
}
