// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_vertices_and_unweighted_edges(&vec![(); n], &ab);
    let hld = HeavyLightDecomposition::new(&g);
    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
            i: usize,
        }
        let v = hld.jump(s, t, i);
        println!("{}", v as i64);
    }
}
