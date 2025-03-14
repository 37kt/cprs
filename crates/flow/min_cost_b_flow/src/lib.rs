// https://misawa.github.io/others/flow/lets_use_capacity_scaling.html
// TODO: dual がめちゃくちゃ遅い
// TODO: 流量制限, slope

use std::{cmp::Reverse, collections::BinaryHeap, iter::FusedIterator};

#[derive(Clone)]
pub struct MinCostBFlow {
    n: usize,

    edges: Vec<internal::Edge>,
    head: Vec<usize>,
    next: Vec<usize>,

    b: Vec<i64>,
    potential: Vec<i64>,

    farthest: i64,
    dist: Vec<i64>,
    parent: Vec<usize>,
    pq: BinaryHeap<Reverse<(i64, usize)>>,

    excess_vs: Vec<usize>,
    deficit_vs: Vec<usize>,

    time: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub lower: i64,
    pub upper: i64,
    pub cost: i64,
    pub flow: i64,
}

impl MinCostBFlow {
    pub fn new() -> Self {
        Self {
            n: 0,
            edges: vec![],
            head: vec![],
            next: vec![],
            b: vec![],
            potential: vec![],
            farthest: 0,
            dist: vec![],
            parent: vec![],
            pq: BinaryHeap::new(),
            excess_vs: vec![],
            deficit_vs: vec![],
            time: 0.0,
        }
    }

    /// 頂点を追加してその id を返す
    pub fn add_vertex(&mut self) -> usize {
        self.n += 1;
        self.head.push(!0);
        self.b.push(0);
        self.n - 1
    }

    /// 頂点を n 個追加してそれらの id を返す
    pub fn add_vertices(
        &mut self,
        n: usize,
    ) -> impl Iterator<Item = usize> + DoubleEndedIterator + ExactSizeIterator + FusedIterator {
        self.head.resize(self.n + n, !0);
        self.b.resize(self.n + n, 0);
        self.n += n;
        self.n - n..self.n
    }

    /// src->dst に流量制約 [lower, upper], cost の辺を張り、その id を返す
    pub fn add_edge(&mut self, src: usize, dst: usize, lower: i64, upper: i64, cost: i64) -> usize {
        assert!(src < self.n);
        assert!(dst < self.n);
        assert!(lower <= upper);
        let m = self.edges.len();
        self.next.push(self.head[src]);
        self.head[src] = m;
        self.next.push(self.head[dst]);
        self.head[dst] = m + 1;
        self.edges.push(internal::Edge {
            dst,
            cap: upper,
            cost,
            flow: 0,
        });
        self.edges.push(internal::Edge {
            dst: src,
            cap: -lower,
            cost: -cost,
            flow: 0,
        });
        m / 2
    }

    /// 頂点 v に湧き出しを追加する
    pub fn add_supply(&mut self, v: usize, amount: i64) {
        self.b[v] += amount;
    }

    /// 頂点 v に吸い込みを追加する
    pub fn add_demand(&mut self, v: usize, amount: i64) {
        self.b[v] -= amount;
    }

    pub fn edge(&self, e: usize) -> Edge {
        assert!(e <= self.edges.len() / 2);
        let e = e * 2;
        Edge {
            src: self.src(e),
            dst: self.dst(e),
            lower: self.lower(e),
            upper: self.upper(e),
            cost: self.cost(e),
            flow: self.flow(e),
        }
    }

    pub fn edges(
        &self,
    ) -> impl Iterator<Item = Edge> + DoubleEndedIterator + ExactSizeIterator + FusedIterator + '_
    {
        (0..self.edges.len() / 2).map(|e| self.edge(e))
    }

    /// min cost b-flow を求める  
    /// - `Ok(cost)`: 最小費用 (最大流とは限らない)  
    /// - `Err(cost)`: 流量の制約を満たすフローが存在しない
    pub fn min_cost_b_flow(&mut self) -> Result<i128, i128> {
        self.potential.resize(self.n, 0);
        self.potential.fill(0);

        for v in 0..self.n {
            let mut e = self.head[v];
            while e != !0 {
                let rcap = self.residual_cap(e);
                if rcap < 0 {
                    self.push(e, rcap);
                    self.b[v] -= rcap;
                    let u = self.dst(e);
                    self.b[u] += rcap;
                }
                e = self.next[e];
            }
        }

        let inf_flow = (0..self.edges.len())
            .map(|e| self.residual_cap(e))
            .max()
            .unwrap_or_default()
            .max(1);
        let mut delta = 1;
        while delta * 2 <= inf_flow {
            delta *= 2;
        }

        while delta > 0 {
            self.saturate_negative(delta);
            while self.dual(delta) {
                self.primal(delta);
            }
            delta /= 2;
        }

        eprintln!("dual time: {}", self.time);

        let mut cost = 0;
        for &e in &self.edges {
            cost += e.flow as i128 * e.cost as i128;
        }
        cost /= 2;

        if self.excess_vs.is_empty() && self.deficit_vs.is_empty() {
            Ok(cost)
        } else {
            Err(cost)
        }
    }

