use std::ops::Index;

#[derive(Clone)]
pub struct Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    vertices: Vec<V>,
    edges: Vec<(usize, E)>,
    pos: Vec<usize>,
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
    V: Clone,
    E: Clone,
{
    pub fn from_vertices_and_directed_edges(vertices: &[V], edges: &[(usize, usize, E)]) -> Self {
        if edges.is_empty() {
            return Self {
                vertices: vertices.to_vec(),
                edges: vec![],
                pos: vec![0; vertices.len() + 1],
            };
        }

        let n = vertices.len();
        let mut pos = vec![0; n + 1];
        for &(u, _, _) in edges {
            pos[u] += 1;
        }
        for i in 1..=n {
            pos[i] += pos[i - 1];
        }
        let mut ord = vec![0; edges.len()];
        for i in (0..edges.len()).rev() {
            let u = edges[i].0;
            pos[u] -= 1;
            ord[pos[u]] = i;
        }

        Self {
            vertices: vertices.to_vec(),
            edges: ord
                .into_iter()
                .map(|i| (edges[i].1, edges[i].2.clone()))
                .collect(),
            pos,
        }
    }

    pub fn from_vertices_and_undirected_edges(vertices: &[V], edges: &[(usize, usize, E)]) -> Self {
        if edges.is_empty() {
            return Self {
                vertices: vertices.to_vec(),
                edges: vec![],
                pos: vec![0; vertices.len() + 1],
            };
        }

        let n = vertices.len();
        let mut pos = vec![0; n + 1];
        for &(u, v, _) in edges {
            pos[u] += 1;
            pos[v] += 1;
        }
        for i in 1..=n {
            pos[i] += pos[i - 1];
        }
        let mut ord = vec![0; edges.len() * 2];
        for i in (0..edges.len() * 2).rev() {
            if i & 1 == 0 {
                let u = edges[i >> 1].0;
                pos[u] -= 1;
                ord[pos[u]] = i;
            } else {
                let v = edges[i >> 1].1;
                pos[v] -= 1;
                ord[pos[v]] = i;
            }
        }

        Self {
            vertices: vertices.to_vec(),
            edges: ord
                .into_iter()
                .map(|i| {
                    (
                        if i & 1 == 0 {
                            edges[i >> 1].1
                        } else {
                            edges[i >> 1].0
                        },
                        edges[i >> 1].2.clone(),
                    )
                })
                .collect(),
            pos,
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertex(&self, v: usize) -> &V {
        &self.vertices[v]
    }

    pub fn edges(&self, v: usize) -> &[(usize, E)] {
        let l = self.pos[v];
        let r = self.pos[v + 1];
        &self.edges[l..r]
    }

    /// (i, j) -> i * w + j
    pub fn from_grid(
        grid: &Vec<Vec<V>>,
        neighbours: &[(usize, usize)],
        cost: impl Fn(&V, &V) -> Option<E>,
    ) -> Self {
        let h = grid.len();
        let w = grid[0].len();
        let mut edges = vec![];
        for i in 0..h {
            for j in 0..w {
                for &(di, dj) in neighbours {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if ni >= h || nj >= w {
                        continue;
                    }
                    if let Some(c) = cost(&grid[i][j], &grid[ni][nj]) {
                        edges.push((i * w + j, ni * w + nj, c));
                    }
                }
            }
        }
        Self::from_vertices_and_directed_edges(
            &grid.into_iter().flatten().cloned().collect::<Vec<_>>(),
            &edges,
        )
    }
}

impl<V, E> Index<usize> for Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    type Output = [(usize, E)];

    fn index(&self, v: usize) -> &[(usize, E)] {
        self.edges(v)
    }
}

impl<E> Graph<(), E>
where
    E: Clone,
{
    pub fn from_directed_edges(n: usize, edges: &[(usize, usize, E)]) -> Self {
        Self::from_vertices_and_directed_edges(&vec![(); n], edges)
    }

    pub fn from_undirected_edges(n: usize, edges: &[(usize, usize, E)]) -> Self {
        Self::from_vertices_and_undirected_edges(&vec![(); n], edges)
    }
}

impl<V> Graph<V, ()>
where
    V: Clone,
{
    pub fn from_vertices_and_unweighted_directed_edges(
        vertices: &[V],
        edges: &[(usize, usize)],
    ) -> Self {
        Self::from_vertices_and_directed_edges(
            vertices,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }

    pub fn from_vertices_and_unweighted_undirected_edges(
        vertices: &[V],
        edges: &[(usize, usize)],
    ) -> Self {
        Self::from_vertices_and_undirected_edges(
            vertices,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}

impl Graph<(), ()> {
    pub fn from_unweighted_directed_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        Self::from_directed_edges(
            n,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }

    pub fn from_unweighted_undirected_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        Self::from_undirected_edges(
            n,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}
