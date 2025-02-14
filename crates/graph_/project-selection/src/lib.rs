use max_flow::MaxFlow;

type Cost1 = [i64; 2];
type Cost2 = [Cost1; 2];
type Cost3 = [Cost2; 2];

const S: usize = !0;
const T: usize = !1;

/// Project Selection Problem  
///
/// # 概要
/// n 個のアイテムについて 0 か 1 を選ぶ。  
///
/// - アイテム i について選択肢 x を選ぶとコストがかかる
/// - アイテム i, j についてそれぞれ選択肢 x, y を選ぶとコストがかかる
/// - アイテム i, j, k についてそれぞれ選択肢 x, y, z を選ぶとコストがかかる
///
/// といった条件がある。このときの最小コストとそれを達成する選択肢の組を求める。
#[derive(Clone)]
pub struct ProjectSelection {
    n_item: usize,
    n_aux: usize,
    cost0: i64,
    cost1: Vec<Cost1>,
    edges: Vec<(usize, usize, i64)>,
}

impl ProjectSelection {
    pub fn new(n: usize) -> Self {
        Self {
            n_item: n,
            n_aux: 0,
            cost0: 0,
            cost1: vec![Default::default(); n],
            edges: vec![],
        }
    }

    /// 選択肢の選び方に依存しないコストを追加する
    pub fn add_cost(&mut self, cost: i64) {
        self.cost0 += cost;
    }

    /// 選択肢の選び方に依存しない利益を追加する
    pub fn add_profit(&mut self, profit: i64) {
        self.add_cost(-profit);
    }

    /// アイテム i について選択肢 j を選ぶとコスト cost\[j\] かかるという条件を追加する
    pub fn add_cost_1item(&mut self, i: usize, cost: Cost1) {
        self.cost1[i][0] += cost[0];
        self.cost1[i][1] += cost[1];
    }

    /// アイテム i について選択肢 j を選ぶと利益 profit\[j\] が得られるという条件を追加する
    pub fn add_profit_1item(&mut self, i: usize, profit: Cost1) {
        self.add_cost_1item(i, [-profit[0], -profit[1]]);
    }

