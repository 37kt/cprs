// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_2_64

use convolution::convolution_mod_2_64;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m],
    }
    let c = convolution_mod_2_64(&a, &b);
    for i in 0..c.len() {
        print!("{}", c[i]);
        print!("{}", if i == c.len() - 1 { "\n" } else { " " });
    }
}
