use std::marker::PhantomData;

use ac_library::{Monoid, Segtree};
use graph::Graph;
use heavy_light_decomposition::HeavyLightDecomposition;

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

pub struct TreeQuery<M: Monoid, Q: QueryType> {
    n: usize,
    hld: HeavyLightDecomposition,
    seg_up: Segtree<M>,
    seg_down: Segtree<M>,
    _marker: PhantomData<fn() -> Q>,
}

impl<M, Q> TreeQuery<M, Q>
where
    M: Monoid,
    Q: QueryType,
{
    pub fn prod_path(&self, u: usize, v: usize) -> M::S {
        let (up, down) = self.hld.path(u, v, Q::edge());
        let mut res = M::identity();
        for &(l, r) in &up {
            let t = self.seg_up.prod(self.n - r..self.n - l);
            res = M::binary_operation(&res, &t);
        }
        for &(l, r) in &down {
            let t = self.seg_down.prod(l..r);
            res = M::binary_operation(&res, &t);
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
    V: Copy,
    M: Monoid<S = V>,
{
    pub fn build<E>(g: &Graph<V, E>) -> Self
    where
        E: Copy,
    {
        let n = g.size();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::identity(); n];
        for v in 0..n {
            let k = hld.vertex(v);
            a[k] = g.vertex(v);
        }
        let seg_down = Segtree::from(a.clone());
        a.reverse();
        let seg_up = Segtree::from(a);
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
        self.seg_up.set(self.n - 1 - k, x);
        self.seg_down.set(k, x);
    }

    pub fn get(&self, v: usize) -> M::S {
        let k = self.hld.vertex(v);
        self.seg_down.get(k)
    }
}

impl<E, M> TreeQuery<M, Edge>
where
    E: Copy,
    M: Monoid<S = E>,
{
    pub fn build<V>(g: &Graph<V, E>) -> Self
    where
        V: Copy,
    {
        let n = g.size();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::identity(); n];
        for v in 0..n {
            for &(u, w) in g.out_edges(v) {
                let k = hld.edge(u, v);
                a[k] = w;
            }
        }
        let seg_down = Segtree::from(a.clone());
        a.reverse();
        let seg_up = Segtree::from(a);
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
        self.seg_up.set(self.n - 1 - k, x);
        self.seg_down.set(k, x);
    }

    pub fn get(&self, u: usize, v: usize) -> M::S {
        let k = self.hld.edge(u, v);
        self.seg_down.get(k)
    }
}
