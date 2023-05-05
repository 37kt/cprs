// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_of_formal_power_series_sparse

use formal_power_series::SparseFormalPowerSeries;
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
    if let Some(b) = a.sqrt(n) {
        println!("{}", b);
    } else {
        println!("-1");
    }
}
