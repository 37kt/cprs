// verification-helper: PROBLEM https://judge.yosupo.jp/problem/pow_of_matrix

use itertools::Itertools;
use matrix::Matrix;
use modint::ModInt998244353 as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[Mint; n]; n],
    }
    let a = Matrix::<Mint>::from(a);
    let b = a.pow(k);
    for i in 0..n {
        println!("{}", b[i].iter().join(" "));
    }
}
