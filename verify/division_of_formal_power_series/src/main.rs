// verification-helper: PROBLEM https://judge.yosupo.jp/problem/division_of_formal_power_series

use ac_library::ModInt998244353 as Mint;
use formal_power_series::FPS;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        f: [Mint; n],
        g: [Mint; m],
    }
    let f = FPS(f);
    let g = FPS(g);
    let (q, r) = f.divmod(&g);
    println!("{} {}", q.len(), r.len());
    println!("{}", q.iter().join(" "));
    println!("{}", r.iter().join(" "));
}
