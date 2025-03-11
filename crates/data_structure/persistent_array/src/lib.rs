use std::{ops::Index, ptr::NonNull};

/// 全永続配列  
/// len は提供しない
#[derive(Clone, Copy)]
pub struct PersistentArray<T, const M: usize = 8> {
    root: Option<NonNull<Node<T, M>>>,
}

struct Node<T, const M: usize> {
    val: NonNull<T>,
    ch: [Option<NonNull<Self>>; M],
}

impl<T, const M: usize> FromIterator<T> for PersistentArray<T, M> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        let Some(first) = iter.next() else {
            return Self { root: None };
        };

        let root = new_ptr(Node::new(first));
        let mut n = 1;
        while let Some(val) = iter.next() {
            let mut vp = root;
            let mut i = n;
            loop {
                let v = unsafe { vp.as_mut() };
                let ci = i % M;
                i = (i - 1) / M;
                if i == 0 {
                    v.ch[ci] = Some(new_ptr(Node::new(val)));
                    break;
                } else {
                    vp = v.ch[ci].unwrap();
                }
            }
            n += 1;
        }

        Self { root: Some(root) }
    }
}

impl<T, const M: usize> PersistentArray<T, M> {
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> T) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn update_with(&self, mut i: usize, f: impl FnOnce(&T) -> T) -> Self {
        let root = unsafe { self.root.expect("out of range").as_ref() };
        let new_root = new_ptr(root.clone());
        let mut vp = new_root;
        let mut x = unsafe { vp.as_ref().val };
        while i != 0 {
            let v = unsafe { vp.as_mut() };
            let ci = i % M;
            i = (i - 1) / M;
            let c = unsafe { v.ch[ci].expect("out of range").as_ref() };
            x = c.val;
            let new_c = new_ptr(c.clone());
            v.ch[ci] = Some(new_c);
            vp = new_c;
        }
        unsafe { vp.as_mut().val = new_ptr(f(x.as_ref())) };
        Self {
            root: Some(new_root),
        }
    }

    pub fn set(&self, i: usize, x: T) -> Self {
        self.update_with(i, |_| x)
    }
}

impl<T, const M: usize> Index<usize> for PersistentArray<T, M> {
    type Output = T;

    fn index(&self, mut i: usize) -> &Self::Output {
        let mut vp = self.root.expect("out of range");
        while i != 0 {
            let v = unsafe { vp.as_ref() };
            let c = i % M;
            i = (i - 1) / M;
            vp = v.ch[c].expect("out of range");
        }

        unsafe { vp.as_ref().val.as_ref() }
    }
}

fn new_ptr<T>(val: T) -> NonNull<T> {
    NonNull::new(Box::into_raw(Box::new(val))).unwrap()
}

impl<T, const M: usize> Node<T, M> {
    fn new(val: T) -> Self {
        Self {
            val: new_ptr(val),
            ch: [None; M],
        }
    }
}

impl<T, const M: usize> Clone for Node<T, M> {
    fn clone(&self) -> Self {
        Self {
            val: self.val,
            ch: self.ch,
        }
    }
}
