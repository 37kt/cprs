// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use dijkstra::Dijkstra;
use graph::{DirectedGraph, GraphBuilder};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize, i64); m],
    }
    let g = DirectedGraph::from_edges(n, &edges);
    let res = Dijkstra::solve(&g, &[s]);
    let Some(x) = res.dist(t) else {
        println!("-1");
        return;
    };
    let path = res.path(t).unwrap();
    let y = path.len() - 1;
    println!("{} {}", x, y);
    for i in 0..y {
        println!("{} {}", path[i], path[i + 1]);
    }
}
