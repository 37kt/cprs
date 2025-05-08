// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_range_contour_sum_on_tree

use algebraic_structure::magma::AddOperator;
use fenwick_tree::FenwickTree;
use graph::{GraphBuilder, UndirectedGraph};
use proconio::{fastout, input};
use tree_contour_range::TreeContourRange;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        uv: [(usize, usize); n - 1],
    }

    let g = UndirectedGraph::from_edges(n, &uv);
    let cr = TreeContourRange::new(&g);
    let mut seg = FenwickTree::<AddOperator<i64>>::new(cr.len());
    for (v, &x) in a.iter().enumerate() {
        for i in cr.point(v) {
            seg.add(i, x);
        }
    }
    for _ in 0..q {
        input! {
            t: usize,
            p: usize,
        }
        if t == 0 {
            input! {
                x: i64,
            }
            for i in cr.point(p) {
                seg.add(i, x);
            }
            a[p] += x;
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let mut res = cr.range(p, l..r).map(|range| seg.fold(range)).sum::<i64>();
            if l == 0 && 0 < r {
                res += a[p];
            }
            if n == 2 {
                let q = p ^ 1;
                if l <= 1 && 1 < r {
                    res += a[q];
                }
            }
            println!("{}", res);
        }
    }
}
