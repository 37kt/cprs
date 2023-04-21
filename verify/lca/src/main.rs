// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use graph::Graph;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut g = Graph::<(), ()>::new(n);
    for v in 1..n {
        input! {
            p: usize,
        }
        g.add_undirected_edge(v, p, ());
    }
    let hld = HeavyLightDecomposition::new(&g);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", hld.lca(u, v));
    }
}
