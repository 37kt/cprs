use algebraic_structure::magma::TrivialGroup;
use algebraic_traits::Group;

use crate::union_find_impl::UnionFindImpl;

pub type PotentializedUnionFind<Potential> = PotentializedUnionFindBase<Potential, false>;
pub type UndoablePotentializedUnionFind<Potential> = PotentializedUnionFindBase<Potential, true>;

#[derive(Clone)]
pub struct PotentializedUnionFindBase<Potential, const UNDOABLE: bool>
where
    Potential: Group,
    Potential::Value: Clone,
{
    inner: UnionFindImpl<Potential, TrivialGroup, UNDOABLE>,
}

impl<Potential, const UNDOABLE: bool> PotentializedUnionFindBase<Potential, UNDOABLE>
where
    Potential: Group,
    Potential::Value: Clone,
{
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

    /// -p[x] + p[y] = d
    pub fn merge_with(
        &mut self,
        x: usize,
        y: usize,
        d: Potential::Value,
        f: impl FnMut(usize, usize),
    ) -> bool {
        self.inner.merge_with(x, y, d, f)
    }

    /// -p[x] + p[y] = d
    pub fn merge(&mut self, x: usize, y: usize, d: Potential::Value) -> bool {
        self.merge_with(x, y, d, |_, _| {})
    }

    pub fn root(&mut self, x: usize) -> usize {
        self.inner.root(x)
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.inner.same(x, y)
    }

    /// -p[x] + p[y]
    pub fn diff(&mut self, x: usize, y: usize) -> Option<Potential::Value> {
        self.inner.diff(x, y)
    }

    pub fn potential(&mut self, x: usize) -> Potential::Value {
        self.inner.potential(x)
    }
}

impl<Potential> PotentializedUnionFindBase<Potential, true>
where
    Potential: Group,
    Potential::Value: Clone,
{
    pub fn undo(&mut self) {
        self.inner.undo();
    }
}
