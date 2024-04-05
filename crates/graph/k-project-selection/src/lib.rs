use project_selection::ProjectSelection;

const INF: i64 = 1 << 61;

pub struct KProjectSelection<const K: usize> {
    n: usize,
    ps: ProjectSelection,
}

fn id(i: usize, d: usize, k: usize) -> usize {
    if d == 0 {
        !0
    } else {
        i * (k - 1) + (d - 1)
    }
}

impl<const K: usize> KProjectSelection<K> {
    pub fn new(n: usize) -> Self {
        let mut ps = ProjectSelection::new(n * (K - 1));
        for i in 0..n {
            for d in 1..K - 1 {
                ps.add_cost_double_10(id(i, d, K), id(i, d + 1, K), INF);
            }
        }
        Self { n, ps }
    }

    pub fn add_cost(&mut self, cost: i64) {
        self.ps.add_cost(cost);
    }

    pub fn add_profit(&mut self, profit: i64) {
        self.ps.add_profit(profit);
    }

    pub fn add_cost_single(&mut self, i: usize, cost: [i64; K]) {
        self.ps.add_cost(cost[K - 1]);
        for d in 1..K {
            self.ps
                .add_cost_single_1(id(i, d, K), cost[d - 1] - cost[d]);
        }
    }

    pub fn add_profit_single(&mut self, i: usize, mut profit: [i64; K]) {
        for i in 0..K {
            profit[i] = -profit[i];
        }
        self.add_cost_single(i, profit);
    }

    pub fn add_cost_double(&mut self, i: usize, j: usize, mut cost: [[i64; K]; K]) {
        let mut cost_i = [0; K];
        let mut cost_j = [0; K];
        for di in 0..K {
            cost_i[di] = cost[di][0];
            for dj in 0..K {
                cost[di][dj] -= cost_i[di];
            }
        }
        for dj in 0..K {
            cost_j[dj] = cost[0][dj];
            for di in 0..K {
                cost[di][dj] -= cost_j[dj];
            }
        }
        self.add_cost_single(i, cost_i);
        self.add_cost_single(j, cost_j);

        assert!((0..K).all(|di| cost[di][0] == 0));
        assert!((0..K).all(|dj| cost[0][dj] == 0));

        for di in 1..K {
            for dj in 1..K {
                let cost_00 =
                    cost[di][dj] - cost[di][dj - 1] - cost[di - 1][dj] + cost[di - 1][dj - 1];
                assert!(cost_00 <= 0);
                self.ps
                    .add_profit_double_00(id(i, di, K), id(j, dj, K), -cost_00);
            }
        }
    }

    pub fn min_cost(&mut self) -> (i64, Vec<usize>) {
        let (res, f) = self.ps.min_cost();
        let mut g = vec![0; self.n];
        for i in 0..self.n {
            for di in 1..K {
                if !f[id(i, di, K)] {
                    g[i] += 1;
                }
            }
        }
        (res, g)
    }

    pub fn max_profit(&mut self) -> (i64, Vec<usize>) {
        let (mut res, f) = self.min_cost();
        res = -res;
        (res, f)
    }
}
