use std::ops::Add;

use graph::Graph;

pub fn bellman_ford<T>(g: &Graph<(), T>, starts: &[usize], inf: T) -> Option<Vec<T>>
where
    T: Clone + PartialOrd + Add<Output = T> + Default,
{
    let n = g.size();
    let mut dist = vec![inf.clone(); n];
    for &s in starts {
        dist[s] = T::default();
    }
    for i in 0..n {
        for v in 0..n {
            for (u, w) in g.out_edges(v) {
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
