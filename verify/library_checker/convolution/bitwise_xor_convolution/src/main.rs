// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_xor_convolution

use algebraic_structure::AddMulOperator;
use bitwise_xor_convolution::bitwise_xor_convolution;
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
    let inv2 = Mint::from(2).recip();
    let c = bitwise_xor_convolution::<AddMulOperator<_>>(&a, &b, |x| x * inv2);
    for i in 0..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
