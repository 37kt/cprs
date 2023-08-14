// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use itertools::Itertools;
use modint::ModInt998244353 as Mint;
use or_convolution::or_convolution;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        mut a: [Mint; 1 << n],
        mut b: [Mint; 1 << n],
    }
    a.reverse();
    b.reverse();
    let c = or_convolution(a, b);
    println!("{}", c.iter().rev().join(" "));
}
