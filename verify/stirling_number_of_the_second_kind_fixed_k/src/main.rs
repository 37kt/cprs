// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind_fixed_k

use proconio::input;
use stirling_second_fixed_k::stirling_second_fixed_k;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut st = stirling_second_fixed_k::<998244353>(n, k);
    st.drain(..k);
    println!("{}", st);
}
