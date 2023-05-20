// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use itertools::Itertools;
use proconio::{input, marker::Bytes};
use rolling_hash::RollingHashModInt61;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let rh = RollingHashModInt61::new(&s);
    let res = (0..s.len()).map(|i| rh.lcp(.., &rh, i..)).collect_vec();
    println!("{}", res.iter().join(" "));
}
