// verification-helper: PROBLEM https://judge.yosupo.jp/problem/log_of_formal_power_series

use ac_library::ModInt998244353 as Mint;
use formal_power_series::FPS;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; n],
    }
    let f = FPS(a);
    let g = f.log(n);
    println!("{}", g.iter().join(" "));
}
