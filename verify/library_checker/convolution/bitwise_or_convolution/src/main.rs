// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use algebraic_structure::AddMulOperator;
use bitwise_or_convolution::bitwise_or_convolution;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [Mint; 1 << n],
        mut b: [Mint; 1 << n],
    }
    a.reverse();
    b.reverse();
    let mut c = bitwise_or_convolution::<AddMulOperator<_>>(&a, &b);
    c.reverse();
    for i in 0..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
