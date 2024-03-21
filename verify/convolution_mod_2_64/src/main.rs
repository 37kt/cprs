// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_2_64

use convolution_u64::convolution_u64;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m],
    }
    let c = convolution_u64(&a, &b);
    println!("{}", c.iter().join(" "));
}
