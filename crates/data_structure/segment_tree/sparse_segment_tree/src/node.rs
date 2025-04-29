use std::ptr::NonNull;

use algebraic_traits::Monoid;

pub(crate) struct Node<M>
where
    M: Monoid,
{
    pub l: Option<NonNull<Node<M>>>,
    pub r: Option<NonNull<Node<M>>>,
    pub x: M::Value,
}

impl<M> Node<M>
where
    M: Monoid,
{
    pub fn new(x: M::Value) -> Self {
        Self {
            l: None,
            r: None,
            x,
        }
    }
}
