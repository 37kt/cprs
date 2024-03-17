// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inverse_matrix

use matrix::Matrix;
use modint::ModInt998244353 as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [[Mint; n]; n],
    }
    let a = Matrix::<Mint>::from(a);
    if let Some(b) = a.inv() {
        println!("{}", b);
    } else {
        println!("-1");
    }
}
