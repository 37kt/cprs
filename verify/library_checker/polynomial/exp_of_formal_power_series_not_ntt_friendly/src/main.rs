// verification-helper: PROBLEM https://judge.yosupo.jp/problem/exp_of_formal_power_series

use dynamic_modint::DefaultDynamicModInt as Mint;
use formal_power_series::FormalPowerSeries;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    Mint::set_modulus(998_244_353);
    input! {
        n: usize,
        a: [Mint; n],
    }
    let f: FormalPowerSeries<Mint> = a.into();
    let g = f.exp(n);
    for i in 0..n {
        print!("{} ", g[i]);
    }
    println!();
}
