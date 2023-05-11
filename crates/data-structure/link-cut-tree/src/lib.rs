use std::ptr::null_mut;

use algebraic::{Act, Monoid};
use splay_tree_internal::SplayTreeNode;

pub struct LinkCutTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    nodes: Vec<SplayTreeNode<M, F>>,
}

impl<M, F> From<&[M::S]> for LinkCutTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    fn from(a: &[M::S]) -> Self {
        Self {
            nodes: a.iter().map(|x| SplayTreeNode::new(x.clone())).collect(),
        }
    }
}

impl<M, F> LinkCutTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    /// u の親を v にする
    pub fn link(&mut self, u: usize, v: usize) {
        Self::evert(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        self.nodes[u].par = &mut self.nodes[v];
    }

    pub fn cut(&mut self, u: usize, v: usize) {
        Self::evert(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        assert!(std::ptr::eq(self.nodes[u].par, &self.nodes[v]));
        self.nodes[v].lch = null_mut();
        self.nodes[u].par = null_mut();
        SplayTreeNode::update(&mut self.nodes[v]);
    }

    pub fn set(&mut self, v: usize, val: M::S) {
        let t = &mut self.nodes[v];
        Self::evert(t);
        t.val = val;
        SplayTreeNode::update(t);
    }

    pub fn get(&mut self, v: usize) -> &M::S {
        let t = &mut self.nodes[v];
        Self::evert(t);
        &t.val
    }

    pub fn apply(&mut self, u: usize, v: usize, f: F::S) {
        Self::evert(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        self.nodes[v].propagate(&f);
    }

    pub fn prod(&mut self, u: usize, v: usize) -> M::S {
        Self::evert(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        self.nodes[v].prod.clone()
    }

    fn expose(t: &mut SplayTreeNode<M, F>) -> &mut SplayTreeNode<M, F> {
        let mut rp = null_mut();
        let mut cur: *mut _ = t;
        while let Some(c) = unsafe { cur.as_mut() } {
            c.splay();
            c.rch = rp;
            SplayTreeNode::update(c);
            rp = c;
            cur = c.par;
        }
        t.splay();
        unsafe { rp.as_mut() }.unwrap()
    }

    fn evert(t: &mut SplayTreeNode<M, F>) {
        Self::expose(t);
        SplayTreeNode::toggle(t);
        SplayTreeNode::push(t);
    }
}
