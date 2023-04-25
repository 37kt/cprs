// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

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
    let mut g = Graph::from(a);
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
        }
        g.add_undirected_edge(u, v, ());
    }
    let mut tq = TreeQueryVertex::<M>::build(&g);
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                p: usize,
                x: i64,
            }
            let t = tq.get(p);
            tq.set(p, t + x);
        } else {
            input! {
                u: usize,
                v: usize,
            }
            let t = tq.prod_path(u, v);
            println!("{}", t);
        }
    }
}
