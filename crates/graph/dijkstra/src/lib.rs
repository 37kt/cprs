use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

use graph::Graph;

pub struct DijkstraResult<T>
where
    T: Clone + Ord + Add<Output = T> + Default,
{
    pub dist: Vec<T>,
    pub prev: Vec<usize>,
}

/// ダイクストラ法  
/// 始点集合からの最短距離を求める。
pub fn dijkstra<V, T>(g: &Graph<V, T>, starts: &[usize], inf: T) -> DijkstraResult<T>
where
    V: Clone,
    T: Clone + Ord + Add<Output = T> + Default,
{
    assert!(starts.len() > 0);
    let mut dist = vec![inf.clone(); g.len()];
    let mut prev = vec![!0; g.len()];
    let mut pq = BinaryHeap::new();
    for &s in starts {
        dist[s] = T::default();
        pq.push(Reverse((T::default(), s)));
    }
    while let Some(Reverse((s, v))) = pq.pop() {
        if dist[v] < s {
            continue;
        }
        for (u, w) in &g[v] {
            assert!(w.clone() >= T::default());
            if dist[*u] > dist[v].clone() + w.clone() {
                dist[*u] = dist[v].clone() + w.clone();
                prev[*u] = v;
                pq.push(Reverse((dist[*u].clone(), *u)));
            }
        }
    }
    DijkstraResult { dist, prev }
}

impl<T> DijkstraResult<T>
where
    T: Clone + Ord + Add<Output = T> + Default,
{
    /// 終点 v までの最短経路を求める。
    /// 終点に到達できない場合は None を返す。
    pub fn path(&self, mut v: usize) -> Option<Vec<usize>> {
        if self.dist[v].clone() != T::default() && self.prev[v] == !0 {
            return None;
        }
        let mut path = vec![];
        while v != !0 {
            path.push(v);
            v = self.prev[v];
        }
        path.reverse();
        Some(path)
    }
}