    /// アイテム i について選択肢 0 を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_1item_0(&mut self, i: usize, cost: i64) {
        self.add_cost_1item(i, [cost, 0]);
    }

    /// アイテム i について選択肢 0 を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_1item_0(&mut self, i: usize, profit: i64) {
        self.add_profit_1item(i, [profit, 0]);
    }

    /// アイテム i について選択肢 1 を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_1item_1(&mut self, i: usize, cost: i64) {
        self.add_cost_1item(i, [0, cost]);
    }

    /// アイテム i について選択肢 1 を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_1item_1(&mut self, i: usize, profit: i64) {
        self.add_profit_1item(i, [0, profit]);
    }

    /// アイテム i, j についてそれぞれ選択肢 x, y を選ぶとコスト cost\[x\]\[y\] かかるという条件を追加する
    pub fn add_cost_2items(&mut self, i: usize, j: usize, cost: Cost2) {
        assert!(i != j);
        self.add_cost(cost[0][0]);
        self.add_cost_1item_1(i, cost[1][0] - cost[0][0]);
        self.add_cost_1item_1(j, cost[1][1] - cost[1][0]);
        self.add_cost_2items_01(i, j, (cost[0][1] + cost[1][0]) - (cost[0][0] + cost[1][1]));
    }

    /// アイテム i, j についてそれぞれ選択肢 x, y を選ぶと利益 profit\[x\]\[y\] が得られるという条件を追加する
    pub fn add_profit_2items(&mut self, i: usize, j: usize, profit: Cost2) {
        self.add_cost_2items(
            i,
            j,
            [
                [-profit[0][0], -profit[0][1]],
                [-profit[1][0], -profit[1][1]],
            ],
        );
    }

    /// アイテム i, j についてそれぞれ選択肢 0, 1 を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_2items_01(&mut self, i: usize, j: usize, cost: i64) {
        assert!(i != j);
        self.add_edge(i, j, cost);
    }

    /// アイテム i, j についてそれぞれ選択肢 1, 0 を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_2items_10(&mut self, i: usize, j: usize, cost: i64) {
        self.add_cost_2items_01(j, i, cost);
    }

    /// アイテム i, j について異なる選択肢を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_2items_not_same(&mut self, i: usize, j: usize, cost: i64) {
        self.add_cost_2items(i, j, [[0, cost], [cost, 0]]);
    }

    /// アイテム i, j についてどちらも選択肢 0 を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_2items_00(&mut self, i: usize, j: usize, profit: i64) {
        self.add_profit_2items(i, j, [[profit, 0], [0, 0]]);
    }

    /// アイテム i, j についてどちらも選択肢 1 を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_2items_11(&mut self, i: usize, j: usize, profit: i64) {
        self.add_profit_2items(i, j, [[0, 0], [0, profit]]);
    }

    /// アイテム i, j について同じ選択肢を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_2items_same(&mut self, i: usize, j: usize, profit: i64) {
        self.add_profit_2items(i, j, [[profit, 0], [0, profit]]);
    }

    /// アイテム i, j, k についてそれぞれ選択肢 x, y, z を選ぶとコスト cost\[x\]\[y\]\[z\] かかるという条件を追加する
    pub fn add_cost_3items(&mut self, i: usize, j: usize, k: usize, cost: Cost3) {
        assert!(i != j && j != k && k != i);
        let a = cost[0][0][0];
        let b = cost[0][0][1];
        let c = cost[0][1][0];
        let d = cost[0][1][1];
        let e = cost[1][0][0];
        let f = cost[1][0][1];
        let g = cost[1][1][0];
        let h = cost[1][1][1];
        let p = (a + d + f + g) - (b + c + e + h);
        if p >= 0 {
            let p1 = f - b;
            let p2 = g - e;
            let p3 = d - c;
            let p12 = (c + e) - (a + g);
            let p23 = (b + c) - (a + d);
            let p31 = (b + e) - (a + f);
            self.add_cost(a);
            self.add_cost_1item_1(i, p1);
            self.add_cost_1item_1(j, p2);
            self.add_cost_1item_1(k, p3);
            self.add_cost_2items_01(i, j, p12);
            self.add_cost_2items_01(j, k, p23);
            self.add_cost_2items_01(k, i, p31);
            self.add_profit_all_1(&[i, j, k], p);
        } else {
            let p1 = c - g;
            let p2 = b - d;
            let p3 = e - f;
            let p21 = (d + f) - (b + h);
            let p32 = (f + g) - (e + h);
            let p13 = (d + g) - (c + h);
            self.add_cost(h);
            self.add_cost_1item_0(i, p1);
            self.add_cost_1item_0(j, p2);
            self.add_cost_1item_0(k, p3);
            self.add_cost_2items_10(i, j, p21);
            self.add_cost_2items_10(j, k, p32);
            self.add_cost_2items_10(k, i, p13);
            self.add_profit_all_0(&[i, j, k], -p);
        }
    }

    /// アイテム i, j, k についてそれぞれ選択肢 x, y, z を選ぶと利益 profit\[x\]\[y\]\[z\] が得られるという条件を追加する
    pub fn add_profit_3items(&mut self, i: usize, j: usize, k: usize, profit: Cost3) {
        self.add_cost_3items(
            i,
            j,
            k,
            [
                [
                    [-profit[0][0][0], -profit[0][0][1]],
                    [-profit[0][1][0], -profit[0][1][1]],
                ],
                [
                    [-profit[1][0][0], -profit[1][0][1]],
                    [-profit[1][1][0], -profit[1][1][1]],
                ],
            ],
        );
    }

    /// アイテム is についてすべて選択肢 0 を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_all_0(&mut self, is: &[usize], profit: i64) {
        let n = is.len();
        let mut is = is.to_vec();
        is.sort();
        is.dedup();
        assert!(is.len() == n);

        if is.len() == 0 {
            self.add_profit(profit);
        } else if is.len() == 1 {
            self.add_profit_1item_0(is[0], profit);
        } else if is.len() == 2 {
            self.add_profit_2items_00(is[0], is[1], profit);
        } else {
            self.add_profit(profit);
            let aux = self.n_item + self.n_aux;
            self.n_aux += 1;
            self.add_edge(S, aux, profit);
            for &i in &is {
                self.add_edge(aux, i, profit);
            }
        }
    }

    /// アイテム is についてすべて選択肢 1 を選ぶと利益 profit が得られるという条件を追加する
    pub fn add_profit_all_1(&mut self, is: &[usize], profit: i64) {
        let n = is.len();
        let mut is = is.to_vec();
        is.sort();
        is.dedup();
        assert!(is.len() == n);

        if is.len() == 0 {
            self.add_profit(profit);
        } else if is.len() == 1 {
            self.add_profit_1item_1(is[0], profit);
        } else if is.len() == 2 {
            self.add_profit_2items_11(is[0], is[1], profit);
        } else {
            self.add_profit(profit);
            let aux = self.n_item + self.n_aux;
            self.n_aux += 1;
            for &i in &is {
                self.add_edge(i, aux, profit);
            }
            self.add_edge(aux, T, profit);
        }
    }

    /// アイテム is についてどれか 1 つでも選択肢 0 を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_any_0(&mut self, is: &[usize], cost: i64) {
        self.add_cost(cost);
        self.add_profit_all_1(is, cost);
    }

    /// アイテム is についてどれか 1 つでも選択肢 1 を選ぶとコスト cost かかるという条件を追加する
    pub fn add_cost_any_1(&mut self, is: &[usize], cost: i64) {
        self.add_cost(cost);
        self.add_profit_all_0(is, cost);
    }

    /// 最小コストとそれを達成する選択肢の組を求める
    pub fn min_cost(&mut self) -> (i64, Vec<usize>) {
        let mut g = MaxFlow::new(self.n_item + self.n_aux + 2);
        let s = self.n_item + self.n_aux;
        let t = s + 1;

        for i in 0..self.n_item {
            let cost = self.cost1[i];
            if cost[0] <= cost[1] {
                self.add_cost(cost[0]);
                self.add_edge(S, i, cost[1] - cost[0]);
            } else {
                self.add_cost(cost[1]);
                self.add_edge(i, T, cost[0] - cost[1]);
            }
            self.cost1[i] = [0, 0];
        }

        for &(i, j, cost) in &self.edges {
            let u = match i {
                S => s,
                T => t,
                _ => i,
            };
            let v = match j {
                S => s,
                T => t,
                _ => j,
            };
            g.add_edge(u, v, cost);
        }
        let res = self.cost0 + g.max_flow(s, t);
        let mut cut = g.min_cut(s);
        cut.truncate(self.n_item);
        let choice = cut.iter().map(|&b| !b as usize).collect();
        (res, choice)
    }

    /// 最大利益とそれを達成する選択肢の組を求める
    pub fn max_profit(&mut self) -> (i64, Vec<usize>) {
        let (mut res, choice) = self.min_cost();
        res = -res;
        (res, choice)
    }

    fn add_edge(&mut self, i: usize, j: usize, cost: i64) {
        assert!(cost >= 0);
        assert!(i != j);
        assert!(i == S || i == T || i < self.n_item + self.n_aux);
        assert!(j == S || j == T || j < self.n_item + self.n_aux);
        if cost == 0 {
            return;
        }
        self.edges.push((i, j, cost));
    }
}
