// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use ac_library::ModInt998244353 as Mint;
use algebraic::{algebra, monoid};
use graph::Graph;
use proconio::input;
use tree_query::TreeQueryVertex;

algebra!(M, (Mint, Mint));
monoid!(M, (1.into(), 0.into()), |&(a, b), &(c, d)| (
    a * c,
    b * c + d
));

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(Mint, Mint); n],
        uv: [(usize, usize); n - 1],
    }
    let g = Graph::from_vertices_and_unweighted_undirected_edges(&a, &uv);
    let mut tq = TreeQueryVertex::<M>::build(&g);
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                p: usize,
                c: Mint,
                d: Mint,
            }
            tq.set(p, (c, d));
        } else {
            input! {
                u: usize,
                v: usize,
                x: Mint,
            }
            let (a, b) = tq.prod_path(u, v);
            println!("{}", a * x + b);
        }
    }
}
