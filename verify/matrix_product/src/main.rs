// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product

use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use matrix::{Element, Matrix};
use proconio::input;

#[derive(Clone)]
enum E {}
impl Element for E {
    type S = Mint;
    fn zero() -> Self::S {
        0.into()
    }
    fn one() -> Self::S {
        1.into()
    }
    fn add(a: Self::S, b: Self::S) -> Self::S {
        a + b
    }
    fn mul(a: Self::S, b: Self::S) -> Self::S {
        a * b
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [[Mint; m]; n],
        b: [[Mint; k]; m],
    }
    let a = Matrix::<E>::from(a);
    let b = Matrix::<E>::from(b);
    let c = &a * &b;
    for i in 0..n {
        println!("{}", c[i].iter().join(" "));
    }
}
