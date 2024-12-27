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
    let (groups, _) = strongly_connected_components(&g);
    println!("{}", groups.len());
    for g in groups {
        println!("{} {}", g.len(), g.iter().join(" "));
    }
}
