// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_det

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
    let det = a.det().unwrap();
    println!("{}", det);
}
