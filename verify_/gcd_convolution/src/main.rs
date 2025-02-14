// verification-helper: PROBLEM https://judge.yosupo.jp/problem/gcd_convolution

use ac_library::ModInt998244353 as Mint;
use gcd_convolution::gcd_convolution;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        mut a: [Mint; n],
        mut b: [Mint; n],
    }
    a.insert(0, 0.into());
    b.insert(0, 0.into());
    let c = gcd_convolution(a, b);
    println!("{}", c[1..].iter().join(" "));
}
