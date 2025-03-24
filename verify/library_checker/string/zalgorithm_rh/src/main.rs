// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use proconio::{fastout, input, marker::Bytes};
use rolling_hash::RollingHashSequence;

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }

    let rh = RollingHashSequence::from_iter(s);
    for i in 0..rh.len() {
        print!("{} ", rh.lcp(.., &rh, i..));
    }
    println!();
}
