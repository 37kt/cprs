use std::{cmp::Ordering, collections::BinaryHeap};

use csr_array::CsrArray;
use numeric_traits::{Inf, Numeric};

pub struct Dijkstra<T>
where
    T: Numeric + Inf,
{
    dist: Vec<T>,
    prev: Vec<u32>,
}

#[derive(PartialEq)]
struct State<T>
where
    T: Numeric + Inf,
{
    cost: T,
    v: u32,
}

impl<T> Eq for State<T> where T: Numeric + Inf {}

impl<T> Ord for State<T>
where
    T: Numeric + Inf,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialOrd for State<T>
where
    T: Numeric + Inf,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<T> Dijkstra<T>
where
    T: Numeric + Inf,
{
    pub fn solve<'a>(
        g: &CsrArray<(usize, T)>,
        starts: impl IntoIterator<Item = &'a usize>,
    ) -> Self {
        let n = g.len();
        let mut dist = vec![T::inf(); n];
        let mut prev = vec![!0; n];
        let mut pq = BinaryHeap::<State<T>>::new();
        for &s in starts {
            dist[s] = T::zero();
            pq.push(State {
                cost: T::zero(),
                v: s as u32,
            });
        }
        while let Some(State { cost, v }) = pq.pop() {
            let v = v as usize;
            if dist[v] < cost {
                continue;
            }
            for &(u, w) in &g[v] {
                assert!(w >= T::zero(), "Weight must be non-negative");

                let new_cost = cost + w;
                if dist[u] > new_cost {
                    dist[u] = new_cost;
                    prev[u] = v as u32;
                    pq.push(State {
                        cost: new_cost,
                        v: u as u32,
                    });
                }
            }
        }
        Self { dist, prev }
    }

    pub fn dist(&self, v: usize) -> Option<T> {
        if self.dist[v] >= T::inf() {
            None
        } else {
            Some(self.dist[v])
        }
    }

    pub fn path(&self, mut v: usize) -> Option<Vec<usize>> {
        if self.dist[v] >= T::inf() {
            return None;
        }
        let mut path = vec![v];
        while self.prev[v] != !0 {
            v = self.prev[v] as usize;
            path.push(v);
        }
        path.reverse();
        Some(path)
    }
}
