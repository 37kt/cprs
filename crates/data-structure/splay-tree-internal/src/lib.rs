use std::{cmp::Ordering, mem::swap, ptr::null_mut};

use algebraic::{Act, Monoid};

pub struct SplayTreeNode<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    pub lch: *mut Self,
    pub rch: *mut Self,
    pub par: *mut Self,
    pub len: usize,
    pub rev: bool,
    pub val: M::S,
    pub prod: M::S,
    pub prod_rev: M::S,
    pub lazy: F::S,
}

impl<M, F> SplayTreeNode<M, F>
where
    M: Monoid,
    M::S: Clone,
    F: Monoid + Act<X = M::S>,
    F::S: Clone,
{
    pub fn new(val: M::S) -> Self {
        Self {
            lch: null_mut(),
            rch: null_mut(),
            par: null_mut(),
            len: 1,
            rev: false,
            prod: val.clone(),
            prod_rev: val.clone(),
            val,
            lazy: F::e(),
        }
    }

    pub fn splay(&mut self) {
        Self::push(self);
        while !self.is_root() {
            let p = unsafe { self.par.as_mut() }.unwrap();
            if p.is_root() {
                Self::push(p);
                Self::push(self);
                self.rotate();
            } else {
                let g = unsafe { p.par.as_mut() }.unwrap();
                Self::push(g);
                Self::push(p);
                Self::push(self);
                if self.pos() == p.pos() {
                    p.rotate();
                    self.rotate();
                } else {
                    self.rotate();
                    self.rotate();
                }
            }
        }
    }

    pub fn merge(l: *mut Self, r: *mut Self) -> *mut Self {
        match unsafe { (l.as_mut(), r.as_mut()) } {
            (None, None) => null_mut(),
            (None, Some(r)) => {
                r.splay();
                r
            }
            (Some(l), None) => {
                l.splay();
                l
            }
            (Some(mut l), Some(r)) => {
                l.splay();
                r.splay();
                l = l.get_right();
                l.splay();
                l.rch = r;
                r.par = l;
                Self::update(l);
                l
            }
        }
    }

    pub fn split(t: *mut Self, k: usize) -> (*mut Self, *mut Self) {
        if t.is_null() {
            (null_mut(), null_mut())
        } else if k == 0 {
            (null_mut(), t)
        } else if k == Self::len(t) {
            (t, null_mut())
        } else {
            let t = unsafe { t.as_mut() }.unwrap();
            Self::push(t);
            if k <= Self::len(t.lch) {
                let (x, y) = Self::split(t.lch, k);
                t.lch = y;
                t.par = null_mut();
                if let Some(y) = unsafe { y.as_mut() } {
                    y.par = t;
                }
                Self::update(t);
                (x, t)
            } else {
                let (x, y) = Self::split(t.rch, k - Self::len(t.lch) - 1);
                t.rch = x;
                t.par = null_mut();
                if let Some(x) = unsafe { x.as_mut() } {
                    x.par = t;
                }
                Self::update(t);
                (t, y)
            }
        }
    }

    pub fn insert(t: &mut *mut Self, k: usize, val: M::S) {
        if let Some(t) = unsafe { t.as_mut() } {
            t.splay();
        }
        let (x, y) = Self::split(*t, k);
        let z = Box::leak(Box::new(Self::new(val)));
        *t = Self::merge(Self::merge(x, z), y);
    }

    pub fn remove(t: &mut *mut Self, k: usize) -> M::S {
        unsafe { t.as_mut() }.unwrap().splay();
        let (x, y) = Self::split(*t, k);
        let (y, z) = Self::split(y, 1);
        *t = Self::merge(x, z);
        unsafe { Box::from_raw(y) }.val
    }

    pub fn access(&mut self, mut k: usize) -> &mut Self {
        let mut v = self;
        loop {
            Self::push(v);
            Self::push(v.lch);
            Self::push(v.rch);
            v = match k.cmp(&Self::len(v.lch)) {
                Ordering::Less => unsafe { v.lch.as_mut() }.unwrap(),
                Ordering::Equal => {
                    v.splay();
                    return v;
                }
                Ordering::Greater => {
                    k -= Self::len(v.lch) + 1;
                    unsafe { v.rch.as_mut() }.unwrap()
                }
            }
        }
    }

    pub fn build(a: &[M::S]) -> *mut Self {
        let n = a.len();
        if n == 0 {
            null_mut()
        } else if n == 1 {
            Box::leak(Box::new(Self::new(a[0].clone())))
        } else {
            Self::merge(Self::build(&a[0..n / 2]), Self::build(&a[n / 2..n]))
        }
    }

    pub fn get_left(&mut self) -> &mut Self {
        let mut v: &mut Self = self;
        while let Some(l) = unsafe { v.lch.as_mut() } {
            Self::push(l);
            v = l;
        }
        v
    }

    pub fn get_right(&mut self) -> &mut Self {
        let mut v: &mut Self = self;
        while let Some(r) = unsafe { v.rch.as_mut() } {
            Self::push(r);
            v = r;
        }
        v
    }

    pub fn update(t: *mut Self) {
        if let Some(t) = unsafe { t.as_mut() } {
            t.len = 1;
            t.prod = t.val.clone();
            t.prod_rev = t.val.clone();
            if let Some(l) = unsafe { t.lch.as_mut() } {
                t.len += l.len;
                t.prod = M::op(&l.prod, &t.prod);
                t.prod_rev = M::op(&t.prod_rev, &l.prod_rev);
            }
            if let Some(r) = unsafe { t.rch.as_mut() } {
                t.len += r.len;
                t.prod = M::op(&t.prod, &r.prod);
                t.prod_rev = M::op(&r.prod_rev, &t.prod_rev);
            }
        }
    }

    pub fn toggle(&mut self) {
        swap(&mut self.lch, &mut self.rch);
        swap(&mut self.prod, &mut self.prod_rev);
        self.rev ^= true;
    }

    pub fn push(t: *mut Self) {
        if let Some(t) = unsafe { t.as_mut() } {
            if t.rev {
                if let Some(l) = unsafe { t.lch.as_mut() } {
                    l.toggle();
                }
                if let Some(r) = unsafe { t.rch.as_mut() } {
                    r.toggle();
                }
                t.rev = false;
            }
            if let Some(l) = unsafe { t.lch.as_mut() } {
                l.propagate(&t.lazy);
            }
            if let Some(r) = unsafe { t.rch.as_mut() } {
                r.propagate(&t.lazy);
            }
            t.lazy = F::e();
        }
    }

    pub fn propagate(&mut self, f: &F::S) {
        self.lazy = F::op(f, &self.lazy);
        self.val = F::act(f, &self.val);
        self.prod = F::act(f, &self.prod);
        self.prod_rev = F::act(f, &self.prod_rev);
    }

    pub fn is_root(&self) -> bool {
        self.par.is_null() || {
            let p = unsafe { self.par.as_mut() }.unwrap();
            !std::ptr::eq(self, p.lch) && !std::ptr::eq(self, p.rch)
        }
    }

    pub fn len(t: *mut Self) -> usize {
        unsafe { t.as_ref() }.map_or(0, |t| t.len)
    }

    pub fn pos(&self) -> i32 {
        if let Some(p) = unsafe { self.par.as_mut() } {
            if std::ptr::eq(self, p.lch) {
                -1
            } else if std::ptr::eq(self, p.rch) {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn rotate(&mut self) {
        let p = unsafe { self.par.as_mut() }.unwrap();
        let g = p.par;
        if self.pos() == -1 {
            p.lch = self.rch;
            if let Some(r) = unsafe { self.rch.as_mut() } {
                r.par = p;
            }
            self.rch = p;
            p.par = self;
        } else {
            p.rch = self.lch;
            if let Some(l) = unsafe { self.lch.as_mut() } {
                l.par = p;
            }
            self.lch = p;
            p.par = self;
        }
        Self::update(p);
        Self::update(self);
        self.par = g;
        if let Some(g) = unsafe { g.as_mut() } {
            if std::ptr::eq(p, g.lch) {
                g.lch = self;
            } else if std::ptr::eq(p, g.rch) {
                g.rch = self;
            }
        }
    }
}
