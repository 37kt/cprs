// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stirling_number_of_the_first_kind

use proconio::input;
use stirling_first::signed_stirling_first;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let st = signed_stirling_first::<998244353>(n);
    println!("{}", st);
}
