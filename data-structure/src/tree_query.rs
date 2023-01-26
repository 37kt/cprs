use std::marker::PhantomData;

use ac_library_rs::{Monoid, Segtree};

use crate::HeavyLightDecomposition;

pub trait ValueOn {
    fn value_on_vartex() -> bool;
    fn value_on_edge() -> bool;
}

pub struct Vartex;
pub struct Edge;

impl ValueOn for Vartex {
    fn value_on_vartex() -> bool {
        true
    }
    fn value_on_edge() -> bool {
        false
    }
}

impl ValueOn for Edge {
    fn value_on_vartex() -> bool {
        false
    }
    fn value_on_edge() -> bool {
        true
    }
}

pub struct TreeQuery<M: Monoid, V: ValueOn> {
    n: usize,
    hld: HeavyLightDecomposition,
    seg_up: Segtree<M>,
    seg_down: Segtree<M>,
    _marker: PhantomData<fn() -> V>,
}

impl<M: Monoid, V: ValueOn> TreeQuery<M, V> {
    pub fn new(g: &Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let hld = HeavyLightDecomposition::new(g);
        Self {
            n,
            hld,
            seg_up: Segtree::<M>::new(n),
            seg_down: Segtree::<M>::new(n),
            _marker: PhantomData::default(),
        }
    }

    pub fn prod_path(&self, u: usize, v: usize) -> M::S {
        let (up, down) = self.hld.path(u, v, V::value_on_edge());
        let mut res = M::identity();
        for &(l, r) in &up {
            let t = self.seg_up.prod(self.n - r, self.n - l);
            res = M::binary_operation(&res, &t);
        }
        for &(l, r) in &down {
            let t = self.seg_down.prod(l, r);
            res = M::binary_operation(&res, &t);
        }
        res
    }

    pub fn prod_subtree(&self, v: usize) -> M::S {
        let (l, r) = self.hld.subtree(v, V::value_on_edge());
        self.seg_down.prod(l, r)
    }
}

impl<M: Monoid> TreeQuery<M, Vartex> {
    pub fn build_with_values(g: &Vec<Vec<usize>>, values: &[M::S]) -> Self {
        assert_eq!(g.len(), values.len());
        let n = g.len();
        let hld = HeavyLightDecomposition::new(g);
        let mut a = vec![M::identity(); n];
        for v in 0..n {
            let k = hld.vertex(v);
            a[k] = values[v].clone();
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
        self.seg_up.set(self.n - 1 - k, x.clone());
        self.seg_down.set(k, x);
    }

    pub fn get(&self, v: usize) -> M::S {
        let k = self.hld.vertex(v);
        self.seg_down.get(k)
    }
}

impl<M: Monoid> TreeQuery<M, Edge> {
    pub fn build_with_values(g: &Vec<Vec<(usize, M::S)>>) -> Self {
        let n = g.len();
        let mut h = vec![vec![]; n];
        for v in 0..n {
            for &(u, _) in &g[v] {
                h[v].push(u);
            }
        }
        let hld = HeavyLightDecomposition::new(&h);
        let mut a = vec![M::identity(); n];
        for v in 0..n {
            for (u, w) in &g[v] {
                let k = hld.edge(*u, v);
                a[k] = w.clone();
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
        self.seg_up.set(self.n - 1 - k, x.clone());
        self.seg_down.set(k, x);
    }

    pub fn get(&self, u: usize, v: usize) -> M::S {
        let k = self.hld.edge(u, v);
        self.seg_down.get(k)
    }
}
