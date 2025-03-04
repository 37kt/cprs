// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lcm_convolution

use algebraic_structure::AddMulOperator;
use lcm_convolution::lcm_convolution;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [Mint; n],
        mut b: [Mint; n],
    }
    a.insert(0, 0.into());
    b.insert(0, 0.into());
    let c = lcm_convolution::<AddMulOperator<_>>(&a, &b);
    for i in 1..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
