// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stirling_number_of_the_first_kind_fixed_k

use proconio::input;
use stirling_first_fixed_k::signed_stirling_first_fixed_k;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut st = signed_stirling_first_fixed_k::<998244353>(n, k);
    st.drain(..k);
    println!("{}", st);
}
