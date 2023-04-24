#[derive(Clone)]
pub struct Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    vs: Vec<V>,
    es: Vec<Vec<(usize, E)>>,
}

impl<V, E> Graph<V, E>
where
    V: Clone + Default,
    E: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            vs: vec![Default::default(); n],
            es: vec![vec![]; n],
        }
    }
}

impl<V, E> From<Vec<V>> for Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    fn from(vs: Vec<V>) -> Self {
        Self {
            es: vec![vec![]; vs.len()],
            vs,
        }
    }
}

impl<V, E> Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    pub fn size(&self) -> usize {
        self.vs.len()
    }

    pub fn set_vertex(&mut self, v: usize, w: V) {
        self.vs[v] = w;
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: E) {
        self.es[u].push((v, w));
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, w: E) {
        self.add_edge(u, v, w.clone());
        self.add_edge(v, u, w);
    }

    pub fn vertex(&self, v: usize) -> &V {
        &self.vs[v]
    }

    pub fn out_edges(&self, v: usize) -> &Vec<(usize, E)> {
        &self.es[v]
    }
}
