use algebraic_structure::magma::TrivialGroup;
use algebraic_traits::Group;

use crate::union_find_base::UnionFindBase;

#[derive(Clone)]
pub struct PotentializedUnionFind<Potential>
where
    Potential: Group,
    Potential::Value: Clone + Eq,
{
    inner: UnionFindBase<Potential, TrivialGroup, false>,
}

impl<Potential> PotentializedUnionFind<Potential>
where
    Potential: Group,
    Potential::Value: Clone + Eq,
{
    pub fn new(n: usize) -> Self {
        Self {
            inner: UnionFindBase::new(n),
        }
    }

    pub fn merge_with(
        &mut self,
        x: usize,
        y: usize,
        d: Potential::Value,
        f: impl FnMut(usize, usize),
    ) -> bool {
        self.inner.merge_with(x, y, d, f)
    }

    pub fn merge(&mut self, x: usize, y: usize, d: Potential::Value) -> bool {
        self.merge_with(x, y, d, |_, _| {})
    }

    pub fn root(&mut self, x: usize) -> usize {
        self.inner.root(x)
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.inner.same(x, y)
    }

    pub fn diff(&mut self, x: usize, y: usize) -> Option<Potential::Value> {
        self.inner.diff(x, y)
    }
}
