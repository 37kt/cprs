use project_selection::ProjectSelection;

const INF: i64 = 1 << 61;

pub struct KProjectSelection {
    n: usize,
    ks: Vec<usize>,
    id: Vec<Vec<usize>>,
    ps: ProjectSelection,
}

impl KProjectSelection {
    pub fn new(ks: &[usize]) -> Self {
        let n = ks.len();
        let mut id = vec![vec![]; n];
        let mut cnt = 0;
        for i in 0..n {
            assert!(ks[i] > 0);
            id[i].resize(ks[i], !0);
            for d in 1..ks[i] {
                id[i][d] = cnt;
                cnt += 1;
            }
        }
        let mut ps = ProjectSelection::new(cnt);
        for i in 0..n {
            for d in 1..ks[i] - 1 {
                ps.add_cost_double_10(id[i][d], id[i][d + 1], INF);
            }
        }
        Self {
            n,
            ks: ks.to_vec(),
            id,
            ps,
        }
    }

    pub fn add_cost(&mut self, cost: i64) {
        self.ps.add_cost(cost);
    }

    pub fn add_profit(&mut self, profit: i64) {
        self.ps.add_profit(profit);
    }

    pub fn add_cost_single(&mut self, i: usize, cost: &[i64]) {
        assert!(cost.len() == self.ks[i]);
        self.ps.add_cost(cost[self.ks[i] - 1]);
        for d in 1..self.ks[i] {
            self.ps
                .add_cost_single_1(self.id[i][d], cost[d - 1] - cost[d]);
        }
    }

    pub fn add_profit_single(&mut self, i: usize, profit: &[i64]) {
        let mut profit = profit.to_vec();
        for p in &mut profit {
            *p = -*p;
        }
        self.add_cost_single(i, &profit);
    }

    pub fn add_cost_double(&mut self, i: usize, j: usize, mut cost: Vec<Vec<i64>>) {
        assert!(i != j);
        assert!(cost.len() == self.ks[i]);
        for di in 0..self.ks[i] {
            assert!(cost[di].len() == self.ks[j]);
        }

        let mut cost_i = vec![0; self.ks[i]];
        let mut cost_j = vec![0; self.ks[j]];
        for di in 0..self.ks[i] {
            cost_i[di] = cost[di][0];
            for dj in 0..self.ks[j] {
                cost[di][dj] -= cost_i[di];
            }
        }
        for dj in 0..self.ks[j] {
            cost_j[dj] = cost[0][dj];
            for di in 0..self.ks[i] {
                cost[di][dj] -= cost_j[dj];
            }
        }
        self.add_cost_single(i, &cost_i);
        self.add_cost_single(j, &cost_j);

        assert!((0..self.ks[i]).all(|di| cost[di][0] == 0));
        assert!((0..self.ks[j]).all(|dj| cost[0][dj] == 0));

        for di in 1..self.ks[i] {
            for dj in 1..self.ks[j] {
                let cost_00 =
                    cost[di][dj] - cost[di][dj - 1] - cost[di - 1][dj] + cost[di - 1][dj - 1];
                assert!(cost_00 <= 0);
                self.ps
                    .add_profit_double_00(self.id[i][di], self.id[j][dj], -cost_00);
            }
        }
    }

    pub fn min_cost(&mut self) -> (i64, Vec<usize>) {
        let (res, f) = self.ps.min_cost();
        let mut g = vec![0; self.n];
        for i in 0..self.n {
            for di in 1..self.ks[i] {
                if !f[self.id[i][di]] {
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
