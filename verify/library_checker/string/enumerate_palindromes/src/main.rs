// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use manacher::manacher;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let n = s.len();
    let rad = manacher(&s);
    for &x in &rad[1..n * 2] {
        print!("{} ", x);
    }
    println!();
}
