// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product

use algebraic_structure::AddMulOperator;
use matrix::Matrix;
use proconio::{fastout, input};
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [Mint; n * m],
        b: [Mint; m * k],
    }
    let a = Matrix::<AddMulOperator<_>>::from_fn(n, m, |i, j| a[i * m + j]);
    let b = Matrix::from_fn(m, k, |i, j| b[i * k + j]);
    let c = a * b;
    for i in 0..n {
        for j in 0..k {
            print!("{} ", c[i][j]);
        }
        println!();
    }
}
