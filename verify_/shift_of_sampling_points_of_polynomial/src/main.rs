// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shift_of_sampling_points_of_polynomial

use itertools::Itertools;
use modint::ModInt998244353;
use proconio::input;
use shift_of_sampling_points::shift_of_sampling_points;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: Mint,
        ys: [Mint; n],
    }
    let res = shift_of_sampling_points(&ys, c, m);
    println!("{}", res.iter().join(" "));
}
