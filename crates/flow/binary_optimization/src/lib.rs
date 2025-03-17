use std::borrow::Borrow;

use max_flow::MaxFlow;

/// 2 値変数の最適化 (最小化)
#[derive(Clone)]
pub struct BinaryOptimization {
    n_item: usize,
    n_aux: usize,

    src: usize,
    dst: usize,

    cost_0: i64,           // 0 変数についてのコスト
    cost_1: Vec<[i64; 2]>, // 1 変数についてのコスト

    edges: Vec<(usize, usize, i64)>,
}

impl BinaryOptimization {
    pub fn new(n: usize) -> Self {
        Self {
            n_item: n,
            n_aux: 0,

            src: n,
            dst: n + 1,

            cost_0: 0,
            cost_1: vec![[0; 2]; n],

            edges: vec![],
        }
    }

    pub fn add(&mut self, cost: i64) {
        self.cost_0 += cost;
    }

    pub fn add_1(&mut self, i: usize, mut cost: impl FnMut(usize) -> i64) {
        self.cost_1[i][0] += cost(0);
        self.cost_1[i][1] += cost(1);
    }

    /// monge 性が必要
    pub fn add_2(&mut self, i: usize, j: usize, mut cost: impl FnMut(usize, usize) -> i64) {
        let mut x: [[i64; 2]; 2] = [[0; 2]; 2];
        for bi in 0..2 {
            for bj in 0..2 {
                x[bi][bj] = cost(bi, bj);
            }
        }
        assert!(x[0][0] + x[1][1] <= x[0][1] + x[1][0], "must be monge");

        self.add(x[0][0]);
        self.add_1(i, |bi| [0, x[1][0] - x[0][0]][bi]);
        self.add_1(j, |bj| [0, x[1][1] - x[1][0]][bj]);
        self.add_edge(i, j, (x[0][1] + x[1][0]) - (x[0][0] + x[1][1]));
    }

    pub fn add_3(
        &mut self,
        i: usize,
        j: usize,
        k: usize,
        mut cost: impl FnMut(usize, usize, usize) -> i64,
    ) {
        assert!(i != j && j != k && k != i);

        let x000 = cost(0, 0, 0);
        let x001 = cost(0, 0, 1);
        let x010 = cost(0, 1, 0);
        let x011 = cost(0, 1, 1);
        let x100 = cost(1, 0, 0);
        let x101 = cost(1, 0, 1);
        let x110 = cost(1, 1, 0);
        let x111 = cost(1, 1, 1);
        let p = x000 - x100 - x010 - x001 + x110 + x101 + x011 - x111;

        if p >= 0 {
            self.add(x000);
            self.add_1(i, |bi| [x100 - x000, 0][bi]);
            self.add_1(j, |bj| [x010 - x000, 0][bj]);
            self.add_1(k, |bk| [x001 - x000, 0][bk]);
            self.add_2(i, j, |bi, bj| {
                [[x000 + x110 - x100 - x010, 0], [0, 0]][bi][bj]
            });
            self.add_2(j, k, |bj, bk| {
                [[x000 + x011 - x010 - x001, 0], [0, 0]][bj][bk]
            });
            self.add_2(k, i, |bk, bi| {
                [[x000 + x101 - x001 - x100, 0], [0, 0]][bk][bi]
            });
            self.add_all1([i, j, k], -p);
        } else {
            self.add(x111);
            self.add_1(i, |bi| [x011 - x111, 0][bi]);
            self.add_1(j, |bj| [x101 - x111, 0][bj]);
            self.add_1(k, |bk| [x110 - x111, 0][bk]);
            self.add_2(i, j, |bi, bj| {
                [[x111 + x001 - x011 - x101, 0], [0, 0]][bi][bj]
            });
            self.add_2(j, k, |bj, bk| {
                [[x111 + x100 - x101 - x110, 0], [0, 0]][bj][bk]
            });
            self.add_2(k, i, |bk, bi| {
                [[x111 + x010 - x110 - x011, 0], [0, 0]][bk][bi]
            });
            self.add_all0([i, j, k], p);
        }
    }

    pub fn add_all0<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        let mut items = items.into_iter().map(|x| *x.borrow()).collect::<Vec<_>>();
        items.sort_unstable();
        items.dedup();

        match &items[..] {
            [] => self.add(cost),
            &[i] => self.add_1(i, |bi| [cost, 0][bi]),
            &[i, j] => self.add_2(i, j, |bi, bj| [[cost, 0], [0, 0]][bi][bj]),
            items => {
                self.add(cost);
                let aux = self.n_item + 2 + self.n_aux;
                self.n_aux += 1;
                self.add_edge(self.src, aux, -cost);
                for &i in items {
                    self.add_edge(aux, i, -cost);
                }
            }
        }
    }

    pub fn add_all1<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        let mut items = items.into_iter().map(|x| *x.borrow()).collect::<Vec<_>>();
        items.sort_unstable();
        items.dedup();

        match &items[..] {
            [] => self.add(cost),
            &[i] => self.add_1(i, |bi| [0, cost][bi]),
            &[i, j] => self.add_2(i, j, |bi, bj| [[0, 0], [0, cost]][bi][bj]),
            items => {
                self.add(cost);
                let aux = self.n_item + 2 + self.n_aux;
                self.n_aux += 1;
                for &i in items {
                    self.add_edge(i, aux, -cost);
                }
                self.add_edge(aux, self.dst, -cost);
            }
        }
    }

    pub fn add_any0<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        self.add(cost);
        self.add_all1(items, -cost);
    }

    pub fn add_any1<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        self.add(cost);
        self.add_all0(items, -cost);
    }

    /// (最小コスト, 変数の値)
    pub fn solve(&mut self) -> (i64, Vec<usize>) {
        for i in 0..self.n_item {
            let cost = self.cost_1[i];
            if cost[0] < cost[1] {
                self.add(cost[0]);
                self.add_edge(self.src, i, cost[1] - cost[0]);
            } else {
                self.add(cost[1]);
                self.add_edge(i, self.dst, cost[0] - cost[1]);
            }
            self.cost_1[i] = [0; 2];
        }

        let mut flow = MaxFlow::new();
        flow.add_vertices(self.n_item + 2 + self.n_aux);
        for &(i, j, cost) in &self.edges {
            flow.add_edge(i, j, cost);
        }

        let cost = self.cost_0 + flow.max_flow(self.src, self.dst);
        let mut cut = flow.min_cut();
        cut.truncate(self.n_item);

        (cost, cut)
    }

    fn add_edge(&mut self, i: usize, j: usize, cost: i64) {
        assert!(cost >= 0);
        assert!(i != j);
        if cost == 0 {
            return;
        }
        self.edges.push((i, j, cost));
    }
}
