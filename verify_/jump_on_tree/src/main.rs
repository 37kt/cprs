// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_unweighted_edges(n, &ab);
    let hld = HeavyLightDecomposition::new(&g, 0);
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
