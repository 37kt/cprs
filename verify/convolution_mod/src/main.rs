// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod

use convolution_ntt_friendly::convolution_ntt_friendly;
use modint::ModInt998244353 as Mint;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [Mint; n],
        mut b: [Mint; m],
    }
    let c = convolution_ntt_friendly(a, b);
    for i in 0..c.len() {
        print!("{}", c[i]);
        if i < c.len() - 1 {
            print!(" ");
        } else {
            println!();
        }
    }
}
