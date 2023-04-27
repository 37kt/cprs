// verify: https://atcoder.jp/contests/abc225/tasks/abc225_g

use std::collections::VecDeque;

pub type FlowType = i64;
const INF: FlowType = std::i64::MAX / 2;

/// dinic法+scalingで最大流を求める
pub struct MaxFlow {
    n: usize,
    edges: Vec<Edge_>,
    head: Vec<usize>,
    next: Vec<usize>,
    min_cost: Vec<i32>,
    iter: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cap: FlowType,
    pub flow: FlowType,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![],
            head: vec![!0; n],
            next: vec![],
            min_cost: vec![],
            iter: vec![],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: FlowType) -> usize {
        assert!(from != to);
        assert!(from < self.n);
        assert!(to < self.n);
        assert!(cap >= 0);
        let m = self.edges.len();
        self.next.push(self.head[from]);
        self.head[from] = m;
        self.next.push(self.head[to]);
        self.head[to] = m + 1;
        self.edges.push(Edge_ { to, cap });
        self.edges.push(Edge_ { to: from, cap: 0 });
        m / 2
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> FlowType {
        assert!(s != t);
        let max_cap = self.edges.iter().map(|e| e.cap).max().unwrap_or(0);
        if max_cap == 0 {
            return 0;
        }
        let mut flow = 0;
        for i in (0..64 - max_cap.leading_zeros()).rev() {
            let now = 1 << i;
            while self.build_augment_path(s, t, now) {
                self.iter = self.head.clone();
                flow += self.find_augment_path(s, t, now, INF);
            }
        }
        flow
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut vis = vec![false; self.n];
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            vis[v] = true;
            let mut e = self.head[v];
            while e != !0 {
                let Edge_ { to, cap } = self.edges[e];
                if cap != 0 && !vis[to] {
                    vis[to] = true;
                    q.push_back(to);
                }
                e = self.next[e];
            }
        }
        vis
    }

    pub fn edges(&self) -> Vec<Edge> {
        (0..self.edges.len())
            .step_by(2)
            .map(|i| {
                let e = &self.edges[i];
                let f = &self.edges[i + 1];
                Edge {
                    from: f.to,
                    to: e.to,
                    cap: e.cap + f.cap,
                    flow: f.cap,
                }
            })
            .collect()
    }

    fn build_augment_path(&mut self, s: usize, t: usize, base: FlowType) -> bool {
        self.min_cost = vec![-1; self.n];
        let mut q = VecDeque::new();
        self.min_cost[s] = 0;
        q.push_back(s);
        while q.len() > 0 && self.min_cost[t] == -1 {
            let v = q.pop_front().unwrap();
            let mut e = self.head[v];
            while e != !0 {
                let Edge_ { to, cap } = self.edges[e];
                if cap >= base && self.min_cost[to] == -1 {
                    self.min_cost[to] = self.min_cost[v] + 1;
                    q.push_back(to);
                }
                e = self.next[e];
            }
        }
        self.min_cost[t] != -1
    }

    fn find_augment_path(
        &mut self,
        v: usize,
        t: usize,
        base: FlowType,
        flow: FlowType,
    ) -> FlowType {
        if v == t {
            return flow;
        }
        let mut sum = 0;
        while self.iter[v] != !0 {
            let Edge_ { to, cap } = self.edges[self.iter[v]];
            if cap >= base && self.min_cost[v] < self.min_cost[to] {
                let d = self.find_augment_path(to, t, base, cap.min(flow - sum));
                if d > 0 {
                    self.edges[self.iter[v]].cap -= d;
                    self.edges[self.iter[v] ^ 1].cap += d;
                    sum += d;
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

struct Edge_ {
    to: usize,
    cap: FlowType,
}
