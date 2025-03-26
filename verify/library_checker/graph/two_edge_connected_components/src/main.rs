// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components

use graph::{GraphBuilder, UndirectedGraph};
use proconio::{fastout, input};
use two_edge_connected_components::TwoEdgeConnectedComponents;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let g = UndirectedGraph::from_edges(n, &ab);
    let tecc = TwoEdgeConnectedComponents::new(&g);

    println!("{}", tecc.groups.len());
    for g in &tecc.groups {
        print!("{}", g.len());
        for v in g {
            print!(" {}", v);
        }
        println!();
    }
}
