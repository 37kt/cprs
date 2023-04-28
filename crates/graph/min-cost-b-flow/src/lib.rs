// reference: https://misawa.github.io/others/flow/lets_use_capacity_scaling.html

use std::{cmp::Reverse, collections::BinaryHeap};

const UNREACHABLE: i64 = std::i64::MAX;
const SCALING_FACTOR: i64 = 2;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub lower: i64,
    pub upper: i64,
    pub cost: i64,
    pub gain: i64,
    pub flow: i64,
}

pub struct MinCostBFlow {
    n: usize,
    edges: Vec<Edge_>,
    head: Vec<usize>,
    next: Vec<usize>,
    b: Vec<i64>,
    farthest: i64,
    potential: Vec<i64>,
    dist: Vec<i64>,
    parent: Vec<usize>,
    pq: BinaryHeap<Reverse<(i64, usize)>>,
    excess_vs: Vec<usize>,
    deficit_vs: Vec<usize>,
}

impl MinCostBFlow {
    pub fn new() -> Self {
        Self {
            n: 0,
            edges: vec![],
            head: vec![],
            next: vec![],
            b: vec![],
            farthest: 0,
            potential: vec![],
            dist: vec![],
            parent: vec![],
            pq: Default::default(),
            excess_vs: vec![],
            deficit_vs: vec![],
        }
    }

    pub fn add_vertex(&mut self) -> usize {
        self.n += 1;
        self.head.push(!0);
        self.b.push(0);
        self.n - 1
    }

    pub fn add_vertices(&mut self, size: usize) -> Vec<usize> {
        self.n += size;
        self.head.append(&mut vec![!0; size]);
        self.b.append(&mut vec![0; size]);
        (self.n - size..self.n).collect()
    }

    pub fn add_edge(&mut self, from: usize, to: usize, lower: i64, upper: i64, cost: i64) -> usize {
        assert!(lower <= upper);
        assert!(from < self.n);
        assert!(to < self.n);
        let m = self.edges.len();
        self.next.push(self.head[from]);
        self.head[from] = m;
        self.next.push(self.head[to]);
        self.head[to] = m + 1;
        self.edges.push(Edge_ {
            to,
            cap: upper,
            cost,
            flow: 0,
        });
        self.edges.push(Edge_ {
            to: from,
            cap: -lower,
            cost: -cost,
            flow: 0,
        });
        m / 2
    }

    pub fn add_supply(&mut self, v: usize, amount: i64) {
        self.b[v] += amount;
    }

    pub fn add_demand(&mut self, v: usize, amount: i64) {
        self.b[v] -= amount;
    }

    pub fn get_edge(&self, e: usize) -> Edge {
        assert!(e <= self.edges.len() / 2);
        let e = e * 2;
        Edge {
            from: self.from(e),
            to: self.to(e),
            lower: self.lower(e),
            upper: self.upper(e),
            cost: self.cost(e),
            gain: self.gain(e),
            flow: self.flow(e),
        }
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        (0..self.edges.len() / 2)
            .map(|e| self.get_edge(e))
            .collect()
    }

    pub fn min_cost_b_flow(&mut self) -> Result<i64, i64> {
        self.potential.resize(self.n, 0);
        for u in 0..self.n {
            let mut e = self.head[u];
            while e != !0 {
                let rcap = self.residual_cap(e);
                if rcap < 0 {
                    self.push(e, rcap);
                    self.b[u] -= rcap;
                    let v = self.to(e);
                    self.b[v] += rcap;
                }
                e = self.next[e];
            }
        }

        let mut inf_flow = 1;
        for e in 0..self.edges.len() {
            inf_flow = inf_flow.max(self.residual_cap(e));
        }
        let mut delta = 1;
        while delta * SCALING_FACTOR <= inf_flow {
            delta *= SCALING_FACTOR;
        }

        while delta > 0 {
            self.saturate_negative(delta);
            while self.dual(delta) {
                self.primal(delta);
            }
            delta /= SCALING_FACTOR;
        }

        let mut value = 0;
        for Edge_ { flow, cost, .. } in &self.edges {
            value += flow * cost;
        }
        value /= 2;

        if self.excess_vs.is_empty() && self.deficit_vs.is_empty() {
            Ok(value)
        } else {
            Err(value)
        }
    }

    pub fn min_cost_flow(&mut self, s: usize, t: usize) -> Result<(i64, i64), (i64, i64)> {
        assert!(s != t);
        let mut inf_flow = self.b[s].abs();
        let mut e = self.head[s];
        while e != !0 {
            inf_flow += 0.max(self.edges[e].cap);
            e = self.next[e];
        }

        self.add_edge(t, s, 0, inf_flow, 0);
        if let Err(circulation_value) = self.min_cost_b_flow() {
            self.head[s] = self.next[s];
            self.head[t] = self.next[t];
            self.edges.pop();
            self.edges.pop();
            return Err((circulation_value, 0));
        }

        let mut inf_flow = self.b[s].abs();
        let mut e = self.head[s];
        while e != !0 {
            inf_flow += self.residual_cap(e);
            e = self.next[e];
        }
        self.b[s] += inf_flow;
        self.b[t] -= inf_flow;
        let mf_value = match self.min_cost_b_flow() {
            Ok(v) => v,
            Err(v) => v,
        };
        self.b[s] -= inf_flow;
        self.b[t] += inf_flow;

        self.head[s] = self.next[s];
        self.head[t] = self.next[t];
        self.edges.pop();
        self.edges.pop();
        Ok((mf_value, self.b[t]))
    }

