use algebraic_structure::magma::TrivialGroup;
use algebraic_traits::{Commutative, Semigroup, Unital};

use crate::union_find_impl::UnionFindImpl;

pub type UnionFindComponentSum<Aggregate> = UnionFindComponentSumBase<Aggregate, false>;
pub type UndoableUnionFindComponentSum<Aggregate> = UnionFindComponentSumBase<Aggregate, true>;

#[derive(Clone)]
pub struct UnionFindComponentSumBase<Aggregate, const UNDOABLE: bool>
where
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    inner: UnionFindImpl<TrivialGroup, Aggregate, UNDOABLE>,
}

impl<Aggregate, const UNDOABLE: bool> UnionFindComponentSumBase<Aggregate, UNDOABLE>
where
    Aggregate: Commutative + Semigroup + Unital,
    Aggregate::Value: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            inner: UnionFindImpl::new(n),
        }
    }
}

impl<Aggregate, const UNDOABLE: bool> UnionFindComponentSumBase<Aggregate, UNDOABLE>
where
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> Aggregate::Value) -> Self {
        Self {
            inner: UnionFindImpl::from_fn(n, f),
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

    pub fn component_sum(&mut self, x: usize) -> Aggregate::Value {
        self.inner.component_sum(x)
    }
}

impl<Aggregate> UnionFindComponentSumBase<Aggregate, true>
where
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    pub fn undo(&mut self) {
        self.inner.undo();
    }
}
