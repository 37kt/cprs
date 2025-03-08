// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007

use convolution::convolution_arbitrary_mod;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt1000000007 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Mint; n],
        b: [Mint; m],
    }
    let c = convolution_arbitrary_mod(&a, &b);
    for i in 0..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
