// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use itertools::Itertools;
use proconio::{input, marker::Bytes};
use z_algorithm::z_algorithm;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let z = z_algorithm(&s);
    println!("{}", z.iter().join(" "));
}
