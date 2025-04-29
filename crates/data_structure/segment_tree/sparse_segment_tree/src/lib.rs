// ABC403-G

use std::{ops::RangeBounds, ptr::NonNull};

use algebraic_traits::Monoid;
use allocator::new_ptr;
use as_half_open_range::AsHalfOpenRange;
use node::Node;

mod allocator;
mod node;

pub struct SparseSegmentTree<M>
where
    M: Monoid,
{
    n: usize,
    root: NonNull<Node<M>>,
}

impl<M> SparseSegmentTree<M>
where
    M: Monoid,
{
    pub fn new(n: usize) -> Self {
        let root = new_ptr(Node::new(M::unit()));
        Self { n, root }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn set(&mut self, i: usize, x: M::Value) {
        assert!(i < self.n);
        Self::set_rec(&mut Some(self.root), 0, self.n, i, x);
    }

    pub fn add(&mut self, i: usize, x: M::Value) {
        assert!(i < self.n);
        Self::add_rec(&mut Some(self.root), 0, self.n, i, x);
    }

    pub fn clear_range(&mut self, range: impl RangeBounds<usize>) {
        let (l, r) = range.as_half_open_range(0, self.n);
        Self::clear_range_rec(&mut Some(self.root), 0, self.n, l, r);
    }

    pub fn get(&self, i: usize) -> M::Value {
        assert!(i < self.n);
        let mut v = Some(self.root);
        let mut a = 0;
        let mut b = self.n;
        while a + 1 < b && v.is_some() {
            let c = (a + b) / 2;
            let v_ref = unsafe { v.as_ref().unwrap().as_ref() };
            if i < c {
                v = v_ref.l;
                b = c;
            } else {
                v = v_ref.r;
                a = c;
            }
        }

        v.as_ref()
            .map_or(M::unit(), |v| unsafe { v.as_ref() }.x.clone())
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {
        let (l, r) = range.as_half_open_range(0, self.n);
        Self::fold_rec(Some(self.root), 0, self.n, l, r)
    }

    fn fold_node(v: Option<NonNull<Node<M>>>) -> M::Value {
        v.as_ref()
            .map_or(M::unit(), |v| unsafe { v.as_ref() }.x.clone())
    }

    fn update_node(v: &mut Node<M>) {
        v.x = M::op(&Self::fold_node(v.l), &Self::fold_node(v.r));
    }

    fn set_rec(v: &mut Option<NonNull<Node<M>>>, a: usize, b: usize, i: usize, x: M::Value) {
        if a == b {
            return;
        }

        if v.is_none() {
            *v = Some(new_ptr(Node::new(M::unit())));
        }
        let v = unsafe { v.as_mut().unwrap().as_mut() };
        if a + 1 == b {
            v.x = x;
            return;
        }

        let c = (a + b) / 2;
        if i < c {
            Self::set_rec(&mut v.l, a, c, i, x);
        } else {
            Self::set_rec(&mut v.r, c, b, i, x);
        }

        Self::update_node(v);
    }

    fn add_rec(v: &mut Option<NonNull<Node<M>>>, a: usize, b: usize, i: usize, x: M::Value) {
        if a == b {
            return;
        }

        if v.is_none() {
            *v = Some(new_ptr(Node::new(M::unit())));
        }
        let v = unsafe { v.as_mut().unwrap().as_mut() };
        if a + 1 == b {
            v.x = M::op(&v.x, &x);
            return;
        }

        let c = (a + b) / 2;
        if i < c {
            Self::add_rec(&mut v.l, a, c, i, x);
        } else {
            Self::add_rec(&mut v.r, c, b, i, x);
        }

        Self::update_node(v);
    }

    fn fold_rec(v: Option<NonNull<Node<M>>>, a: usize, b: usize, l: usize, r: usize) -> M::Value {
        if v.is_none() || b <= l || r <= a {
            return M::unit();
        }

        let v = unsafe { v.as_ref().unwrap().as_ref() };
        if l <= a && b <= r {
            v.x.clone()
        } else {
            let c = (a + b) / 2;
            M::op(
                &Self::fold_rec(v.l, a, c, l, r),
                &Self::fold_rec(v.r, c, b, l, r),
            )
        }
    }

    fn clear_range_rec(v: &mut Option<NonNull<Node<M>>>, a: usize, b: usize, l: usize, r: usize) {
        if v.is_none() || b <= l || r <= a {
            return;
        }
        if l <= a && b <= r {
            *v = None;
            return;
        }
        let v = unsafe { v.as_mut().unwrap().as_mut() };

        let c = (a + b) / 2;
        Self::clear_range_rec(&mut v.l, a, c, l, r);
        Self::clear_range_rec(&mut v.r, c, b, l, r);

        Self::update_node(v);
    }
}
