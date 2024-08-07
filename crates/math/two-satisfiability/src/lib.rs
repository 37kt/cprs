use graph::Graph;
use strongly_connected_components::strongly_connected_components;

pub struct TwoSatisfiability {
    n: usize,
    // g: Graph<(), ()>,
    es: Vec<(usize, usize)>,
}

impl TwoSatisfiability {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            // g: Graph::new(n * 2),
            es: vec![],
        }
    }

    pub fn set(&mut self, x: usize) {
        if x < self.n {
            // self.g.add_edge(self.id(!x), self.id(x), ());
            self.es.push((self.id(!x), self.id(x)));
        } else {
            // self.g.add_edge(self.id(x), self.id(!x), ());
            self.es.push((self.id(x), self.id(!x)));
        }
    }

    pub fn add(&mut self, x: usize, y: usize) {
        // self.g.add_edge(self.id(!x), self.id(y), ());
        // self.g.add_edge(self.id(!y), self.id(x), ());
        self.es.push((self.id(!x), self.id(y)));
        self.es.push((self.id(!y), self.id(x)));
    }

    pub fn if_then(&mut self, x: usize, y: usize) {
        self.add(!x, y);
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let g = Graph::from_unweighted_directed_edges(self.n * 2, &self.es);
        let scc = strongly_connected_components(&g);
        let mut comp = vec![0; self.n * 2];
        for i in 0..scc.len() {
            for &x in scc.vertex(i) {
                comp[x] = i;
            }
        }
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
