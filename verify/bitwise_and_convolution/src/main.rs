// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use ac_library::ModInt998244353 as Mint;
use and_convolution::and_convolution;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; 1 << n],
        b: [Mint; 1 << n],
    }
    let c = and_convolution(a, b);
    println!("{}", c.iter().join(" "));
}
