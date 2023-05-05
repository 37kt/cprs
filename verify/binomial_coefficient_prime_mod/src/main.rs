// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use combination::Combination;
use modint::DynamicModInt as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
        m: u32,
    }
    Mint::set_modulus(m);
    let comb = Combination::<Mint>::new();
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }
        println!("{}", comb.nck(n, k));
    }
}
