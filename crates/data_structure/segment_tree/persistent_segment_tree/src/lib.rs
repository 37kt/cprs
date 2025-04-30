use std::{ops::RangeBounds, ptr::NonNull};

use algebraic_traits::Monoid;
use as_half_open_range::AsHalfOpenRange;
use node::Node;

mod node;

pub struct PersistentSegmentTree<M>
where
    M: Monoid,
{
    n: usize,
    root: NonNull<Node<M>>,
}

impl<M> Clone for PersistentSegmentTree<M>
where
    M: Monoid,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            root: Node::copy(Some(self.root)).unwrap(),
        }
    }
}

impl<M> PersistentSegmentTree<M>
where
    M: Monoid,
{
    pub fn new(n: usize) -> Self {
        let root = Node::new_ptr(None, None, M::unit());
        Self { n, root }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn set(&self, i: usize, x: M::Value) -> Self {
        assert!(i < self.n);

        let new_root = Self::set_rec(Some(self.root), 0, self.n, i, x).unwrap();
        Self {
            n: self.n,
            root: new_root,
        }
    }

    pub fn add(&self, i: usize, x: M::Value) -> Self {
        assert!(i < self.n);

        let new_root = Self::add_rec(Some(self.root), 0, self.n, i, x).unwrap();
        Self {
            n: self.n,
            root: new_root,
        }
    }

    pub fn copy_range(&self, range: impl RangeBounds<usize>, source: &Self) -> Self {
        let (l, r) = range.as_half_open_range(0, self.n);
        let new_root =
            Self::copy_range_rec(Some(self.root), Some(source.root), 0, self.n, l, r).unwrap();
        Self {
            n: self.n,
            root: new_root,
        }
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

        Node::fold(v)
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {
        let (l, r) = range.as_half_open_range(0, self.n);
        Self::fold_rec(Some(self.root), 0, self.n, l, r)
    }

    fn set_rec(
        v: Option<NonNull<Node<M>>>,
        a: usize,
        b: usize,
        i: usize,
        x: M::Value,
    ) -> Option<NonNull<Node<M>>> {
        if a == b {
            return None;
        }

        let mut v = Node::copy(v).unwrap_or(Node::new_ptr(None, None, M::unit()));
        let v_ref = unsafe { v.as_mut() };
        if a + 1 == b {
            v_ref.x = x;
            return Some(v);
        }

        let c = (a + b) / 2;
        if i < c {
            v_ref.l = Self::set_rec(v_ref.l, a, c, i, x);
        } else {
            v_ref.r = Self::set_rec(v_ref.r, c, b, i, x);
        }

        Node::update(v);
        Some(v)
    }

    fn add_rec(
        v: Option<NonNull<Node<M>>>,
        a: usize,
        b: usize,
        i: usize,
        x: M::Value,
    ) -> Option<NonNull<Node<M>>> {
        if a == b {
            return None;
        }

        let mut v = Node::copy(v).unwrap_or(Node::new_ptr(None, None, M::unit()));
        let v_ref = unsafe { v.as_mut() };
        if a + 1 == b {
            v_ref.x = M::op(&v_ref.x, &x);
            return Some(v);
        }

        let c = (a + b) / 2;
        if i < c {
            v_ref.l = Self::add_rec(v_ref.l, a, c, i, x);
        } else {
            v_ref.r = Self::add_rec(v_ref.r, c, b, i, x);
        }

        Node::update(v);
        Some(v)
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

    fn copy_range_rec(
        v: Option<NonNull<Node<M>>>,
        source: Option<NonNull<Node<M>>>,
        a: usize,
        b: usize,
        l: usize,
        r: usize,
    ) -> Option<NonNull<Node<M>>> {
        if b <= l || r <= a {
            v
        } else if l <= a && b <= r {
            Node::copy(source)
        } else {
            let vl = v.as_ref().and_then(|v| unsafe { v.as_ref() }.l);
            let vr = v.as_ref().and_then(|v| unsafe { v.as_ref() }.r);
            let sl = source.as_ref().and_then(|v| unsafe { v.as_ref() }.l);
            let sr = source.as_ref().and_then(|v| unsafe { v.as_ref() }.r);
            let c = (a + b) / 2;
            let res_l = Self::copy_range_rec(vl, sl, a, c, l, r);
            let res_r = Self::copy_range_rec(vr, sr, c, b, l, r);
            Node::merge(res_l, res_r)
        }
    }
}
