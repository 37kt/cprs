// verification-helper: PROBLEM https://judge.yosupo.jp/problem/subset_convolution

use itertools::Itertools;
use modint::ModInt998244353 as Mint;
use proconio::input;
use subset_convolution::subset_convolution;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; 1 << n],
        b: [Mint; 1 << n],
    }
    let c = subset_convolution(&a, &b);
    println!("{}", c.iter().join(" "));
}
