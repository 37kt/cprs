use std::borrow::Borrow;

use max_flow::MaxFlow;
use numeric_traits::Inf;

/// 多値変数の最適化 (最小化)
#[derive(Clone)]
pub struct MultivaluedOptimization {
    n_options: Vec<usize>,
    n_vertices: usize,

    src: usize,
    dst: usize,

    id: Vec<Vec<usize>>,

    cost_0: i64,

    edges: Vec<(usize, usize, i64)>,
}

impl MultivaluedOptimization {
    pub fn new<I>(n_options: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        let n_options: Vec<_> = n_options.into_iter().map(|c| *c.borrow()).collect();

        let src = 0;
        let dst = 1;
        let mut cur = 2;
        let id = n_options
            .iter()
            .map(|&n| {
                assert!(n > 0);
                let mut id = vec![0; n + 1];
                id[0] = src;
                for i in 1..n {
                    id[i] = cur;
                    cur += 1;
                }
                id[n] = dst;
                id
            })
            .collect();

        let mut opt = Self {
            n_options,
            n_vertices: cur,
            src,
            dst,
            id,
            cost_0: 0,
            edges: vec![],
        };

        for i in 0..opt.n_options.len() {
            for j in 1..opt.n_options[i] - 1 {
                opt.add_edge(opt.id[i][j + 1], opt.id[i][j], i64::inf());
            }
        }

        opt
    }

    pub fn add(&mut self, cost: i64) {
        self.cost_0 += cost;
    }

    pub fn add_1(&mut self, i: usize, mut cost: impl FnMut(usize) -> i64) {
        let mut cost = (0..self.n_options[i])
            .map(|mi| cost(mi))
            .collect::<Vec<_>>();

        let x0 = cost[0];
        self.add(x0);
        for x in &mut cost {
            *x -= x0;
        }

        for j in 1..self.n_options[i] {
            let x = cost[j] - cost[j - 1];
            if x < 0 {
                self.add(x);
                self.add_edge(self.src, self.id[i][j], -x);
            } else {
                self.add_edge(self.id[i][j], self.dst, x);
            }
        }
    }

    pub fn add_2(&mut self, i: usize, j: usize, mut cost: impl FnMut(usize, usize) -> i64) {
        let h = self.n_options[i];
        let w = self.n_options[j];
        let mut cost = (0..h)
            .map(|mi| (0..w).map(|mj| cost(mi, mj)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        self.add_1(j, |mj| cost[0][mj]);

        for mi in (0..h).rev() {
            for mj in 0..w {
                cost[mi][mj] -= cost[0][mj];
            }
        }

        self.add_1(i, |mi| cost[mi][w - 1]);

        for mi in 0..h {
            for mj in 0..w {
                cost[mi][mj] -= cost[mi][w - 1];
            }
        }

        for mi in 1..h {
            for mj in 0..w - 1 {
                let x = cost[mi][mj] + cost[mi - 1][mj + 1] - cost[mi - 1][mj] - cost[mi][mj + 1];
                assert!(x >= 0, "must be monge");
                self.add_edge(self.id[i][mi], self.id[j][mj + 1], x);
            }
        }
    }

    pub fn solve(&self) -> (i64, Vec<usize>) {
        let mut flow = MaxFlow::new();
        flow.add_vertices(self.n_vertices);
        for &(i, j, cost) in &self.edges {
            flow.add_edge(i, j, cost);
        }

        let cost = self.cost_0 + flow.max_flow(self.src, self.dst);
        let cut = flow.min_cut(self.src);

        let mut choice = vec![0; self.n_options.len()];
        for i in 0..self.n_options.len() {
            for j in 1..self.n_options[i] {
                if cut[self.id[i][j]] {
                    choice[i] += 1;
                }
            }
        }

        (cost, choice)
    }

    fn add_edge(&mut self, i: usize, j: usize, cost: i64) {
        assert!(cost >= 0);
        if cost == 0 {
            return;
        }
        self.edges.push((i, j, cost));
    }
}
