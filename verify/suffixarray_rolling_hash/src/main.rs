// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

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
    let mut res = (0..s.len()).sorted_by(|i, j| rhs.compare(i.., &rhs, j..));
    println!("{}", res.join(" "));
}
