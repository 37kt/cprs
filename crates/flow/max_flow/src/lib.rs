use std::iter::FusedIterator;

use numeric_traits::{Inf, Integer};

mod internal;

/// dinic + scaling
#[derive(Clone)]
pub struct MaxFlow {
    edges: Vec<internal::Edge>,
    head: Vec<usize>,
    next: Vec<usize>,
    dist: Vec<i32>,
    iter: Vec<usize>,
    queue: internal::Queue,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub cap: i64,
    pub flow: i64,
}

impl MaxFlow {
    pub fn new() -> Self {
        Self {
            edges: vec![],
            head: vec![],
            next: vec![],
            dist: vec![],
            iter: vec![],
            queue: internal::Queue::new(),
        }
    }

    pub fn count_vertices(&self) -> usize {
        self.head.len()
    }

    pub fn add_vertex(&mut self) -> usize {
        self.head.push(!0);
        self.head.len() - 1
    }

    pub fn add_vertices(
        &mut self,
        n: usize,
    ) -> impl Iterator<Item = usize> + DoubleEndedIterator + ExactSizeIterator + FusedIterator {
        self.head.resize(self.head.len() + n, !0);
        self.head.len() - n..self.head.len()
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, cap: i64) -> usize {
        assert!(src < self.count_vertices());
        assert!(dst < self.count_vertices());
        assert!(cap >= 0);
        let m = self.edges.len();
        self.next.push(self.head[src]);
        self.head[src] = m;
        self.next.push(self.head[dst]);
        self.head[dst] = m + 1;
        self.edges.push(internal::Edge { dst, cap });
        self.edges.push(internal::Edge { dst: src, cap: 0 });
        m / 2
    }

    pub fn edges(
        &self,
    ) -> impl Iterator<Item = Edge> + DoubleEndedIterator + ExactSizeIterator + FusedIterator + '_
    {
        self.edges.windows(2).map(|e| Edge {
            src: e[1].dst,
            dst: e[0].dst,
            cap: e[0].cap + e[1].cap,
            flow: e[1].cap,
        })
    }

    pub fn max_flow(&mut self, src: usize, dst: usize) -> i64 {
        assert!(src != dst);
        assert!(src < self.count_vertices());
        assert!(dst < self.count_vertices());

        let max_cap = self.edges.iter().map(|e| e.cap).max().unwrap_or(0);
        if max_cap == 0 {
            return 0;
        }

        self.iter.resize(self.count_vertices(), 0);
        self.dist.resize(self.count_vertices(), 0);

        let mut flow = 0;
        for f in (0..=max_cap.floor_log2()).map(|i| 1 << i).rev() {
            while self.build_augmenting_path(src, dst, f) {
                self.iter.copy_from_slice(&self.head);
                flow += self.find_augmenting_path(src, dst, f, i64::inf());
            }
        }
        flow
    }

    /// 頂点 src から到達可能かを返す  
    /// max_flow の後に呼び出す
    pub fn min_cut(&mut self, src: usize) -> Vec<bool> {
        let mut vis = vec![false; self.count_vertices()];
        self.queue.clear();
        self.queue.set_capacity(self.count_vertices());
        self.queue.push(src);
        while let Some(v) = self.queue.pop() {
            let mut ei = self.head[v];
            while ei != !0 {
                let e = self.edges[ei];
                if e.cap > 0 && !vis[e.dst] {
                    vis[e.dst] = true;
                    self.queue.push(e.dst);
                }
                ei = self.next[ei];
            }
        }
        vis
    }

    fn build_augmenting_path(&mut self, src: usize, dst: usize, base: i64) -> bool {
        self.dist.fill(i32::inf());
        self.queue.clear();
        self.queue.set_capacity(self.count_vertices());
        self.dist[src] = 0;
        self.queue.push(src);
        while let Some(v) = self.queue.pop() {
            let mut ei = self.head[v];
            while ei != !0 {
                let e = self.edges[ei];
                if e.cap >= base && self.dist[e.dst] == i32::inf() {
                    self.dist[e.dst] = self.dist[v] + 1;
                    self.queue.push(e.dst);
                    if e.dst == dst {
                        return true;
                    }
                }
                ei = self.next[ei];
            }
        }
        false
    }

    fn find_augmenting_path(&mut self, v: usize, dst: usize, base: i64, flow: i64) -> i64 {
        if v == dst {
            return flow;
        }
        let mut sum = 0;
        while self.iter[v] != !0 {
            let e = self.edges[self.iter[v]];
            if e.cap >= base && self.dist[v] < self.dist[e.dst] {
                let diff = self.find_augmenting_path(e.dst, dst, base, e.cap.min(flow - sum));
                if diff > 0 {
                    self.edges[self.iter[v]].cap -= diff;
                    self.edges[self.iter[v] ^ 1].cap += diff;
                    sum += diff;
                    if flow - sum < base {
                        break;
                    }
                }
            }
            self.iter[v] = self.next[self.iter[v]];
        }
        sum
    }
}
