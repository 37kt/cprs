// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_mod

use modint::DynamicModInt as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            y: u32,
            p: u32,
        }
        Mint::set_modulus(p);
        if let Some(res) = Mint::new(y).sqrt() {
            println!("{}", res);
        } else {
            println!("-1");
        }
    }
}
