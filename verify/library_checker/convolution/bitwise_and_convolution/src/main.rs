// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use algebraic_structure::AddMulOperator;
use bitwise_and_convolution::bitwise_and_convolution;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; 1 << n],
        b: [Mint; 1 << n],
    }

    let c = bitwise_and_convolution::<AddMulOperator<_>>(&a, &b);
    for i in 0..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
