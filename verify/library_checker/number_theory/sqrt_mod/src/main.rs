// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_mod

use dynamic_modint::DefaultDynamicModInt;
use proconio::{fastout, input};

type Mint = DefaultDynamicModInt;

#[fastout]
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
        if let Some(x) = Mint::new(y).sqrt() {
            println!("{x}");
        } else {
            println!("-1");
        }
    }
}
