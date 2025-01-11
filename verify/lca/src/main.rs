// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut es = vec![];
    for v in 1..n {
        input! {
            p: usize,
        }
        es.push((p, v));
    }
    let g = UndirectedGraph::from_vertices_and_unweighted_edges(&vec![(); n], &es);
    let hld = HeavyLightDecomposition::new(&g);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", hld.lca(u, v));
    }
}
