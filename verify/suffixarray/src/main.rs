// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use itertools::Itertools;
use proconio::{input, marker::Bytes};
use suffix_array::SuffixArray;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let sa = SuffixArray::build(&s);
    println!("{}", sa.suffix_array().iter().join(" "));
}