    /// src->dst 間に最大流を流した時の最小費用を求める  
    /// - `Ok((flow, cost))`: 最大流, 最小費用  
    /// - `Err(cost)`: 流量の制約を満たすフローが存在しない
    pub fn min_cost_flow(&mut self, src: usize, dst: usize) -> Result<(i64, i128), i128> {
        assert!(src != dst);

        let mut inf_flow = self.b[src].abs();
        let mut e = self.head[src];
        while e != !0 {
            inf_flow += 0.max(self.edges[e].cap);
            e = self.next[e];
        }

        self.add_edge(dst, src, 0, inf_flow, 0);
        if let Err(circulation_cost) = self.min_cost_b_flow() {
            self.head[src] = self.next[src];
            self.head[dst] = self.next[dst];
            self.edges.pop();
            self.edges.pop();
            return Err(circulation_cost);
        }

        let mut inf_flow = self.b[src].abs();
        let mut e = self.head[src];
        while e != !0 {
            inf_flow += self.residual_cap(e);
            e = self.next[e];
        }

        self.b[src] += inf_flow;
        self.b[dst] -= inf_flow;
        let cost = match self.min_cost_b_flow() {
            Ok(cost) => cost,
            Err(cost) => cost,
        };
        self.b[src] -= inf_flow;
        self.b[dst] += inf_flow;

        self.head[src] = self.next[src];
        self.head[dst] = self.next[dst];
        self.edges.pop();
        self.edges.pop();

        Ok((self.b[dst], cost))
    }

    pub fn potentials(&mut self) -> Vec<i64> {
        self.potential.resize(self.n, 0);
        self.potential.fill(0);

        for _ in 0..self.n {
            for e in 0..self.edges.len() {
                if self.residual_cap(e) > 0 {
                    let u = self.dst(e);
                    self.potential[u] =
                        self.potential[u].min(self.potential[self.src(e)] + self.cost(e))
                }
            }
        }

        self.potential.clone()
    }

    fn src(&self, e: usize) -> usize {
        self.edges[e ^ 1].dst
    }

    fn dst(&self, e: usize) -> usize {
        self.edges[e].dst
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

    fn residual_cost(&self, e: usize) -> i64 {
        self.cost(e) + self.potential[self.src(e)] - self.potential[self.dst(e)]
    }

    fn residual_cap(&self, e: usize) -> i64 {
        self.edges[e].cap - self.edges[e].flow
    }

    fn push(&mut self, e: usize, amount: i64) {
        self.edges[e].flow += amount;
        self.edges[e ^ 1].flow -= amount;
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
                v = self.src(self.parent[v]);
            }
            f = f.min(self.b[v]);
            if f < delta {
                continue;
            }
            v = t;
            while self.parent[v] != !0 {
                self.push(self.parent[v], f);
                let u = self.src(self.parent[v]);
                self.parent[v] = !0;
                v = u;
            }
            self.b[v] -= f;
            self.b[t] += f;
        }
    }

    fn dual(&mut self, delta: i64) -> bool {
        let start = std::time::Instant::now();

        self.dist.resize(self.n, i64::MAX);
        self.dist.fill(i64::MAX);
        self.parent.resize(self.n, !0);
        self.parent.fill(!0);
        self.excess_vs.retain(|&v| self.b[v] >= delta);
        self.deficit_vs.retain(|&v| self.b[v] <= -delta);

        self.pq.clear();
        for &v in &self.excess_vs {
            self.dist[v] = 0;
            self.pq.push(Reverse((0, v)));
        }
        self.farthest = 0;
        let mut deficit_cnt = 0;
        while let Some(Reverse((d, v))) = self.pq.pop() {
            if self.dist[v] < d {
                continue;
            }
            self.farthest = d;
            if self.b[v] <= -delta {
                deficit_cnt += 1;
            }
            if deficit_cnt >= self.deficit_vs.len() {
                break;
            }
            let mut e = self.head[v];
            while e != !0 {
                if self.residual_cap(e) >= delta {
                    let u = self.dst(e);
                    let new_d = d + self.residual_cost(e);
                    if self.dist[u] > new_d {
                        self.dist[u] = new_d;
                        self.pq.push(Reverse((new_d, u)));
                        self.parent[u] = e;
                    }
                }
                e = self.next[e];
            }
        }

        for v in 0..self.n {
            self.potential[v] += self.dist[v].min(self.farthest);
        }

        self.time += start.elapsed().as_secs_f64();

        deficit_cnt > 0
    }

    fn saturate_negative(&mut self, delta: i64) {
        self.excess_vs.clear();
        self.deficit_vs.clear();
        for v in 0..self.n {
            let mut e = self.head[v];
            while e != !0 {
                let rcap = self.residual_cap(e);
                let rcost = self.residual_cost(e);
                if rcost < 0 && rcap >= delta {
                    self.push(e, rcap);
                    self.b[v] -= rcap;
                    let u = self.dst(e);
                    self.b[u] += rcap;
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

mod internal {
    #[derive(Clone, Copy)]
    pub(crate) struct Edge {
        pub(crate) dst: usize,
        pub(crate) cap: i64,
        pub(crate) cost: i64,
        pub(crate) flow: i64,
    }
}
