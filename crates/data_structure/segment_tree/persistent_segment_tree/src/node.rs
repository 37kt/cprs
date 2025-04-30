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

impl<M> Clone for Node<M>
where
    M: Monoid,
{
    fn clone(&self) -> Self {
        Self {
            l: self.l,
            r: self.r,
            x: self.x.clone(),
        }
    }
}

impl<M> Node<M>
where
    M: Monoid,
{
    pub fn new(l: Option<NonNull<Node<M>>>, r: Option<NonNull<Node<M>>>, x: M::Value) -> Self {
        Self { l, r, x }
    }

    pub fn new_ptr(
        l: Option<NonNull<Node<M>>>,
        r: Option<NonNull<Node<M>>>,
        x: M::Value,
    ) -> NonNull<Self> {
        ARENA.with(|arena| NonNull::new(arena.borrow_mut().alloc(Self::new(l, r, x))).unwrap())
    }

    pub fn copy(v: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        v.map(|v| {
            ARENA.with(|arena| {
                NonNull::new(arena.borrow_mut().alloc(unsafe { v.as_ref() }.clone())).unwrap()
            })
        })
    }

    pub fn fold(v: Option<NonNull<Self>>) -> M::Value {
        v.map_or(M::unit(), |v| unsafe { v.as_ref() }.x.clone())
    }

    pub fn merge(l: Option<NonNull<Self>>, r: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        if l.is_none() && r.is_none() {
            None
        } else {
            Some(Node::new_ptr(l, r, M::op(&Node::fold(l), &Node::fold(r))))
        }
    }

    pub fn update(mut v: NonNull<Self>) {
        unsafe { v.as_mut() }.x = M::op(
            &Node::fold(unsafe { v.as_ref().l }),
            &Node::fold(unsafe { v.as_ref().r }),
        );
    }
}
