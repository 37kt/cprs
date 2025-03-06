// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_path_composite_sum

use graph::{GraphBuilder, UndirectedGraph};
use proconio::{fastout, input};
use rerooting_tree_dp::{RerootingTreeDp, TreeDpOperator};
use static_modint::ModInt998244353 as Mint;

enum Op {}
impl TreeDpOperator for Op {
    type Value = (Mint, Mint);
    type Vertex = Mint;
    type Edge = (Mint, Mint);

    fn unit() -> Self::Value {
        (0.into(), 0.into())
    }

    fn add_vertex(&(c, s): &Self::Value, v: &Self::Vertex) -> Self::Value {
        (c + 1, s + v)
    }

    fn add_edge(&(c, s): &Self::Value, &(a, b): &Self::Edge) -> Self::Value {
        (c, a * s + b * c)
    }

    fn rake(&(c1, s1): &Self::Value, &(c2, s2): &Self::Value) -> Self::Value {
        (c1 + c2, s1 + s2)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; n],
        edges: [(usize, usize, (Mint, Mint)); n - 1],
    }
    let g = UndirectedGraph::from_edges(n, edges);
    let dp = RerootingTreeDp::<Op>::with_vertices(&g, &a);
    for (_, x) in &dp {
        print!("{} ", x);
    }
    println!();
}
