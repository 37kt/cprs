// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007

use convolution_arbitrary_mod::convolution_arbitrary_mod;
use itertools::Itertools;
use modint::ModInt1000000007 as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [Mint; n],
        mut b: [Mint; m],
    }
    let c = convolution_arbitrary_mod(&a, &b);
    println!("{}", c.iter().join(" "));
}
