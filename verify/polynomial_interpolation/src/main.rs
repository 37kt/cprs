// verification-helper: PROBLEM https://judge.yosupo.jp/problem/polynomial_interpolation

use modint::ModInt998244353;
use polynomial_interpolation::polynomial_interpolation;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        x: [Mint; n],
        y: [Mint; n],
    }
    let f = polynomial_interpolation(&x, &y);
    println!("{}", f);
}
