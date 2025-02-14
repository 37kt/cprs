use std::ops::Add;

use graph::Graph;

/// ベルマンフォード法  
/// 始点集合からの最短距離を求める。  
/// 負の閉路が存在する場合は None を返す。
pub fn bellman_ford<T, const DIRECTED: bool>(
    g: &Graph<(), T, DIRECTED>,
    starts: &[usize],
    inf: T,
) -> Option<Vec<T>>
where
    T: Clone + PartialOrd + Add<Output = T> + Default,
{
    let n = g.len();
    let mut dist = vec![inf.clone(); n];
    for &s in starts {
        dist[s] = T::default();
    }
    for i in 0..n {
        for v in 0..n {
            for (u, w) in &g[v] {
                if dist[*u] > dist[v].clone() + w.clone() {
                    if i == n - 1 {
                        return None;
                    }
                    dist[*u] = dist[v].clone() + w.clone();
                }
            }
        }
    }
    Some(dist)
}
