---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/link-cut-tree/src/lib.rs
    title: crates/data-structure/link-cut-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/splay-tree/src/lib.rs
    title: crates/data-structure/splay-tree/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.0/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.0/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Ordering, mem::swap, ptr::null_mut};\n\nuse algebraic::{Act,\
    \ Monoid};\n\npub struct SplayTreeNode<M, F>\nwhere\n    M: Monoid,\n    M::S:\
    \ Clone,\n    F: Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    pub lch: *mut\
    \ Self,\n    pub rch: *mut Self,\n    pub par: *mut Self,\n    pub len: usize,\n\
    \    pub rev: bool,\n    pub val: M::S,\n    pub prod: M::S,\n    pub prod_rev:\
    \ M::S,\n    pub lazy: F::S,\n}\n\nimpl<M, F> SplayTreeNode<M, F>\nwhere\n   \
    \ M: Monoid,\n    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n    F::S: Clone,\n\
    {\n    pub fn new(val: M::S) -> Self {\n        Self {\n            lch: null_mut(),\n\
    \            rch: null_mut(),\n            par: null_mut(),\n            len:\
    \ 1,\n            rev: false,\n            prod: val.clone(),\n            prod_rev:\
    \ val.clone(),\n            val,\n            lazy: F::e(),\n        }\n    }\n\
    \n    pub fn splay(&mut self) {\n        Self::push(self);\n        while !self.is_root()\
    \ {\n            let p = unsafe { self.par.as_mut() }.unwrap();\n            if\
    \ p.is_root() {\n                Self::push(p);\n                Self::push(self);\n\
    \                self.rotate();\n            } else {\n                let g =\
    \ unsafe { p.par.as_mut() }.unwrap();\n                Self::push(g);\n      \
    \          Self::push(p);\n                Self::push(self);\n               \
    \ if self.pos() == p.pos() {\n                    p.rotate();\n              \
    \      self.rotate();\n                } else {\n                    self.rotate();\n\
    \                    self.rotate();\n                }\n            }\n      \
    \  }\n    }\n\n    pub fn merge(l: *mut Self, r: *mut Self) -> *mut Self {\n \
    \       match unsafe { (l.as_mut(), r.as_mut()) } {\n            (None, None)\
    \ => null_mut(),\n            (None, Some(r)) => {\n                r.splay();\n\
    \                r\n            }\n            (Some(l), None) => {\n        \
    \        l.splay();\n                l\n            }\n            (Some(mut l),\
    \ Some(r)) => {\n                l.splay();\n                r.splay();\n    \
    \            l = l.get_right();\n                l.splay();\n                l.rch\
    \ = r;\n                r.par = l;\n                Self::update(l);\n       \
    \         l\n            }\n        }\n    }\n\n    pub fn split(t: *mut Self,\
    \ k: usize) -> (*mut Self, *mut Self) {\n        if t.is_null() {\n          \
    \  (null_mut(), null_mut())\n        } else if k == 0 {\n            (null_mut(),\
    \ t)\n        } else if k == Self::len(t) {\n            (t, null_mut())\n   \
    \     } else {\n            let t = unsafe { t.as_mut() }.unwrap();\n        \
    \    Self::push(t);\n            if k <= Self::len(t.lch) {\n                let\
    \ (x, y) = Self::split(t.lch, k);\n                t.lch = y;\n              \
    \  t.par = null_mut();\n                if let Some(y) = unsafe { y.as_mut() }\
    \ {\n                    y.par = t;\n                }\n                Self::update(t);\n\
    \                (x, t)\n            } else {\n                let (x, y) = Self::split(t.rch,\
    \ k - Self::len(t.lch) - 1);\n                t.rch = x;\n                t.par\
    \ = null_mut();\n                if let Some(x) = unsafe { x.as_mut() } {\n  \
    \                  x.par = t;\n                }\n                Self::update(t);\n\
    \                (t, y)\n            }\n        }\n    }\n\n    pub fn insert(t:\
    \ &mut *mut Self, k: usize, val: M::S) {\n        if let Some(t) = unsafe { t.as_mut()\
    \ } {\n            t.splay();\n        }\n        let (x, y) = Self::split(*t,\
    \ k);\n        let z = Box::leak(Box::new(Self::new(val)));\n        *t = Self::merge(Self::merge(x,\
    \ z), y);\n    }\n\n    pub fn remove(t: &mut *mut Self, k: usize) -> M::S {\n\
    \        unsafe { t.as_mut() }.unwrap().splay();\n        let (x, y) = Self::split(*t,\
    \ k);\n        let (y, z) = Self::split(y, 1);\n        *t = Self::merge(x, z);\n\
    \        unsafe { Box::from_raw(y) }.val\n    }\n\n    pub fn access(&mut self,\
    \ mut k: usize) -> &mut Self {\n        let mut v = self;\n        loop {\n  \
    \          Self::push(v);\n            Self::push(v.lch);\n            Self::push(v.rch);\n\
    \            v = match k.cmp(&Self::len(v.lch)) {\n                Ordering::Less\
    \ => unsafe { v.lch.as_mut() }.unwrap(),\n                Ordering::Equal => {\n\
    \                    v.splay();\n                    return v;\n             \
    \   }\n                Ordering::Greater => {\n                    k -= Self::len(v.lch)\
    \ + 1;\n                    unsafe { v.rch.as_mut() }.unwrap()\n             \
    \   }\n            }\n        }\n    }\n\n    pub fn build(a: &[M::S]) -> *mut\
    \ Self {\n        let n = a.len();\n        if n == 0 {\n            null_mut()\n\
    \        } else if n == 1 {\n            Box::leak(Box::new(Self::new(a[0].clone())))\n\
    \        } else {\n            Self::merge(Self::build(&a[0..n / 2]), Self::build(&a[n\
    \ / 2..n]))\n        }\n    }\n\n    pub fn get_left(&mut self) -> &mut Self {\n\
    \        let mut v: &mut Self = self;\n        while let Some(l) = unsafe { v.lch.as_mut()\
    \ } {\n            Self::push(l);\n            v = l;\n        }\n        v\n\
    \    }\n\n    pub fn get_right(&mut self) -> &mut Self {\n        let mut v: &mut\
    \ Self = self;\n        while let Some(r) = unsafe { v.rch.as_mut() } {\n    \
    \        Self::push(r);\n            v = r;\n        }\n        v\n    }\n\n \
    \   pub fn update(t: *mut Self) {\n        if let Some(t) = unsafe { t.as_mut()\
    \ } {\n            t.len = 1;\n            t.prod = t.val.clone();\n         \
    \   t.prod_rev = t.val.clone();\n            if let Some(l) = unsafe { t.lch.as_mut()\
    \ } {\n                t.len += l.len;\n                t.prod = M::op(&l.prod,\
    \ &t.prod);\n                t.prod_rev = M::op(&t.prod_rev, &l.prod_rev);\n \
    \           }\n            if let Some(r) = unsafe { t.rch.as_mut() } {\n    \
    \            t.len += r.len;\n                t.prod = M::op(&t.prod, &r.prod);\n\
    \                t.prod_rev = M::op(&r.prod_rev, &t.prod_rev);\n            }\n\
    \        }\n    }\n\n    pub fn toggle(&mut self) {\n        swap(&mut self.lch,\
    \ &mut self.rch);\n        swap(&mut self.prod, &mut self.prod_rev);\n       \
    \ self.rev ^= true;\n    }\n\n    pub fn push(t: *mut Self) {\n        if let\
    \ Some(t) = unsafe { t.as_mut() } {\n            if t.rev {\n                if\
    \ let Some(l) = unsafe { t.lch.as_mut() } {\n                    l.toggle();\n\
    \                }\n                if let Some(r) = unsafe { t.rch.as_mut() }\
    \ {\n                    r.toggle();\n                }\n                t.rev\
    \ = false;\n            }\n            if let Some(l) = unsafe { t.lch.as_mut()\
    \ } {\n                l.propagate(&t.lazy);\n            }\n            if let\
    \ Some(r) = unsafe { t.rch.as_mut() } {\n                r.propagate(&t.lazy);\n\
    \            }\n            t.lazy = F::e();\n        }\n    }\n\n    pub fn propagate(&mut\
    \ self, f: &F::S) {\n        self.lazy = F::op(f, &self.lazy);\n        self.val\
    \ = F::act(f, &self.val);\n        self.prod = F::act(f, &self.prod);\n      \
    \  self.prod_rev = F::act(f, &self.prod_rev);\n    }\n\n    pub fn is_root(&self)\
    \ -> bool {\n        self.par.is_null() || {\n            let p = unsafe { self.par.as_mut()\
    \ }.unwrap();\n            !std::ptr::eq(self, p.lch) && !std::ptr::eq(self, p.rch)\n\
    \        }\n    }\n\n    pub fn len(t: *mut Self) -> usize {\n        unsafe {\
    \ t.as_ref() }.map_or(0, |t| t.len)\n    }\n\n    pub fn pos(&self) -> i32 {\n\
    \        if let Some(p) = unsafe { self.par.as_mut() } {\n            if std::ptr::eq(self,\
    \ p.lch) {\n                -1\n            } else if std::ptr::eq(self, p.rch)\
    \ {\n                1\n            } else {\n                0\n            }\n\
    \        } else {\n            0\n        }\n    }\n\n    pub fn rotate(&mut self)\
    \ {\n        let p = unsafe { self.par.as_mut() }.unwrap();\n        let g = p.par;\n\
    \        if self.pos() == -1 {\n            p.lch = self.rch;\n            if\
    \ let Some(r) = unsafe { self.rch.as_mut() } {\n                r.par = p;\n \
    \           }\n            self.rch = p;\n            p.par = self;\n        }\
    \ else {\n            p.rch = self.lch;\n            if let Some(l) = unsafe {\
    \ self.lch.as_mut() } {\n                l.par = p;\n            }\n         \
    \   self.lch = p;\n            p.par = self;\n        }\n        Self::update(p);\n\
    \        Self::update(self);\n        self.par = g;\n        if let Some(g) =\
    \ unsafe { g.as_mut() } {\n            if std::ptr::eq(p, g.lch) {\n         \
    \       g.lch = self;\n            } else if std::ptr::eq(p, g.rch) {\n      \
    \          g.rch = self;\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/splay-tree-internal/src/lib.rs
  requiredBy:
  - crates/data-structure/link-cut-tree/src/lib.rs
  - crates/data-structure/splay-tree/src/lib.rs
  timestamp: '2023-05-11 17:21:10+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/splay-tree-internal/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/splay-tree-internal/src/lib.rs
- /library/crates/data-structure/splay-tree-internal/src/lib.rs.html
title: crates/data-structure/splay-tree-internal/src/lib.rs
---
