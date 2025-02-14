// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_plus_convolution_convex_arbitrary

use itertools::Itertools;
use min_plus_convolution::min_plus_convolution_arbitrary_convex;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    }
    let c = min_plus_convolution_arbitrary_convex(&b, &a);
    println!("{}", c.iter().join(" "));
}
