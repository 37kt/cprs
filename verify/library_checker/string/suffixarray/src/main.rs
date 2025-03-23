// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use proconio::{fastout, input, marker::Bytes};
use suffix_array::SuffixArray;

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }

    let sa = SuffixArray::new(&s);
    for &x in sa.suffix_array() {
        print!("{} ", x);
    }
    println!();
}
