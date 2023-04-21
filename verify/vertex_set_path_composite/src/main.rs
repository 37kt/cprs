// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use ac_library::{ModInt998244353 as Mint, Monoid};
use graph::Graph;
use proconio::input;
use tree_query::TreeQueryVertex;

enum M {}
impl Monoid for M {
    type S = (Mint, Mint);
    fn identity() -> Self::S {
        (1.into(), 0.into())
    }
    fn binary_operation(&(a, b): &Self::S, &(c, d): &Self::S) -> Self::S {
        (a * c, b * c + d)
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(Mint, Mint); n],
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
