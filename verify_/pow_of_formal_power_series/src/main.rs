// verification-helper: PROBLEM https://judge.yosupo.jp/problem/pow_of_formal_power_series

use formal_power_series::FormalPowerSeries;
use itertools::Itertools;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Mint; n],
    }
    let a = FormalPowerSeries(a);
    let b = a.pow(m, n);
    println!("{}", b.iter().join(" "));
}
