#[derive(Clone)]
pub struct Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    vertices: Vec<V>,
    head: Vec<usize>,
    next: Vec<usize>,
    edges: Vec<(usize, E)>,
}

impl<V, E> Graph<V, E>
where
    V: Clone + Default,
    E: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            vertices: vec![Default::default(); n],
            head: vec![!0; n],
            next: vec![],
            edges: vec![],
        }
    }
}

impl<V, E> From<Vec<V>> for Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    fn from(vertices: Vec<V>) -> Self {
        Self {
            head: vec![!0; vertices.len()],
            vertices,
            next: vec![],
            edges: vec![],
        }
    }
}

impl<V, E> Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn set_vertex(&mut self, v: usize, w: V) {
        self.vertices[v] = w;
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: E) {
        self.next.push(self.head[u]);
        self.head[u] = self.edges.len();
        self.edges.push((v, w));
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, w: E) {
        self.add_edge(u, v, w.clone());
        self.add_edge(v, u, w);
    }

    pub fn vertices(&self) -> &[V] {
        &self.vertices
    }

    pub fn out_edges(&self, v: usize) -> impl Iterator<Item = &(usize, E)> {
        let mut e = self.head[v];
        std::iter::from_fn(move || {
            (e != !0).then(|| {
                let res = &self.edges[e];
                e = self.next[e];
                res
            })
        })
    }
}
