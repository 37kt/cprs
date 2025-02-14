// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_range_contour_sum_on_tree

use algebraic::{algebra, monoid};
use proconio::input;
use vertex_add_range_contour_sum::VertexAddRangeContourSum;

algebra!(M, i64);
monoid!(M, 0, |x, y| x + y);

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        uv: [(usize, usize); n - 1],
    }
    let mut rcq = VertexAddRangeContourSum::<M>::new(&a, &uv);
    for _ in 0..q {
        input! {
            t: usize,
            p: usize,
        }
        if t == 0 {
            input! {
                x: i64,
            }
            rcq.add(p, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", rcq.prod(p, l, r));
        }
    }
}
