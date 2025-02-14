// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use proconio::input;
use static_range_inversions_query::StaticRangeInversionsQuery;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let riq = StaticRangeInversionsQuery::new(&a);
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", riq.inversions(l..r));
    }
}
