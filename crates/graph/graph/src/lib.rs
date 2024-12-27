use std::ops::Index;

/// 隣接リスト
#[derive(Clone)]
pub struct Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    vertices: Vec<V>,
    edges: Vec<(usize, E)>,
    pos: Vec<usize>,
    edge_id: Vec<usize>,
    edges_count: usize,
}

/// グリッドグラフの 4 近傍  
/// 上, 左, 下, 右
pub const GRID_NEIGHBOURS_4: &[(usize, usize)] = &[(!0, 0), (0, !0), (1, 0), (0, 1)];

/// グリッドグラフの 8 近傍  
/// 上, 左, 下, 右, 左上, 左下, 右下, 右上
pub const GRID_NEIGHBOURS_8: &[(usize, usize)] = &[
    (!0, 0),
    (0, !0),
    (1, 0),
    (0, 1),
    (!0, !0),
    (1, !0),
    (1, 1),
    (!0, 1),
];

impl<V, E> Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    /// 頂点重みと重み付き有向辺からグラフを構築する
    pub fn from_vertices_and_directed_edges(vertices: &[V], edges: &[(usize, usize, E)]) -> Self {
        if edges.is_empty() {
            return Self {
                vertices: vertices.to_vec(),
                edges: vec![],
                pos: vec![0; vertices.len() + 1],
                edge_id: vec![],
                edges_count: 0,
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
            edge_id: ord.clone(),
            edges: ord
                .into_iter()
                .map(|i| (edges[i].1, edges[i].2.clone()))
                .collect(),
            pos,
            edges_count: edges.len(),
        }
    }

    /// 頂点重みと重み付き無向辺からグラフを構築する
    pub fn from_vertices_and_undirected_edges(vertices: &[V], edges: &[(usize, usize, E)]) -> Self {
        if edges.is_empty() {
            return Self {
                vertices: vertices.to_vec(),
                edges: vec![],
                pos: vec![0; vertices.len() + 1],
                edge_id: vec![],
                edges_count: 0,
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
            edge_id: ord.iter().map(|&i| i / 2).collect(),
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
            edges_count: edges.len(),
        }
    }

    /// 頂点数を返す
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// 辺数を返す
    pub fn edges_count(&self) -> usize {
        self.edges_count
    }

    /// 頂点重みを返す
    pub fn vertex(&self, v: usize) -> &V {
        &self.vertices[v]
    }

    /// 頂点 v から出る辺を返す  
    /// g\[v\] と同じ
    pub fn edges(&self, v: usize) -> &[(usize, E)] {
        let l = self.pos[v];
        let r = self.pos[v + 1];
        &self.edges[l..r]
    }

    /// 頂点 v の i 番目の辺の id を返す  
    pub fn edge_id(&self, v: usize, i: usize) -> usize {
        self.edge_id[self.pos[v] + i]
    }

    /// グリッドからグラフを構築する
    ///
    /// # 引数
    ///
    /// * `grid` - グリッド
    /// * `neighbours` - 近傍
    /// * `cost` - grid の値から重みを計算する関数
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
    /// 重み付き有向辺からグラフを構築する
    pub fn from_directed_edges(n: usize, edges: &[(usize, usize, E)]) -> Self {
        Self::from_vertices_and_directed_edges(&vec![(); n], edges)
    }

    /// 重み付き無向辺からグラフを構築する
    pub fn from_undirected_edges(n: usize, edges: &[(usize, usize, E)]) -> Self {
        Self::from_vertices_and_undirected_edges(&vec![(); n], edges)
    }
}

impl<V> Graph<V, ()>
where
    V: Clone,
{
    /// 頂点重みと重みなし有向辺からグラフを構築する
    pub fn from_vertices_and_unweighted_directed_edges(
        vertices: &[V],
        edges: &[(usize, usize)],
    ) -> Self {
        Self::from_vertices_and_directed_edges(
            vertices,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }

    /// 頂点重みと重みなし無向辺からグラフを構築する
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
    /// 重みなし有向辺からグラフを構築する
    pub fn from_unweighted_directed_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        Self::from_directed_edges(
            n,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }

    /// 重みなし無向辺からグラフを構築する
    pub fn from_unweighted_undirected_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        Self::from_undirected_edges(
            n,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}
