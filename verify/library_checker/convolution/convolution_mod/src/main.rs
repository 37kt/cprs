// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod

use convolution::convolution_ntt_friendly;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Mint; n],
        b: [Mint; m],
    }
    let c = convolution_ntt_friendly(&a, &b);
    for i in 0..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
