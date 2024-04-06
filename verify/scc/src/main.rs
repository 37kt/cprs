// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc

use graph::Graph;
use itertools::Itertools;
use proconio::input;
use strongly_connected_components::strongly_connected_components;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let g = Graph::from_unweighted_directed_edges(n, &ab);
    let h = strongly_connected_components(&g);
    println!("{}", h.len());
    for i in 0..h.len() {
        let vs = h.vertex(i);
        println!("{} {}", vs.len(), vs.iter().join(" "));
    }
}
