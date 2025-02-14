// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use algebraic::{algebra, monoid};
use proconio::input;
use segment_tree::SegmentTree;

algebra!(M, i64);
monoid!(M, 0, |x, y| x + y);

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut seg = SegmentTree::<M>::from(a);
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                p: usize,
                x: i64,
            }
            let y = seg.get(p);
            seg.set(p, y + x);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", seg.prod(l..r));
        }
    }
}
