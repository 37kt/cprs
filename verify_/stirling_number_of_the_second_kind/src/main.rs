// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind

use proconio::input;
use stirling_second::stirling_second;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let st = stirling_second::<998244353>(n);
    println!("{}", st);
}
