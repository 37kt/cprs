// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_quotients

use itertools::Itertools;
use proconio::input;
use quotients::quotients;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = quotients(n).collect::<Vec<_>>();
    v.reverse();
    println!("{}", v.len());
    println!("{}", v.iter().join(" "));
}
