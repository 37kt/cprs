// verification-helper: PROBLEM https://judge.yosupo.jp/problem/compositional_inverse_of_formal_power_series_large

use compositional_inverse::compositional_inverse;
use formal_power_series::FormalPowerSeries;
use modint::ModInt998244353;
use proconio::fastout;
use proconio::input;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; n],
    }
    let f = FormalPowerSeries(a);
    let g = compositional_inverse(&f, n);
    println!("{}", g);
}
