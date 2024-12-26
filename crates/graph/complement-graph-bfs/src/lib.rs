use std::collections::VecDeque;

use graph::Graph;

const INF: usize = 1 << 60;

/// 補グラフ上の最短距離を求める。  
/// start から到達できない場合は 2^60 を返す。
pub fn complement_graph_bfs(g: &Graph<(), ()>, start: usize) -> Vec<usize> {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut q = VecDeque::new();
    dist[start] = 0;
    q.push_back(start);
    let mut s = (0..start).chain(start + 1..n).collect();
    let mut f = vec![false; n];
    while let Some(v) = q.pop_front() {
        let mut l = vec![];
        for &(u, _) in &g[v] {
            f[u] = true;
        }
        for &u in &s {
            if f[u] {
                l.push(u);
            } else {
                dist[u] = dist[v] + 1;
                q.push_back(u);
            }
        }
        for &(u, _) in &g[v] {
            f[u] = false;
        }
        s = l;
    }
    dist
}
