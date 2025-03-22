// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::fastout;
use proconio::input;
use range_minimum_query::RangeMinimumQuery;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
    }
    let rmq = RangeMinimumQuery::from_iter(a);
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", rmq.min(l..r).unwrap());
    }
}
