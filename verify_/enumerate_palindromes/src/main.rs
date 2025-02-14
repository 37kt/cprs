// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use itertools::Itertools;
use manacher::manacher;
use proconio::{input, marker::Bytes};

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let res = manacher(&s);
    println!("{}", res[1..res.len() - 1].iter().join(" "));
}
