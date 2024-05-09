// verification-helper: PROBLEM https://judge.yosupo.jp/problem/wildcard_pattern_matching

use itertools::Itertools;
use proconio::{input, marker::Bytes};
use wildcard_pattern_matching::wildcard_pattern_matching;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }
    let res = wildcard_pattern_matching(&s, &t, b'*');
    println!("{}", res.iter().map(|&f| f as u8).join(""));
}
