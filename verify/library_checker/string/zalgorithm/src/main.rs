// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use proconio::{fastout, input, marker::Bytes};
use z_algorithm::z_algorithm;

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }

    let z = z_algorithm(&s);
    for &x in &z {
        print!("{} ", x);
    }
    println!();
}
