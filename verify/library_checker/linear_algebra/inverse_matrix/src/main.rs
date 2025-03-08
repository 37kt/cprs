// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inverse_matrix

use algebraic_structure::AddMulOperator;
use matrix::Matrix;
use proconio::{fastout, input};
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[Mint; n]; n],
    }
    let a = Matrix::<AddMulOperator<_>>::from_fn(n, n, |i, j| a[i][j]);
    let Some(inv) = a.inv() else {
        println!("-1");
        return;
    };
    for i in 0..n {
        for j in 0..n {
            print!("{} ", inv[i][j]);
        }
        println!();
    }
}
