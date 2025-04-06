/// TODO: 辺の変更
use std::iter::FusedIterator;

use numeric_traits::Integer;

mod graph;
mod queue;

/// dinic + scaling  
#[derive(Clone)]
pub struct MaxFlow {
    edges: Vec<Vec<graph::Edge>>,
    pos: Vec<(usize, usize)>,

    iter: Vec<usize>,
    dist: Vec<u64>,
    queue: queue::Queue,

    zero: u64,

    src: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub cap: i64,
    pub flow: i64,
}

impl Default for MaxFlow {
    fn default() -> Self {
        Self::new()
    }
}

impl MaxFlow {
    pub fn new() -> Self {
        Self {
            edges: vec![],
            pos: vec![],

            iter: vec![],
            dist: vec![],
            queue: queue::Queue::new(),

            zero: 1 << 60,

            src: None,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.edges.len()
    }

    pub fn num_edges(&self) -> usize {
        self.pos.len()
    }

    pub fn add_vertex(&mut self) -> usize {
        self.src = None;

        let v = self.edges.len();
        self.edges.push(vec![]);
        self.iter.push(0);
        self.dist.push(self.zero);
        v
    }

    pub fn add_vertices(&mut self, n: usize) -> Vec<usize> {
        self.src = None;

        let v = self.edges.len();
        self.edges.resize(v + n, vec![]);
        self.iter.resize(v + n, 0);
        self.dist.resize(v + n, self.zero);
        (v..v + n).collect()
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, cap: i64) -> usize {
        assert!(src < self.num_vertices());
        assert!(dst < self.num_vertices());
        assert!(cap >= 0);

        self.src = None;

        let e = self.pos.len();
        let i = self.edges[src].len();
        let j = self.edges[dst].len() + if src == dst { 1 } else { 0 };
        self.edges[src].push(graph::Edge { dst, cap, rev: j });
        self.edges[dst].push(graph::Edge {
            dst: src,
            cap: 0,
            rev: i,
        });
        self.pos.push((src, i));
        e
    }

    pub fn edge(&self, e: usize) -> Edge {
        let (src, i) = self.pos[e];
        let e1 = self.edges[src][i];
        let e2 = self.edges[e1.dst][e1.rev];
        Edge {
            src,
            dst: e1.dst,
            cap: e1.cap + e2.cap,
            flow: e2.cap,
        }
    }

    pub fn edges(
        &self,
    ) -> impl Iterator<Item = Edge> + DoubleEndedIterator + ExactSizeIterator + FusedIterator + '_
    {
        (0..self.num_edges()).map(|e| self.edge(e))
    }

    pub fn max_flow(&mut self, src: usize, dst: usize, limit: Option<i64>) -> i64 {
        assert!(src != dst);
        assert!(src < self.num_vertices());
        assert!(dst < self.num_vertices());

        self.src = Some(src);
        let limit = limit.unwrap_or(i64::MAX);

        let max_cap = self
            .edges
            .iter()
            .flatten()
            .map(|e| e.cap)
            .max()
            .unwrap_or(0);
        if max_cap == 0 {
            return 0;
        }

        let mut flow = 0;
        for f in (0..=limit.min(max_cap).floor_log2()).map(|i| 1 << i).rev() {
            while flow <= limit - f && self.build_augmenting_path(src, dst, f) {
                self.iter.fill(0);
                flow += self.find_augmenting_path(src, dst, f, limit - flow);
            }
        }

        flow
    }

    /// src 側が 0, dst 側が 1  
    /// max_flow の後に呼び出す
    pub fn min_cut(&mut self) -> Vec<usize> {
        let src = self.src.expect("max_flow is not called");

        let mut res = vec![1; self.num_vertices()];

        self.queue.clear();
        self.queue.set_capacity(self.num_vertices());
        res[src] = 0;
        self.queue.push(src);

        while let Some(v) = self.queue.pop() {
            for e in &self.edges[v] {
                if e.cap > 0 && res[e.dst] == 1 {
                    res[e.dst] = 0;
                    self.queue.push(e.dst);
                }
            }
        }
        res
    }

    fn build_augmenting_path(&mut self, src: usize, dst: usize, base: i64) -> bool {
        self.zero -= 1 << 30;

        self.queue.clear();
        self.queue.set_capacity(self.num_vertices());

        self.dist[src] = self.zero;
        self.queue.push(src);
        while let Some(v) = self.queue.pop() {
            for e in &self.edges[v] {
                if e.cap >= base && self.dist[e.dst] > self.dist[v] + 1 {
                    self.dist[e.dst] = self.dist[v] + 1;
                    self.queue.push(e.dst);
                    if e.dst == dst {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn find_augmenting_path(&mut self, v: usize, dst: usize, base: i64, flow: i64) -> i64 {
        if v == dst {
            return flow;
        }

        let mut sum = 0;
        while self.iter[v] < self.edges[v].len() {
            let e = self.edges[v][self.iter[v]];
            // if e.cap >= base && self.dist[v] < self.dist[e.dst] {
            if e.cap >= base && self.dist[v] + 1 == self.dist[e.dst] {
                let diff = self.find_augmenting_path(e.dst, dst, base, e.cap.min(flow - sum));
                if diff > 0 {
                    self.edges[v][self.iter[v]].cap -= diff;
                    self.edges[e.dst][e.rev].cap += diff;
                    sum += diff;
                    if flow - sum < base {
                        break;
                    }
                }
            }
            self.iter[v] += 1;
        }
        sum
    }
}
