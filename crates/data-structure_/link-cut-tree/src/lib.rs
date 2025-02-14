use std::ptr::null_mut;

use algebraic::{Act, Monoid};
use splay_tree_internal::SplayTreeNode;

/// Link-Cut Tree
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
    /// 頂点の値を a で初期化する。
    /// 辺は存在しない。
    fn from(a: &[M::S]) -> Self {
        Self {
            nodes: a
                .iter()
                .enumerate()
                .map(|(i, x)| SplayTreeNode::new(x.clone(), i))
                .collect(),
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
    /// 頂点 u の親を頂点 v にする。
    pub fn link(&mut self, u: usize, v: usize) {
        Self::evert_(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        self.nodes[u].par = &mut self.nodes[v];
    }

    /// 頂点 u と頂点 v の間の辺を削除する。
    pub fn cut(&mut self, u: usize, v: usize) {
        Self::evert_(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        assert!(std::ptr::eq(self.nodes[u].par, &self.nodes[v]));
        self.nodes[v].lch = null_mut();
        self.nodes[u].par = null_mut();
        SplayTreeNode::update(&mut self.nodes[v]);
    }

    /// 頂点 v の値を val に更新する。
    pub fn set(&mut self, v: usize, val: M::S) {
        let t = &mut self.nodes[v];
        Self::evert_(t);
        t.val = val;
        SplayTreeNode::update(t);
    }

    /// 頂点 v の値を取得する。
    pub fn get(&mut self, v: usize) -> &M::S {
        let t = &mut self.nodes[v];
        Self::evert_(t);
        &t.val
    }

    /// パス (u, v) 上の頂点すべてに f を作用させる。
    pub fn apply(&mut self, u: usize, v: usize, f: F::S) {
        Self::evert_(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        self.nodes[v].propagate(&f);
    }

    /// パス (u, v) 上の頂点すべての値のモノイド積を取得する。
    pub fn prod(&mut self, u: usize, v: usize) -> M::S {
        Self::evert_(&mut self.nodes[u]);
        Self::expose(&mut self.nodes[v]);
        self.nodes[v].prod.clone()
    }

    /// 頂点 v を根にする。
    pub fn evert(&mut self, v: usize) {
        Self::evert_(&mut self.nodes[v]);
    }

    /// 頂点 v が含まれる木の根の index を取得する。
    pub fn root(&mut self, v: usize) -> usize {
        let mut t = &mut self.nodes[v];
        Self::expose(t);
        while let Some(x) = unsafe { t.lch.as_mut() } {
            SplayTreeNode::push(t);
            t = x;
        }
        t.idx
    }

    /// 頂点 u と頂点 v の lca を取得する。
    pub fn lca(&mut self, u: usize, v: usize) -> Option<usize> {
        if self.root(u) != self.root(v) {
            None
        } else {
            Self::expose(&mut self.nodes[u]);
            Some(Self::expose(&mut self.nodes[v]).idx)
        }
    }

    /// 頂点 v の親を取得する。
    pub fn parent(&mut self, v: usize) -> Option<usize> {
        Self::expose(&mut self.nodes[v]);
        if let Some(mut p) = unsafe { self.nodes[v].lch.as_mut() } {
            loop {
                SplayTreeNode::push(p);
                if p.rch.is_null() {
                    return Some(p.idx);
                }
                p = unsafe { p.rch.as_mut() }.unwrap();
            }
        } else {
            None
        }
    }

    /// 頂点 v の k 個上の親を取得する。
    pub fn kth_parent(&mut self, v: usize, mut k: usize) -> Option<usize> {
        let t = &mut self.nodes[v];
        Self::expose(t);
        let mut t: *mut _ = t;
        while let Some(x) = unsafe { t.as_mut() } {
            SplayTreeNode::push(x);
            if !x.rch.is_null() && unsafe { x.rch.as_mut() }.unwrap().len > k {
                t = x.rch;
            } else {
                if !x.rch.is_null() {
                    k -= unsafe { x.rch.as_mut() }.unwrap().len;
                }
                if k == 0 {
                    return Some(x.idx);
                }
                k -= 1;
                t = x.lch;
            }
        }
        None
    }

    /// 根から頂点 t までのパスを Preferred Edge からなるパスにする。
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

    /// 頂点 t を根にする。
    fn evert_(t: &mut SplayTreeNode<M, F>) {
        Self::expose(t);
        SplayTreeNode::toggle(t);
        SplayTreeNode::push(t);
    }
}
