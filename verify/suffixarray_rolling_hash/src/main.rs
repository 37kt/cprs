// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use itertools::Itertools;
use proconio::{input, marker::Bytes};
use rolling_hash::RollingHashModInt61;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let rh = RollingHashModInt61::new(&s);
    let mut res = (0..s.len()).sorted_by(|i, j| rh.compare(i.., &rh, j..));
    println!("{}", res.join(" "));
}
