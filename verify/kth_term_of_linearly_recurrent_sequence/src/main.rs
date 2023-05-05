// verification-helper: PROBLEM https://judge.yosupo.jp/problem/kth_term_of_linearly_recurrent_sequence

use bostan_mori::bostan_mori;
use formal_power_series::{fps, FormalPowerSeries};
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        d: usize,
        k: usize,
        a: [Mint; d],
        c: [Mint; d],
    }
    let a = FormalPowerSeries(a);
    let c = FormalPowerSeries(c);
    let c = fps![1] - (c << 1);
    let res = bostan_mori((&a * &c).pre(d), c, k);
    println!("{}", res);
}
