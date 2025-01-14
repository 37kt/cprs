use std::ops::Index;

use csr_array::CSRArray;

/// 隣接リスト
#[derive(Clone)]
pub struct Graph<V, E, const DIRECTED: bool>
where
    V: Clone,
    E: Clone,
{
    vertices: Vec<V>,
    edges: CSRArray<(usize, E)>,
}

pub type DirectedGraph<V, E> = Graph<V, E, true>;
pub type UndirectedGraph<V, E> = Graph<V, E, false>;

/// グリッドグラフの 4 近傍  
/// 上, 左, 下, 右
pub const GRID_NEIGHBOURS_4: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];

/// グリッドグラフの 8 近傍  
/// 上, 左, 下, 右, 左上, 左下, 右下, 右上
pub const GRID_NEIGHBOURS_8: [(usize, usize); 8] = [
    (!0, 0),
    (0, !0),
    (1, 0),
    (0, 1),
    (!0, !0),
    (1, !0),
    (1, 1),
    (!0, 1),
];

impl<V, E> DirectedGraph<V, E>
where
    V: Clone,
    E: Clone,
{
    /// 頂点重みと重み付き有向辺からグラフを構築する
    pub fn from_vertices_and_edges(vertices: &[V], edges: &[(usize, usize, E)]) -> Self {
        let edges = edges
            .iter()
            .map(|(u, v, w)| (*u, (*v, w.clone())))
            .collect::<Vec<_>>();

        Self {
            vertices: vertices.to_vec(),
            edges: CSRArray::new(vertices.len(), &edges),
        }
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
        Self::from_vertices_and_edges(
            &grid.into_iter().flatten().cloned().collect::<Vec<_>>(),
            &edges,
        )
    }
}

impl<V, E> UndirectedGraph<V, E>
where
    V: Clone,
    E: Clone,
{
    /// 頂点重みと重み付き無向辺からグラフを構築する
    pub fn from_vertices_and_edges(vertices: &[V], edges: &[(usize, usize, E)]) -> Self {
        let edges = edges
            .iter()
            .map(|(u, v, w)| [(*u, (*v, w.clone())), (*v, (*u, w.clone()))])
            .flatten()
            .collect::<Vec<_>>();

        Self {
            vertices: vertices.to_vec(),
            edges: CSRArray::new(vertices.len(), &edges),
        }
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
                    let u = i * w + j;
                    let v = ni * w + nj;
                    if u > v {
                        continue;
                    }
                    if let Some(c) = cost(&grid[i][j], &grid[ni][nj]) {
                        edges.push((u, v, c));
                    }
                }
            }
        }
        Self::from_vertices_and_edges(
            &grid.into_iter().flatten().cloned().collect::<Vec<_>>(),
            &edges,
        )
    }
}

impl<V, E, const DIRECTED: bool> Graph<V, E, DIRECTED>
where
    V: Clone,
    E: Clone,
{
    /// 頂点数を返す
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// 頂点重みを返す
    pub fn vertex(&self, v: usize) -> &V {
        &self.vertices[v]
    }

    /// 頂点 v から出る辺を返す  
    /// g\[v\] と同じ
    pub fn edges(&self, v: usize) -> &[(usize, E)] {
        &self.edges[v]
    }
}

impl<V, E, const DIRECTED: bool> Index<usize> for Graph<V, E, DIRECTED>
where
    V: Clone,
    E: Clone,
{
    type Output = [(usize, E)];

    fn index(&self, v: usize) -> &[(usize, E)] {
        self.edges(v)
    }
}

impl<V> DirectedGraph<V, ()>
where
    V: Clone,
{
    /// 頂点重みと重みなし有向辺からグラフを構築する
    pub fn from_vertices_and_unweighted_edges(vertices: &[V], edges: &[(usize, usize)]) -> Self {
        Self::from_vertices_and_edges(
            vertices,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}

impl<V> UndirectedGraph<V, ()>
where
    V: Clone,
{
    /// 頂点重みと重みなし無向辺からグラフを構築する
    pub fn from_vertices_and_unweighted_edges(vertices: &[V], edges: &[(usize, usize)]) -> Self {
        Self::from_vertices_and_edges(
            vertices,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}

impl<E> DirectedGraph<(), E>
where
    E: Clone,
{
    /// 重み付き有向辺からグラフを構築する
    pub fn from_edges(n: usize, edges: &[(usize, usize, E)]) -> Self {
        Self::from_vertices_and_edges(&vec![(); n], edges)
    }
}

impl<E> UndirectedGraph<(), E>
where
    E: Clone,
{
    /// 重み付き無向辺からグラフを構築する
    pub fn from_edges(n: usize, edges: &[(usize, usize, E)]) -> Self {
        Self::from_vertices_and_edges(&vec![(); n], edges)
    }
}

impl DirectedGraph<(), ()> {
    /// 重みなし有向辺からグラフを構築する
    pub fn from_unweighted_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        Self::from_edges(
            n,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}

impl UndirectedGraph<(), ()> {
    /// 重みなし無向辺からグラフを構築する
    pub fn from_unweighted_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        Self::from_edges(
            n,
            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),
        )
    }
}
