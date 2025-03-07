// verification-helper: PROBLEM https://judge.yosupo.jp/problem/pow_of_matrix

use algebraic_structure::AddMulOperator;
use matrix::Matrix;
use proconio::{fastout, input};
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Mint; n * n],
    }
    let a = Matrix::<AddMulOperator<_>>::from_fn(n, n, |i, j| a[i * n + j]);
    let b = a.pow(k);
    for i in 0..n {
        for j in 0..n {
            print!("{} ", b[i][j]);
        }
        println!();
    }
}
