use graph::DirectedGraph;
use strongly_connected_components::strongly_connected_components;

/// 2-SAT
pub struct TwoSatisfiability {
    n: usize,
    es: Vec<(usize, usize)>,
}

impl TwoSatisfiability {
    /// n 個の変数を持つ 2-SAT を初期化する
    pub fn new(n: usize) -> Self {
        Self { n, es: vec![] }
    }

    /// 条件 x_i = f を追加する
    pub fn set(&mut self, i: usize, f: bool) {
        self.es.push((self.id(i, !f), self.id(i, f)));
    }

    /// 条件 x_i = f -> x_j = g を追加する
    pub fn if_then(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.or(i, !f, j, g);
    }

    /// 条件 x_i = f ∨ x_j = g を追加する
    pub fn or(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.es.push((self.id(i, !f), self.id(j, g)));
        self.es.push((self.id(j, !g), self.id(i, f)));
    }

    /// 条件 ￢(x_i = f ∧ x_j = g) を追加する
    pub fn nand(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.or(i, !f, j, !g);
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let g = DirectedGraph::from_unweighted_edges(self.n * 2, &self.es);
        let (_, comp) = strongly_connected_components(&g);
        let mut res = vec![false; self.n];
        for i in 0..self.n {
            if comp[i] == comp[i + self.n] {
                return None;
            }
            res[i] = comp[i] > comp[i + self.n];
        }
        Some(res)
    }

    fn id(&self, i: usize, f: bool) -> usize {
        assert!(i < self.n);
        if f {
            i
        } else {
            i + self.n
        }
    }
}
