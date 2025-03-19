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

    /// 定数を加算する  
    ///
    /// # 引数
    /// - `cost`: 加算する定数項のコスト。
    pub fn add_nullary(&mut self, cost: i64) {
        self.cost_0 = self.cost_0.saturating_add(cost);
    }

    /// 1 変数関数を加算する  
    /// +∞ は None で表す  
    ///
    /// # 引数
    /// - `i`: 対象の変数のインデックス。
    /// - `cost`: 各選択肢に対するコストを返すクロージャ。Noneの場合は+∞とみなす。
    pub fn add_unary(&mut self, i: usize, mut cost: impl FnMut(usize) -> Option<i64>) {
        for bi in 0..2 {
            let x = cost(bi).unwrap_or(i64::MAX);
            self.cost_1[i][bi] = self.cost_1[i][bi].saturating_add(x);
        }
    }

    /// 2 変数関数を加算する  
    /// +∞ は None で表す  
    ///
    /// # 制約
    /// - Monge 性を満たす  
    /// - `|cost| <= 10^9` or `cost = None`  
    /// - None は非対角線上に高々 1 つまで、対角線上に存在しない
    ///
    /// # 引数
    /// - `i`, `j`: 対象の2変数関数に適用する変数のインデックス。
    pub fn add_binary(
        &mut self,
        i: usize,
        j: usize,
        mut cost: impl FnMut(usize, usize) -> Option<i64>,
    ) {
        const INF: i64 = 1 << 40; // TODO: これは大丈夫？

        let cost: [[i64; 2]; 2] =
            std::array::from_fn(|bi| std::array::from_fn(|bj| cost(bi, bj).unwrap_or(INF)));

        let x = cost[0][1] + cost[1][0] - cost[0][0] - cost[1][1];
        assert!(x >= 0, "Monge property is violated");

        self.add_nullary(cost[0][0]);
        self.add_unary(i, |bi| Some([0, cost[1][0] - cost[0][0]][bi]));
        self.add_unary(j, |bj| Some([0, cost[1][1] - cost[1][0]][bj]));
        self.add_edge(i, j, x);
    }

    /// 3 変数関数を加算する  
    ///
    /// ??????????
    pub fn add_ternary(
        &mut self,
        _i: usize,
        _j: usize,
        _k: usize,
        mut _cost: impl FnMut(usize, usize, usize) -> i64,
    ) {
        todo!()
    }

    /// 全ての変数が 0 の場合にのみコストを加算する  
    ///
    /// # 制約
    /// - `items.len() < 2` or `cost <= 0`
    ///
    /// # 引数
    /// - `items`: 対象の変数のインデックス。
    /// - `cost`: 加算するコスト。
    pub fn add_if_all_0<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        if cost == 0 {
            return;
        }

        let mut items = items.into_iter().map(|x| *x.borrow()).collect::<Vec<_>>();
        items.sort_unstable();
        items.dedup();

        assert!(
            items.len() < 2 || cost <= 0,
            "if items.len() >= 2, cost must be non-positive"
        );

        match &items[..] {
            [] => self.add_nullary(cost),
            &[i] => self.add_unary(i, |bi| Some([cost, 0][bi])),
            &[i, j] => self.add_binary(i, j, |bi, bj| Some([[cost, 0], [0, 0]][bi][bj])),
            items => {
                self.add_nullary(cost);
                let aux = self.n_item + 2 + self.n_aux;
                self.n_aux += 1;
                self.add_edge(self.src, aux, -cost);
                for &i in items {
                    self.add_edge(aux, i, -cost);
                }
            }
        }
    }

    /// 全ての変数が 1 の場合にのみコストを加算する  
    ///
    /// # 制約
    /// - `items.len() < 2` or `cost <= 0`
    ///
    /// # 引数
    /// - `items`: 対象の変数のインデックス。
    /// - `cost`: 加算するコスト。
    pub fn add_if_all_1<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        if cost == 0 {
            return;
        }

        let mut items = items.into_iter().map(|x| *x.borrow()).collect::<Vec<_>>();
        items.sort_unstable();
        items.dedup();

        assert!(
            items.len() < 2 || cost <= 0,
            "if items.len() >= 2, cost must be non-positive"
        );

        match &items[..] {
            [] => self.add_nullary(cost),
            &[i] => self.add_unary(i, |bi| Some([0, cost][bi])),
            &[i, j] => self.add_binary(i, j, |bi, bj| Some([[0, 0], [0, cost]][bi][bj])),
            items => {
                self.add_nullary(cost);
                let aux = self.n_item + 2 + self.n_aux;
                self.n_aux += 1;
                for &i in items {
                    self.add_edge(i, aux, -cost);
                }
                self.add_edge(aux, self.dst, -cost);
            }
        }
    }

    /// ある変数が 0 の場合にのみコストを加算する  
    ///
    /// # 制約
    /// - `items.len() < 2` or `cost >= 0`
    ///
    /// # 引数
    /// - `items`: 対象の変数のインデックス。
    /// - `cost`: 加算するコスト。
    pub fn add_if_any_0<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        self.add_nullary(cost);
        self.add_if_all_1(items, -cost);
    }

    /// ある変数が 1 の場合にのみコストを加算する  
    ///
    /// # 制約
    /// - `items.len() < 2` or `cost >= 0`
    ///
    /// # 引数
    /// - `items`: 対象の変数のインデックス。
    /// - `cost`: 加算するコスト。
    pub fn add_if_any_1<I>(&mut self, items: I, cost: i64)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        self.add_nullary(cost);
        self.add_if_all_0(items, -cost);
    }

    /// 最適化問題を解いて、最小コストと各変数の選択値を返す。
    ///
    /// # 戻り値
    /// - タプル (最小コスト, 各変数に対する選択されたインデックス)。
    pub fn solve(&mut self) -> (i64, Vec<usize>) {
        for (i, cost) in std::mem::take(&mut self.cost_1).into_iter().enumerate() {
            if cost[0] < cost[1] {
                self.add_nullary(cost[0]);
                self.add_edge(self.src, i, cost[1].saturating_sub(cost[0]));
            } else {
                self.add_nullary(cost[1]);
                self.add_edge(i, self.dst, cost[0].saturating_sub(cost[1]));
            }
        }

        let mut flow = MaxFlow::new();
        flow.add_vertices(self.n_item + 2 + self.n_aux);
        for &(i, j, cost) in &self.edges {
            flow.add_edge(i, j, cost);
        }

        let cost = self
            .cost_0
            .saturating_add(flow.max_flow(self.src, self.dst, None));
        let mut cut = flow.min_cut();
        cut.truncate(self.n_item);

        (cost, cut)
    }

    fn add_edge(&mut self, i: usize, j: usize, cost: i64) {
        assert!(cost >= 0);
        if cost == 0 {
            return;
        }
        self.edges.push((i, j, cost));
    }
}
