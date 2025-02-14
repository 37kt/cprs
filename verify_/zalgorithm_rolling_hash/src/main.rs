// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use itertools::Itertools;
use proconio::{input, marker::Bytes};
use rolling_hash::RollingHash;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let rh = RollingHash::new();
    let rhs = rh.build_table(&s);
    let res = (0..s.len()).map(|i| rhs.lcp(.., &rhs, i..)).collect_vec();
    println!("{}", res.iter().join(" "));
}
