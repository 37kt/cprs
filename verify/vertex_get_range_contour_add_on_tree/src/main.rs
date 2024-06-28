// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_get_range_contour_add_on_tree

use algebraic::{algebra, monoid};
use proconio::input;
use vertex_get_range_contour_add::VertexGetRangeContourAdd;

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
    let mut rcq = VertexGetRangeContourAdd::<M>::new(&a, &uv);
    for _ in 0..q {
        input! {
            t: usize,
            p: usize,
        }
        if t == 0 {
            input! {
                l: usize,
                r: usize,
                x: i64,
            }
            rcq.apply_range(p, l, r, x);
        } else {
            println!("{}", rcq.get(p));
        }
    }
}
