// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_xor_convolution

use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;
use xor_convolution::xor_convolution;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; 1 << n],
        b: [Mint; 1 << n],
    }
    let c = xor_convolution(a, b);
    println!("{}", c.iter().join(" "));
}
