use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
};

use algebraic::{Act, Monoid};
use splay_tree_internal::SplayTreeNode;

pub struct SplayTree<M, F>(*mut SplayTreeNode<M, F>)
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone;

impl<M, F> From<&[M::S]> for SplayTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    fn from(a: &[M::S]) -> Self {
        Self(SplayTreeNode::build(a))
    }
}

impl<M, F> SplayTree<M, F>
where
    M: Monoid,
    M::S: Clone + Debug,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    pub fn dump(&self) {
        eprint!("[");
        Self::dump_dfs(unsafe { self.0.as_ref() }.unwrap());
        eprintln!("]");
    }

    fn dump_dfs(t: &SplayTreeNode<M, F>) {
        if let Some(l) = unsafe { t.lch.as_ref() } {
            Self::dump_dfs(l);
        }
        eprint!("{:?}, ", &t.val);
        if let Some(r) = unsafe { t.rch.as_ref() } {
            Self::dump_dfs(r);
        }
    }

    pub fn set(&mut self, k: usize, val: M::S) {
        self.0 = SplayTreeNode::access(unsafe { self.0.as_mut() }.unwrap(), k);
        unsafe { self.0.as_mut() }.unwrap().val = val;
        SplayTreeNode::update(self.0);
    }

    pub fn get(&mut self, k: usize) -> &M::S {
        self.0 = SplayTreeNode::access(unsafe { self.0.as_mut() }.unwrap(), k);
        &unsafe { self.0.as_ref() }.unwrap().val
    }

    pub fn insert(&mut self, k: usize, val: M::S) {
        SplayTreeNode::insert(&mut self.0, k, val);
    }

    pub fn remove(&mut self, k: usize) -> M::S {
        SplayTreeNode::remove(&mut self.0, k)
    }

    pub fn merge(&mut self, r: Self) {
        self.0 = SplayTreeNode::merge(self.0, r.0);
    }

    pub fn split(self, k: usize) -> (Self, Self) {
        let (x, y) = SplayTreeNode::split(self.0, k);
        (Self(x), Self(y))
    }

    pub fn apply(&mut self, range: impl RangeBounds<usize>, f: F::S) {
        let (l, r) = self.range_to_pair(range);
        if l == r {
            return;
        }
        let (x, y) = SplayTreeNode::split(self.0, l);
        let (y, z) = SplayTreeNode::split(y, r - l);
        SplayTreeNode::propagate(unsafe { y.as_mut() }.unwrap(), &f);
        self.0 = SplayTreeNode::merge(x, SplayTreeNode::merge(y, z));
    }

    pub fn reverse(&mut self, range: impl RangeBounds<usize>) {
        let (l, r) = self.range_to_pair(range);
        if l == r {
            return;
        }
        let (x, y) = SplayTreeNode::split(self.0, l);
        let (y, z) = SplayTreeNode::split(y, r - l);
        unsafe { y.as_mut() }.unwrap().toggle();
        self.0 = SplayTreeNode::merge(x, SplayTreeNode::merge(y, z));
    }

    pub fn prod(&mut self, range: impl RangeBounds<usize>) -> M::S {
        let (l, r) = self.range_to_pair(range);
        let (x, y) = SplayTreeNode::split(self.0, l);
        let (y, z) = SplayTreeNode::split(y, r - l);
        let res = unsafe { y.as_mut() }.map_or(M::e(), |v| v.prod.clone());
        self.0 = SplayTreeNode::merge(x, SplayTreeNode::merge(y, z));
        res
    }

    fn range_to_pair(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let l = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&r) => r + 1,
            Bound::Excluded(&r) => r,
            Bound::Unbounded => SplayTreeNode::len(self.0),
        };
        (l, r)
    }
}

fn deep_free<M, F>(t: *mut SplayTreeNode<M, F>)
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    if let Some(t) = unsafe { t.as_mut() } {
        deep_free(t.lch);
        deep_free(t.rch);
        std::mem::drop(t);
    }
}

impl<M, F> Drop for SplayTree<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    fn drop(&mut self) {
        deep_free(self.0)
    }
}
