use std::{cmp::Ordering, ptr::NonNull};

use as_half_open_range::AsHalfOpenRange;

use crate::{allocator::new_ptr, node::Node, operator::Operator};

pub(crate) struct Tree<O>
where
    O: Operator,
{
    pub(crate) root: Option<NonNull<Node<O>>>,
}

impl<O> Tree<O>
where
    O: Operator,
{
    pub(crate) fn len(&self) -> usize {
        self.root.map_or(0, |root| unsafe { root.as_ref().len })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub(crate) fn access(&mut self, mut i: usize) {
        assert!(i < self.len());
        let mut root = self.root.unwrap();
        loop {
            let root_ref = unsafe { root.as_mut() };
            root_ref.push();
            for ch in root_ref.ch.iter_mut().flatten() {
                let ch_ref = unsafe { ch.as_mut() };
                ch_ref.push();
            }

            let l_len = root_ref.ch[0].map_or(0, |l| unsafe { l.as_ref().len });
            root = match i.cmp(&l_len) {
                Ordering::Less => root_ref.ch[0].unwrap(),
                Ordering::Equal => {
                    root_ref.splay();
                    self.root = Some(root);
                    return;
                }
                Ordering::Greater => {
                    i -= l_len + 1;
                    root_ref.ch[1].unwrap()
                }
            }
        }
    }

    pub(crate) fn append(&mut self, mut right: Self) {
        if self.is_empty() {
            *self = right;
        } else if !right.is_empty() {
            self.access(self.len() - 1);
            let self_ref = unsafe { self.root.as_mut().unwrap().as_mut() };
            let right_ref = unsafe { right.root.as_mut().unwrap().as_mut() };
            self_ref.push();
            self_ref.ch[1] = right.root;
            right_ref.par = self.root;
            self_ref.update();
        }
    }

    pub(crate) fn split_at(&mut self, mid: usize) -> Self {
        assert!(mid <= self.len());
        if mid == 0 {
            std::mem::take(self)
        } else if mid == self.len() {
            Self::default()
        } else {
            self.access(mid - 1);
            let self_ref = unsafe { self.root.as_mut().unwrap().as_mut() };
            self_ref.push();
            let mut right = std::mem::take(&mut self_ref.ch[1]);

            let right_ref = unsafe { right.as_mut().unwrap().as_mut() };
            right_ref.par = None;

            self_ref.update();
            Self { root: right }
        }
    }

    pub(crate) fn reverse(&mut self, range: impl std::ops::RangeBounds<usize>) {
        let (l, r) = range.as_half_open_range(0, self.len());
        if l == r {
            return;
        }

        let mut m = self.split_at(l);
        let r = m.split_at(r - l);

        let m_ref = unsafe { m.root.as_mut().unwrap().as_mut() };
        m_ref.rev ^= true;
        m_ref.push();

        self.append(m);
        self.append(r);
    }

    pub(crate) fn insert(&mut self, i: usize, x: O::X) {
        assert!(i <= self.len());
        let m = Tree {
            root: Some(new_ptr(Node::new(x))),
        };
        let r = self.split_at(i);
        self.append(m);
        self.append(r);
    }

    pub(crate) fn remove(&mut self, i: usize) -> O::X {
        assert!(i < self.len());
        let mut mr = self.split_at(i);
        let r = mr.split_at(1);
        self.append(r);
        unsafe { mr.root.as_mut().unwrap().as_mut() }.val.clone()
    }

    pub(crate) fn get(&mut self, i: usize) -> Option<&O::X> {
        if i >= self.len() {
            None
        } else {
            self.access(i);
            Some(&unsafe { self.root.as_mut().unwrap().as_mut() }.val)
        }
    }

    pub(crate) fn set(&mut self, i: usize, x: O::X) {
        assert!(i < self.len());
        self.access(i);
        let root_ref = unsafe { self.root.as_mut().unwrap().as_mut() };
        root_ref.val = x;
        root_ref.update();
    }

    pub(crate) fn fold(&mut self, range: impl std::ops::RangeBounds<usize>) -> O::P {
        let (l, r) = range.as_half_open_range(0, self.len());
        if l == r {
            return O::unit();
        }
        let mut m = self.split_at(l);
        let r = m.split_at(r - l);
        let m_ref = unsafe { m.root.as_mut().unwrap().as_mut() };
        m_ref.update();
        let res = m_ref.prod.clone();
        self.append(m);
        self.append(r);
        res
    }

    pub(crate) fn apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: O::F) {
        let (l, r) = range.as_half_open_range(0, self.len());
        if l == r {
            return;
        }
        let mut m = self.split_at(l);
        let r = m.split_at(r - l);
        let m_ref = unsafe { m.root.as_mut().unwrap().as_mut() };
        m_ref.act = O::compose(&m_ref.act, &f);
        m_ref.push();
        self.append(m);
        self.append(r);
    }
}

impl<O> Default for Tree<O>
where
    O: Operator,
{
    fn default() -> Self {
        Self { root: None }
    }
}

impl<O> Tree<O>
where
    O: Operator,
    O::X: Default,
{
    pub(crate) fn new(n: usize) -> Self {
        Self::from_fn(n, |_| O::X::default())
    }
}

impl<O> Tree<O>
where
    O: Operator,
{
    pub(crate) fn from_fn(n: usize, f: impl FnMut(usize) -> O::X) -> Self {
        Self::from_iter((0..n).map(f))
    }
}

impl<O> FromIterator<O::X> for Tree<O>
where
    O: Operator,
{
    fn from_iter<T: IntoIterator<Item = O::X>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut root: NonNull<Node<O>> = match iter.next() {
            Some(x) => new_ptr(Node::new(x)),
            None => return Self::default(),
        };

        for x in iter {
            let mut root_ref = unsafe { root.as_mut() };
            let mut new_root = new_ptr(Node::new(x));
            let new_root_ref = unsafe { new_root.as_mut() };
            new_root_ref.ch[0] = Some(root);
            root_ref.par = Some(new_root);
            new_root_ref.update();
            root = new_root;
        }

        Self { root: Some(root) }
    }
}
