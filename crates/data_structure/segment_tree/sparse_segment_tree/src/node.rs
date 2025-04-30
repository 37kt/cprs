use algebraic_traits::Monoid;
use simple_arena::Arena;

use std::{cell::RefCell, ptr::NonNull};

thread_local! {
    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024 * 1024 * 1024));
}

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

    pub fn new_ptr(x: M::Value) -> NonNull<Self> {
        ARENA.with(|arena| NonNull::new(arena.borrow_mut().alloc(Self::new(x))).unwrap())
    }
}
