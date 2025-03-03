// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc

use graph::{DirectedGraph, GraphBuilder};
use proconio::{fastout, input};
use strongly_connected_components::StronglyConnectedComponents;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let g = DirectedGraph::from_edges(n, &ab);
    let scc = StronglyConnectedComponents::new(&g);
    let groups = scc.groups;

    println!("{}", groups.len());
    for g in &groups {
        print!("{}", g.len());
        for v in g {
            print!(" {}", v);
        }
        println!();
    }
}
