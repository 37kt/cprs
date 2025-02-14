// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_rectangle_add_rectangle_sum

use modint::ModInt998244353 as Mint;
use proconio::input;
use static_rectangle_add_rectangle_sum::static_rectangle_add_rectangle_sum;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut add = vec![];
    let mut sum = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            d: usize,
            r: usize,
            u: usize,
            w: Mint,
        }
        add.push((l, r, d, u, w));
    }
    for _ in 0..q {
        input! {
            l: usize,
            d: usize,
            r: usize,
            u: usize,
        }
        sum.push((l, r, d, u));
    }
    let res = static_rectangle_add_rectangle_sum(&add, &sum);
    for r in res {
        println!("{}", r);
    }
}
