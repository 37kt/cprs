// verification-helper: PROBLEM https://judge.yosupo.jp/problem/multipoint_evaluation

use formal_power_series::FormalPowerSeries998244353 as FPS;
use itertools::Itertools;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [Mint; n],
        p: [Mint; m],
    }
    let f: FPS = c.into();
    let ys = f.multipoint_evaluate(&p);
    println!("{}", ys.iter().join(" "));
}
