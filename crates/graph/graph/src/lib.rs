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

pub const GRID_NEIGHBOURS_4: &[(usize, usize)] = &[(!0, 0), (0, !0), (1, 0), (0, 1)];
pub const GRID_NEIGHBOURS_8: &[(usize, usize)] = &[
    (!0, 0),
    (0, !0),
    (1, 0),
    (0, 1),
    (!0, !0),
    (!0, 1),
    (1, !0),
    (1, 1),
];

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

    pub fn add_vertex(&mut self, w: V) -> usize {
        self.vertices.push(w);
        self.head.push(!0);
        self.vertices.len() - 1
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
            if e != !0 {
                let res = &self.edges[e];
                e = self.next[e];
                Some(res)
            } else {
                None
            }
        })
    }

    pub fn from_grid(
        grid: &Vec<Vec<V>>,
        neighbours: &[(usize, usize)],
        cost: impl Fn(&V, &V) -> Option<E>,
    ) -> Self {
        let h = grid.len();
        let w = grid[0].len();
        let mut g = Self::from(grid.into_iter().flatten().cloned().collect::<Vec<_>>());
        for i in 0..h {
            for j in 0..w {
                for &(di, dj) in neighbours {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if ni >= h || nj >= w {
                        continue;
                    }
                    if let Some(c) = cost(&grid[i][j], &grid[ni][nj]) {
                        g.add_edge(i * w + j, ni * w + nj, c);
                    }
                }
            }
        }
        g
    }
}
