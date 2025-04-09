use crate::{operator::Operator, tree::Tree, Actable, Foldable};

pub struct SplaySequence<O>
where
    O: Operator,
{
    tree: Tree<O>,
}

impl<O> SplaySequence<O>
where
    O: Operator,
    O::X: Default,
{
    pub fn new(n: usize) -> Self {
        Self { tree: Tree::new(n) }
    }
}

impl<O> Default for SplaySequence<O>
where
    O: Operator,
    O::X: Default,
{
    fn default() -> Self {
        Self::new(0)
    }
}

impl<O> SplaySequence<O>
where
    O: Operator,
{
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> O::X) -> Self {
        Self {
            tree: Tree::from_fn(n, f),
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn get(&mut self, i: usize) -> Option<&O::X> {
        self.tree.get(i)
    }

    pub fn set(&mut self, i: usize, x: O::X) {
        self.tree.set(i, x);
    }

    pub fn insert(&mut self, i: usize, x: O::X) {
        self.tree.insert(i, x);
    }

    pub fn remove(&mut self, i: usize) -> O::X {
        self.tree.remove(i)
    }

    pub fn reverse(&mut self, range: impl std::ops::RangeBounds<usize>) {
        self.tree.reverse(range);
    }
}

impl<O> SplaySequence<O>
where
    O: Operator + Foldable,
{
    pub fn fold(&mut self, range: impl std::ops::RangeBounds<usize>) -> O::P {
        self.tree.fold(range)
    }
}

impl<O> SplaySequence<O>
where
    O: Operator + Actable,
{
    pub fn apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: O::F) {
        self.tree.apply(range, f);
    }
}
