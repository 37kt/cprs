// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_rank

use algebraic_structure::AddMulOperator;
use matrix::Matrix;
use proconio::{fastout, input};
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Mint; m]; n],
    }
    let a = Matrix::<AddMulOperator<_>>::from_fn(n, m, |i, j| a[i][j]);
    let rank = a.rank();
    println!("{}", rank);
}
