// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_plus_convolution_convex_convex

use minplus_convolution::minplus_convolution_convex_and_convex;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    }
    let c = minplus_convolution_convex_and_convex(&a, &b);
    for &x in &c {
        print!("{} ", x);
    }
    println!();
}
