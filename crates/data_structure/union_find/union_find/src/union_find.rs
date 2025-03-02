use algebraic_structure::magma::TrivialGroup;

use crate::union_find_base::UnionFindBase;

#[derive(Clone)]
pub struct UnionFind {
    inner: UnionFindBase<TrivialGroup, TrivialGroup, false>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            inner: UnionFindBase::new(n),
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
