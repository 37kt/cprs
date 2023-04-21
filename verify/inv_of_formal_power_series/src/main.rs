// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inv_of_formal_power_series

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
    let g = f.inv(n);
    println!("{}", g.iter().join(" "));
}
