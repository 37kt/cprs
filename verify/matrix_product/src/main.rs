// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product

use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use matrix::Matrix;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [[Mint; m]; n],
        b: [[Mint; k]; m],
    }
    let a = Matrix::<Mint>::from(a);
    let b = Matrix::<Mint>::from(b);
    let c = &a * &b;
    for i in 0..n {
        println!("{}", c[i].iter().join(" "));
    }
}
