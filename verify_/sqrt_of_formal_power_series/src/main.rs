// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_of_formal_power_series

use formal_power_series::FormalPowerSeries;
use itertools::Itertools;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; n],
    }
    let a = FormalPowerSeries(a);
    if let Some(b) = a.sqrt(n) {
        println!("{}", b.iter().join(" "));
    } else {
        println!("-1");
    }
}
