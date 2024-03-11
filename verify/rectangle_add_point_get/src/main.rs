// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_add_point_get

use algebraic::{algebra, monoid};
use dual_range_tree::DualRangeTree;
use proconio::input;

algebra!(F, usize);
monoid!(F, 0, |a, b| a + b);

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut qs = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            d: usize,
            r: usize,
            u: usize,
            w: usize,
        }
        qs.push((0, l, d, r, u, w));
    }
    let mut ps = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                l: usize,
                d: usize,
                r: usize,
                u: usize,
                w: usize,
            }
            qs.push((0, l, d, r, u, w));
        } else {
            input! {
                x: usize,
                y: usize,
            }
            qs.push((1, x, y, 0, 0, 0));
            ps.push((x, y));
        }
    }
    let mut seg = DualRangeTree::<usize, F>::new(ps);
    for (t, l, d, r, u, w) in qs {
        if t == 0 {
            seg.apply_range(l..r, d..u, w);
        } else {
            println!("{}", seg.get((l, d)));
        }
    }
}
