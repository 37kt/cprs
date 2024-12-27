use std::{collections::VecDeque, ops::Add};

use graph::Graph;

pub struct ZeroOneBFSResult<T>
where
    T: Clone + Ord + Add<Output = T> + Default,
{
    pub dist: Vec<T>,
    pub prev: Vec<usize>,
}

/// 0-1 BFS  
/// 辺の重みが 0 か 1 のグラフ上で、始点から各頂点への最短距離を求める
///
/// # 戻り値
///
/// ZeroOneBFSResult
/// - dist: 始点から各頂点への最短距離
/// - prev: 始点から各頂点への最短経路における前の頂点
pub fn zero_one_bfs<V, T>(g: &Graph<V, T>, starts: &[usize], inf: T) -> ZeroOneBFSResult<T>
where
    V: Clone,
    T: Clone + Ord + Add<Output = T> + Default + From<u8>,
{
    assert!(starts.len() > 0);
    let zero = T::from(0);
    let one = T::from(1);
    let mut dist = vec![inf.clone(); g.len()];
    let mut prev = vec![!0; g.len()];
    let mut dq = VecDeque::new();
    for &s in starts {
        dist[s] = T::default();
        dq.push_back((zero.clone(), s));
    }
    while let Some((s, v)) = dq.pop_front() {
        if dist[v] < s {
            continue;
        }
        for (u, w) in &g[v] {
            assert!(*w == zero || *w == one);
            let t = dist[v].clone() + w.clone();
            if dist[*u] > t {
                dist[*u] = t.clone();
                prev[*u] = v;
                if *w == zero {
                    dq.push_front((t.clone(), *u));
                } else {
                    dq.push_back((t.clone(), *u));
                }
            }
        }
    }
    ZeroOneBFSResult { dist, prev }
}

impl<T> ZeroOneBFSResult<T>
where
    T: Clone + Ord + Add<Output = T> + Default,
{
    /// 始点から頂点 v への最短経路を求める  
    /// 経路が存在しない場合は None を返す
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
