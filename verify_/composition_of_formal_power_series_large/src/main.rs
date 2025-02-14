// verification-helper: PROBLEM https://judge.yosupo.jp/problem/composition_of_formal_power_series_large

use composition::composition;
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
        b: [Mint; n],
    }
    let f = FormalPowerSeries(a);
    let g = FormalPowerSeries(b);
    let h = composition(&f, &g, n);
    println!("{}", h);
}
