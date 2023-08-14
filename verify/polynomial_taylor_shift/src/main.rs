// verification-helper: PROBLEM https://judge.yosupo.jp/problem/polynomial_taylor_shift

use formal_power_series::FormalPowerSeries998244353 as FPS;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        c: Mint,
        a: [Mint; n],
    }
    let f: FPS = a.into();
    let f = f.taylor_shift(c);
    println!("{}", f);
}
