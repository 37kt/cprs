// verification-helper: PROBLEM https://judge.yosupo.jp/problem/find_linear_recurrence

use berlekamp_massey::berlekamp_massey;
use modint::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; n],
    }
    let c = berlekamp_massey(&a);
    println!("{}", c.len() - 1);
    for i in 1..c.len() {
        print!("{} ", -c[i]);
    }
    println!();
}
