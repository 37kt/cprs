// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use dijkstra::dijkstra;
use graph::DirectedGraph;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        abc: [(usize, usize, i64); m],
    }
    let g = DirectedGraph::from_edges(n, &abc);
    let dijkstra_result = dijkstra(&g, &[s], 1 << 60);
    if let Some(path) = dijkstra_result.path(t) {
        println!("{} {}", dijkstra_result.dist[t], path.len() - 1);
        for i in 0..path.len() - 1 {
            println!("{} {}", path[i], path[i + 1]);
        }
    } else {
        println!("-1");
    }
}
