use std::marker::PhantomData;

use algebraic::Monoid;
use graph::Graph;
use heavy_light_decomposition::HeavyLightDecomposition;
use segment_tree::SegmentTree;

pub type TreeQueryVertex<M> = TreeQuery<M, Vertex>;
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
    pub fn build<E>(g: &Graph<V, E>) -> Self
    where
        E: Clone,
    {
        let n = g.size();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::e(); n];
        for v in 0..n {
            let k = hld.vertex(v);
            a[k] = g.vertices()[v].clone();
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

    pub fn set(&mut self, v: usize, x: M::S) {
        let k = self.hld.vertex(v);
        self.seg_up.set(self.n - 1 - k, x.clone());
        self.seg_down.set(k, x);
    }

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
    pub fn build<V>(g: &Graph<V, E>) -> Self
    where
        V: Clone,
    {
        let n = g.size();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::e(); n];
        for v in 0..n {
            for (u, w) in g.out_edges(v) {
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

    pub fn set(&mut self, u: usize, v: usize, x: M::S) {
        let k = self.hld.edge(u, v);
        self.seg_up.set(self.n - 1 - k, x.clone());
        self.seg_down.set(k, x);
    }

    pub fn get(&self, u: usize, v: usize) -> M::S {
        let k = self.hld.edge(u, v);
        self.seg_down.get(k)
    }
}
