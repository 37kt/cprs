// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inverse_matrix

use ac_library::ModInt998244353 as Mint;
use matrix::Matrix;
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
