use graph::Graph;
use strongly_connected_components::strongly_connected_components;

pub struct TwoSatisfiability {
    n: usize,
    es: Vec<(usize, usize)>,
}

impl TwoSatisfiability {
    pub fn new(n: usize) -> Self {
        Self { n, es: vec![] }
    }

    pub fn set(&mut self, x: usize) {
        if x < self.n {
            self.es.push((self.id(!x), self.id(x)));
        } else {
            self.es.push((self.id(x), self.id(!x)));
        }
    }

    pub fn add(&mut self, x: usize, y: usize) {
        self.es.push((self.id(!x), self.id(y)));
        self.es.push((self.id(!y), self.id(x)));
    }

    pub fn if_then(&mut self, x: usize, y: usize) {
        self.add(!x, y);
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let g = Graph::from_unweighted_directed_edges(self.n * 2, &self.es);
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

    fn id(&self, x: usize) -> usize {
        assert!(x < self.n || !x < self.n);
        if x < self.n {
            x
        } else {
            !x + self.n
        }
    }
}
