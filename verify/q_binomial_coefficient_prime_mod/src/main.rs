// verification-helper: PROBLEM https://judge.yosupo.jp/problem/q_binomial_coefficient_prime_mod

use modint::DynamicModInt as Mint;
use proconio::input;
use q_binomial::QBinomial;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
        m: u32,
    }
    Mint::set_modulus(m);
    input! {
        q: Mint,
    }
    let comb = QBinomial::new(q);
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }
        println!("{}", comb.binom(n, k));
    }
}
