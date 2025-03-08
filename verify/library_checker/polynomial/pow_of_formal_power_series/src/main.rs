// verification-helper: PROBLEM https://judge.yosupo.jp/problem/pow_of_formal_power_series

use formal_power_series::FormalPowerSeries;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        exp: usize,
        a: [Mint; n],
    }
    let f: FormalPowerSeries<Mint> = a.into();
    let g = f.pow(exp, n);
    for i in 0..n {
        print!("{} ", g[i]);
    }
    println!();
}
