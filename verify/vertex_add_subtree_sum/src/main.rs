// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use algebraic::{algebra, monoid};
use graph::Graph;
use proconio::input;
use tree_query::TreeQueryVertex;

algebra!(M, i64);
monoid!(M, 0, |x, y| x + y);

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut es = vec![];
    for v in 1..n {
        input! {
            p: usize,
        }
        es.push((p, v));
    }
    let g = Graph::from_vertices_and_unweighted_undirected_edges(&a, &es);
    let mut tq = TreeQueryVertex::<M>::build(&g);
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                p: usize,
                x: i64
            }
            let t = tq.get(p);
            tq.set(p, t + x);
        } else {
            input! {
                v: usize,
            }
            let t = tq.prod_subtree(v);
            println!("{}", t);
        }
    }
}
