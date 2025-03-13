// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min

use proconio::fastout;
use proconio::input;

use offline_dynamic_convex_hull_trick::OfflineDynamicConvexHullTrick;
use proconio::read_value;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut cht = OfflineDynamicConvexHullTrick::new();
    for i in 0..n + q {
        if i < n || read_value!(usize) == 0 {
            input! {
                a: i64,
                b: i64,
            }
            cht.add_line(a, b);
        } else {
            input! {
                x: i64,
            }
            cht.min(x);
        }
    }
    for y in cht.solve() {
        println!("{}", y);
    }
}
