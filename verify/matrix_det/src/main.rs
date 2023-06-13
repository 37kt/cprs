// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_det

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
    let (_, _, det) = a.gauss_elimination();
    println!("{}", det.unwrap());
}