    pub fn get_result_value_i128(&mut self) -> i128 {
        let mut value = 0;
        for e in &self.edges {
            value += e.flow as i128 * e.cost as i128;
        }
        value / 2
    }

    pub fn get_potential(&mut self) -> Vec<i64> {
        self.potential.fill(0);
        for _ in 0..self.n {
            for e in 0..self.edges.len() {
                if self.residual_cap(e) > 0 {
                    let to = self.to(e);
                    self.potential[to] =
                        self.potential[to].min(self.potential[self.from(e)] + self.cost(e));
                }
            }
        }
        self.potential.clone()
    }

    fn from(&self, e: usize) -> usize {
        self.edges[e ^ 1].to
    }

    fn to(&self, e: usize) -> usize {
        self.edges[e].to
    }

    fn flow(&self, e: usize) -> i64 {
        self.edges[e].flow
    }

    fn lower(&self, e: usize) -> i64 {
        -self.edges[e ^ 1].cap
    }

    fn upper(&self, e: usize) -> i64 {
        self.edges[e].cap
    }

    fn cost(&self, e: usize) -> i64 {
        self.edges[e].cost
    }

    fn gain(&self, e: usize) -> i64 {
        -self.edges[e].cost
    }

    fn push(&mut self, e: usize, amount: i64) {
        self.edges[e].flow += amount;
        self.edges[e ^ 1].flow -= amount;
    }

    fn residual_cost(&self, e: usize) -> i64 {
        self.cost(e) + self.potential[self.from(e)] - self.potential[self.to(e)]
    }

    fn residual_cap(&self, e: usize) -> i64 {
        self.edges[e].cap - self.edges[e].flow
    }

    fn dual(&mut self, delta: i64) -> bool {
        self.dist = vec![UNREACHABLE; self.n];
        self.parent = vec![!0; self.n];
        // self.excess_vs.retain(|v| self.b[*v] >= delta);
        for i in (0..self.excess_vs.len()).rev() {
            if self.b[self.excess_vs[i]] < delta {
                self.excess_vs.swap_remove(i);
            }
        }
        // self.deficit_vs.retain(|v| self.b[*v] <= -delta);
        for i in (0..self.deficit_vs.len()).rev() {
            if self.b[self.deficit_vs[i]] > -delta {
                self.deficit_vs.swap_remove(i);
            }
        }
        for &v in &self.excess_vs {
            self.dist[v] = 0;
            self.pq.push(Reverse((0, v)));
        }
        self.farthest = 0;
        let mut deficit_count = 0;
        while let Some(Reverse((d, u))) = self.pq.pop() {
            if self.dist[u] < d {
                continue;
            }
            self.farthest = d;
            if self.b[u] <= -delta {
                deficit_count += 1;
            }
            if deficit_count >= self.deficit_vs.len() {
                break;
            }
            let mut e = self.head[u];
            while e != !0 {
                if self.residual_cap(e) >= delta {
                    let v = self.to(e);
                    let new_dist = d + self.residual_cost(e);
                    if self.dist[v] > new_dist {
                        self.dist[v] = new_dist;
                        self.pq.push(Reverse((new_dist, v)));
                        self.parent[v] = e;
                    }
                }
                e = self.next[e];
            }
        }
        self.pq.clear();
        for v in 0..self.n {
            self.potential[v] += self.dist[v].min(self.farthest);
        }
        deficit_count > 0
    }

    fn primal(&mut self, delta: i64) {
        for i in 0..self.deficit_vs.len() {
            let t = self.deficit_vs[i];
            if self.dist[t] > self.farthest {
                continue;
            }
            let mut f = -self.b[t];
            let mut v = t;
            while self.parent[v] != !0 && f >= delta {
                f = f.min(self.residual_cap(self.parent[v]));
                v = self.from(self.parent[v]);
            }
            f = f.min(self.b[v]);
            if f < delta {
                continue;
            }
            v = t;
            while self.parent[v] != !0 {
                self.push(self.parent[v], f);
                let u = self.from(self.parent[v]);
                self.parent[v] = !0;
                v = u;
            }
            self.b[t] += f;
            self.b[v] -= f;
        }
    }

    fn saturate_negative(&mut self, delta: i64) {
        self.excess_vs.clear();
        self.deficit_vs.clear();
        for u in 0..self.n {
            let mut e = self.head[u];
            while e != !0 {
                let rcap = self.residual_cap(e);
                let rcost = self.residual_cost(e);
                if rcost < 0 && rcap >= delta {
                    self.push(e, rcap);
                    self.b[u] -= rcap;
                    let v = self.to(e);
                    self.b[v] += rcap;
                }
                e = self.next[e];
            }
        }
        for v in 0..self.n {
            if self.b[v] > 0 {
                self.excess_vs.push(v);
            } else if self.b[v] < 0 {
                self.deficit_vs.push(v);
            }
        }
    }
}

struct Edge_ {
    to: usize,
    cap: i64,
    cost: i64,
    flow: i64,
}
