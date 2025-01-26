// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bell_number

use bell_number::bell_number;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let b = bell_number::<998244353>(n);
    println!("{}", b);
}
