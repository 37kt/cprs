// verification-helper: PROBLEM https://judge.yosupo.jp/problem/division_of_polynomials

use formal_power_series::FormalPowerSeries;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        f: [Mint; n],
        g: [Mint; m],
    }
    let f = FormalPowerSeries(f);
    let g = FormalPowerSeries(g);
    let (q, r) = f.div_mod(&g);
    println!("{} {}", q.len(), r.len());
    println!("{}", q);
    println!("{}", r);
}
