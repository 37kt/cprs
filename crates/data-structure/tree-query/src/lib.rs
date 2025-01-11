use std::marker::PhantomData;

use algebraic::Monoid;
use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;
use segment_tree::SegmentTree;

/// 頂点クエリを処理するためのデータ構造。
pub type TreeQueryVertex<M> = TreeQuery<M, Vertex>;

/// 辺クエリを処理するためのデータ構造。
pub type TreeQueryEdge<M> = TreeQuery<M, Edge>;

pub trait QueryType {
    fn vertex() -> bool;
    fn edge() -> bool;
}

pub enum Vertex {}
pub enum Edge {}

impl QueryType for Vertex {
    fn vertex() -> bool {
        true
    }
    fn edge() -> bool {
        false
    }
}

impl QueryType for Edge {
    fn vertex() -> bool {
        false
    }
    fn edge() -> bool {
        true
    }
}

pub struct TreeQuery<M, Q>
where
    M: Monoid,
    M::S: Clone,
    Q: QueryType,
{
    n: usize,
    hld: HeavyLightDecomposition,
    seg_up: SegmentTree<M>,
    seg_down: SegmentTree<M>,
    _marker: PhantomData<fn() -> Q>,
}

impl<M, Q> TreeQuery<M, Q>
where
    M: Monoid,
    M::S: Clone,
    Q: QueryType,
{
    /// 頂点 u から頂点 v のパスに対するクエリを処理する。
    ///
    /// # 引数
    ///
    /// - `u`: 頂点 u
    /// - `v`: 頂点 v
    ///
    /// # 戻り値
    ///
    /// - パス (u, v) 上の頂点 (もしくは辺) の総積
    ///
    /// # 計算量
    ///
    /// O(log^2 N)
    pub fn prod_path(&self, u: usize, v: usize) -> M::S {
        let (up, down) = self.hld.path(u, v, Q::edge());
        let mut res = M::e();
        for &(l, r) in &up {
            let t = self.seg_up.prod(self.n - r..self.n - l);
            res = M::op(&res, &t);
        }
        for &(l, r) in &down {
            let t = self.seg_down.prod(l..r);
            res = M::op(&res, &t);
        }
        res
    }

    /// 頂点 v を根とする部分木に対するクエリを処理する。
    ///
    /// # 引数
    ///
    /// - `v`: 頂点 v
    ///
    /// # 戻り値
    ///
    /// - 部分木 v の頂点 (もしくは辺) の総積
    ///
    /// # 計算量
    ///
    /// O(log^2 N)
    pub fn prod_subtree(&self, v: usize) -> M::S {
        let (l, r) = self.hld.subtree(v, Q::edge());
        self.seg_down.prod(l..r)
    }
}

impl<V, M> TreeQuery<M, Vertex>
where
    V: Clone,
    M: Monoid<S = V>,
{
    /// グラフから TreeQuery を構築する。
    ///
    /// # 引数
    ///
    /// - `g`: グラフ
    ///
    /// # 計算量
    ///
    /// O(N)
    pub fn build<E>(g: &UndirectedGraph<V, E>) -> Self
    where
        E: Clone,
    {
        let n = g.len();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::e(); n];
        for v in 0..n {
            let k = hld.vertex(v);
            a[k] = g.vertex(v).clone();
        }
        let seg_down = SegmentTree::from(a.clone());
        a.reverse();
        let seg_up = SegmentTree::from(a);
        Self {
            n,
            hld,
            seg_up,
            seg_down,
            _marker: PhantomData::default(),
        }
    }

    /// 頂点 v の値を x に変更する。
    ///
    /// # 引数
    ///
    /// - `v`: 頂点 v
    /// - `x`: 新しい値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn set(&mut self, v: usize, x: M::S) {
        let k = self.hld.vertex(v);
        self.seg_up.set(self.n - 1 - k, x.clone());
        self.seg_down.set(k, x);
    }

    /// 頂点 v の値を取得する。
    ///
    /// # 引数
    ///
    /// - `v`: 頂点 v
    ///
    /// # 戻り値
    ///
    /// - 頂点 v の値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn get(&self, v: usize) -> M::S {
        let k = self.hld.vertex(v);
        self.seg_down.get(k)
    }
}

impl<E, M> TreeQuery<M, Edge>
where
    E: Clone,
    M: Monoid<S = E>,
{
    /// グラフから TreeQuery を構築する。
    ///
    /// # 引数
    ///
    /// - `g`: グラフ
    ///
    /// # 計算量
    ///
    /// O(N)
    pub fn build<V>(g: &UndirectedGraph<V, E>) -> Self
    where
        V: Clone,
    {
        let n = g.len();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::e(); n];
        for v in 0..n {
            for (u, w) in &g[v] {
                let k = hld.edge(*u, v);
                a[k] = w.clone();
            }
        }
        let seg_down = SegmentTree::from(a.clone());
        a.reverse();
        let seg_up = SegmentTree::from(a);
        Self {
            n,
            hld,
            seg_up,
            seg_down,
            _marker: PhantomData::default(),
        }
    }

    /// 辺 (u, v) の値を x に変更する。
    ///
    /// # 引数
    ///
    /// - `u`: 頂点 u
    /// - `v`: 頂点 v
    /// - `x`: 新しい値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn set(&mut self, u: usize, v: usize, x: M::S) {
        let k = self.hld.edge(u, v);
        self.seg_up.set(self.n - 1 - k, x.clone());
        self.seg_down.set(k, x);
    }

    /// 辺 (u, v) の値を取得する。
    ///
    /// # 引数
    ///
    /// - `u`: 頂点 u
    /// - `v`: 頂点 v
    ///
    /// # 戻り値
    ///
    /// - 辺 (u, v) の値
    ///
    /// # 計算量
    ///
    /// O(log N)
    pub fn get(&self, u: usize, v: usize) -> M::S {
        let k = self.hld.edge(u, v);
        self.seg_down.get(k)
    }
}
