use std::borrow::Borrow;

use max_flow::MaxFlow;

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
    /// 新しい MultivaluedOptimization のインスタンスを生成するコンストラクタ。
    ///
    /// # 引数
    /// - `n_options`: 各変数の選択肢の数を表す値のイテレータ。
    ///
    /// # 戻り値
    /// 新しく初期化された MultivaluedOptimization のインスタンス。
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
                let mut id = vec![!0; n];
                for i in 1..n {
                    id[i] = cur;
                    cur += 1;
                }
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
                opt.add_edge(opt.id[i][j + 1], opt.id[i][j], i64::MAX);
            }
        }

        opt
    }

    /// 定数を加算する  
    ///
    /// # 制約
    /// - `|cost| <= 10^9`
    ///
    /// # 引数
    /// - `cost`: 加算する定数項のコスト。
    pub fn add_nullary(&mut self, cost: i64) {
        self.cost_0 += cost;
    }

    /// 1 変数関数を加算する  
    /// +∞ は None で表す  
    ///
    /// # 制約
    /// - 有限値が連続する区間がちょうど 1 つ存在する  
    /// - `|cost| <= 10^9` or `cost = None`
    ///
    /// # 引数
    /// - `i`: 対象の変数のインデックス。
    /// - `cost`: 各選択肢に対するコストを返すクロージャ。Noneの場合は+∞とみなす。
    pub fn add_unary(&mut self, i: usize, mut cost: impl FnMut(usize) -> Option<i64>) {
        let cost = (0..self.n_options[i])
            .map(|mi| cost(mi))
            .collect::<Vec<_>>();

        let mut appeared_finite_value = false;
        if let Some(x0) = cost[0] {
            self.add_nullary(x0);
            appeared_finite_value = true;
        }

        for mi in 1..self.n_options[i] {
            match (cost[mi - 1], cost[mi]) {
                (None, None) => {}
                (None, Some(x1)) => {
                    assert!(
                        !appeared_finite_value,
                        "Finite cost values must form a contiguous segment"
                    );
                    appeared_finite_value = true;
                    self.add_nullary(x1);
                    self.add_edge(self.src, self.id[i][mi], i64::MAX);
                }
                (Some(_), None) => {
                    self.add_edge(self.id[i][mi], self.dst, i64::MAX);
                }
                (Some(x0), Some(x1)) => {
                    let diff = x1 - x0;
                    if diff < 0 {
                        self.add_nullary(diff);
                        self.add_edge(self.src, self.id[i][mi], -diff);
                    } else {
                        self.add_edge(self.id[i][mi], self.dst, diff);
                    }
                }
            }
        }

        assert!(
            appeared_finite_value,
            "There must be at least one finite cost value"
        );
    }

    /// 2 変数関数を加算する  
    /// +∞ は None で表す  
    ///
    /// # 制約
    /// - `|cost| <= 10^9` or `cost = None`  
    /// - `cost[0][0]` から `cost[h-1][w-1]` までが有限値によって連結である  
    /// - 行番号が増える方向について、有限値をとる区間の端点の位置が広義単調増加である  
    ///
    /// # 引数
    /// - `i`, `j`: 対象の2変数関数に適用する変数のインデックス。
    /// - `cost`: 各組合せに対するコストを返すクロージャ。Noneの場合は+∞とみなす。
    pub fn add_binary(
        &mut self,
        i: usize,
        j: usize,
        mut cost: impl FnMut(usize, usize) -> Option<i64>,
    ) {
        let h = self.n_options[i];
        let w = self.n_options[j];
        let mut cost = (0..h)
            .map(|mi| {
                (0..w)
                    .map(|mj| cost(mi, mj).unwrap_or(i64::MAX))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // 各行の有限値の区間を求める
        let mut l = vec![w; h];
        let mut r = vec![0; h];
        for i in 0..h {
            for j in 0..w {
                if cost[i][j] != i64::MAX {
                    l[i] = l[i].min(j);
                    r[i] = r[i].max(j + 1);
                }
            }
            assert!(
                l[i] < r[i],
                "Each row must contain at least one finite cost value"
            );
            assert!(
                (l[i]..r[i]).all(|j| cost[i][j] != i64::MAX),
                "Finite cost values in each row must appear in a contiguous block"
            );
        }

        // 左上と右下が有限値によって連結であることを確認する
        // また、有限値の区間が下の行にいくにつれて右に移動していくことを確認する
        let mut is_connected = true;
        is_connected &= l[0] <= 0 && 0 < r[0];
        for i in 1..h {
            is_connected &= l[i - 1] <= l[i] && l[i] < r[i - 1] && r[i - 1] <= r[i];
        }
        is_connected &= l[h - 1] <= w - 1 && w - 1 < r[h - 1];
        assert!(
            is_connected,
            "Finite cost values in the matrix must form a single connected region that spans from cost[0][0] to cost[h-1][w-1]"
        );

        // monge 性を満たすように無効値を埋める
        // ただし、値は INF 以上とする
        // INF は O(K) 倍してもオーバーフローしない程度の大きな値
        const INF: i64 = 1 << 40;
        for i in (0..h - 1).rev() {
            for j in r[i]..w {
                cost[i][j] = INF.max(cost[i][j - 1] + cost[i + 1][j] - cost[i + 1][j - 1]);
            }
        }
        for i in 1..h {
            for j in (0..l[i]).rev() {
                cost[i][j] = INF.max(cost[i - 1][j] + cost[i][j + 1] - cost[i - 1][j + 1]);
            }
        }

        // 1 変数関数 θ_j(mj) = φ_ij(0, mj) をフローに追加し、φ から引く
        self.add_unary(j, |mj| Some(cost[0][mj]));

        // 1 変数関数 θ_i(mi) = φ_ij(mi, w-1) をフローに追加し、φ から引く
        self.add_unary(i, |mi| Some(cost[mi][w - 1] - cost[0][w - 1]));

        // 2 変数関数 φ_ij(mi, mj) をフローに追加する
        for mi in 1..h {
            for mj in 1..w {
                let x = -cost[mi - 1][mj - 1] + cost[mi - 1][mj] + cost[mi][mj - 1] - cost[mi][mj];
                assert!(x >= 0, "The Monge property is violated");
                self.add_edge(self.id[i][mi], self.id[j][mj], x);
            }
        }
    }

    /// 最適化問題を解いて、最小コストと各変数の選択値を返す。
    ///
    /// # 戻り値
    /// - タプル (最小コスト, 各変数に対する選択されたインデックス)。
    pub fn solve(&self) -> (i64, Vec<usize>) {
        let mut flow = MaxFlow::new();
        flow.add_vertices(self.n_vertices);
        for &(i, j, cost) in &self.edges {
            flow.add_edge(i, j, cost);
        }

        let cost = self.cost_0 + flow.max_flow(self.src, self.dst, None);
        let cut = flow.min_cut();

        let mut choice = vec![0; self.n_options.len()];
        for i in 0..self.n_options.len() {
            for j in 1..self.n_options[i] {
                if cut[self.id[i][j]] == 0 {
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
