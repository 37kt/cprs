use std::{cell::RefCell, ptr::NonNull};

use algebraic_traits::{Act, Algebraic, Magma, Monoid, Unital};
use numeric_traits::Integer;
use simple_arena::Arena;

struct Node<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
    val: Option<NonNull<<A::Operand as Algebraic>::Value>>,
    lz: Option<NonNull<<A::Operator as Algebraic>::Value>>,
    ch: [Option<NonNull<Self>>; 2],
}

impl<A> Clone for Node<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            val: self.val,
            lz: self.lz,
            ch: self.ch,
        }
    }
}

impl<A> Copy for Node<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
}

pub struct PersistentLazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
    n: u32,
    sz: u32,
    root: NonNull<Node<A>>,
}

impl<A> Clone for PersistentLazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            sz: self.sz,
            root: self.root,
        }
    }
}

impl<A> Copy for PersistentLazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
}

impl<A> FromIterator<<A::Operand as Algebraic>::Value> for PersistentLazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = <A::Operand as Algebraic>::Value>>(iter: T) -> Self {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let lg = n.checked_ceil_log2().unwrap_or(0);
        let sz = 1 << lg;
        let mut nodes = (0..sz)
            .map(|_| {
                new_ptr(Node::<A> {
                    val: Some(new_ptr(A::Operand::unit())),
                    lz: None,
                    ch: [None, None],
                })
            })
            .chain(a.into_iter().map(|x| {
                new_ptr(Node::<A> {
                    val: Some(new_ptr(x)),
                    lz: None,
                    ch: [None, None],
                })
            }))
            .chain((0..sz - n).map(|_| {
                new_ptr(Node::<A> {
                    val: Some(new_ptr(A::Operand::unit())),
                    lz: None,
                    ch: [None, None],
                })
            }))
            .collect::<Vec<_>>();
        for i in (1..sz).rev() {
            let node = unsafe { nodes[i].as_mut() };
            node.ch[0] = Some(nodes[i << 1]);
            node.ch[1] = Some(nodes[i << 1 | 1]);
        }
        Self {
            n: n as u32,
            sz: sz as u32,
            root: nodes[1],
        }
    }
}

impl<A> PersistentLazySegmentTree<A>
where
    A: Act,
    A::Operand: Monoid,
    A::Operator: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    <A::Operator as Algebraic>::Value: Clone,
{
    pub fn new(n: usize) -> Self {
        Self::from_iter((0..n).map(|_| A::Operand::unit()))
    }

    pub fn from_fn(n: usize, f: impl FnMut(usize) -> <A::Operand as Algebraic>::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn len(&self) -> usize {
        self.n as usize
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn apply_at(&self, v: &mut Node<A>, f: NonNull<<A::Operator as Algebraic>::Value>) {
        let val = unsafe { v.val.unwrap().as_mut() };
        *val = A::act(val, unsafe { f.as_ref() });
        if let Some(mut g) = v.lz {
            let g = unsafe { g.as_mut() };
            *g = A::Operator::op(g, unsafe { f.as_ref() });
        } else {
            v.lz = Some(f);
        }
    }

    fn push(&self, v: &mut Node<A>) {
        if let Some(f) = v.lz.take() {
            unsafe {
                self.apply_at(v.ch[0].unwrap().as_mut(), f);
                self.apply_at(v.ch[1].unwrap().as_mut(), f);
            }
        }
    }
}

thread_local! {
    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024 * 1024 * 1024));
}

fn new_ptr<T>(val: T) -> NonNull<T> {
    ARENA.with(|arena| NonNull::new(arena.borrow_mut().alloc(val)).unwrap())
}
