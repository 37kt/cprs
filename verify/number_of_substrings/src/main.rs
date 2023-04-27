// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings

use proconio::{input, marker::Bytes};
use suffix_array::SuffixArray;

#[proconio::fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let sa = SuffixArray::build(&s);
    let lcp = sa.lcp_array();
    let n = s.len();
    let res = n * (n + 1) / 2 - lcp.iter().sum::<usize>();
    println!("{}", res);
}
