use algebraic_structure::magma::TrivialGroup;

use crate::union_find_impl::UnionFindImpl;

pub type UnionFind = UnionFindBase<false>;
pub type UndoableUnionFind = UnionFindBase<true>;

#[derive(Clone)]
pub struct UnionFindBase<const UNDOABLE: bool> {
    inner: UnionFindImpl<TrivialGroup, TrivialGroup, UNDOABLE>,
}

impl<const UNDOABLE: bool> UnionFindBase<UNDOABLE> {
    pub fn new(n: usize) -> Self {
        Self {
            inner: UnionFindImpl::new(n),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn merge_with(&mut self, x: usize, y: usize, f: impl FnMut(usize, usize)) -> bool {
        self.inner.merge_with(x, y, (), f)
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        self.merge_with(x, y, |_, _| {})
    }

    pub fn root(&mut self, x: usize) -> usize {
        self.inner.root(x)
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.inner.same(x, y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        self.inner.size(x)
    }
}

impl UnionFindBase<true> {
    pub fn undo(&mut self) {
        self.inner.undo();
    }
}
