// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components

use graph::UndirectedGraph;
use itertools::Itertools;
use proconio::input;
use two_edge_connected_components::two_edge_connected_components;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let g = UndirectedGraph::from_unweighted_edges(n, &ab);
    let (groups, _) = two_edge_connected_components(&g);
    println!("{}", groups.len());
    for group in groups {
        println!("{} {}", group.len(), group.iter().join(" "));
    }
}
