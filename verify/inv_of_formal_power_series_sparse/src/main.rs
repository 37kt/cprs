// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inv_of_formal_power_series_sparse

use formal_power_series::SparseFormalPowerSeries;
use itertools::Itertools;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ia: [(usize, Mint); k],
    }
    let a = SparseFormalPowerSeries(ia);
    let b = a.inv(n);
    println!("{}", b.iter().join(" "));
}
