// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod

use convolution_ntt_friendly::convolution_ntt_friendly;
use itertools::Itertools;
use modint::ModInt998244353 as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [Mint; n],
        mut b: [Mint; m],
    }
    let c = convolution_ntt_friendly(a, b);
    println!("{}", c.iter().join(" "));
}
