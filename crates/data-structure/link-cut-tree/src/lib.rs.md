---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':warning:'
    path: crates/data-structure/splay-tree-internal/src/lib.rs
    title: crates/data-structure/splay-tree-internal/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/dynamic_tree_vertex_add_path_sum/src/main.rs
    title: verify/dynamic_tree_vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/dynamic_tree_vertex_set_path_composite/src/main.rs
    title: verify/dynamic_tree_vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ptr::null_mut;\n\nuse algebraic::{Act, Monoid};\nuse splay_tree_internal::SplayTreeNode;\n\
    \npub struct LinkCutTree<M, F>\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F:\
    \ Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    nodes: Vec<SplayTreeNode<M,\
    \ F>>,\n}\n\nimpl<M, F> From<&[M::S]> for LinkCutTree<M, F>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    fn\
    \ from(a: &[M::S]) -> Self {\n        Self {\n            nodes: a\n         \
    \       .iter()\n                .enumerate()\n                .map(|(i, x)| SplayTreeNode::new(x.clone(),\
    \ i))\n                .collect(),\n        }\n    }\n}\n\nimpl<M, F> LinkCutTree<M,\
    \ F>\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n\
    \    F::S: Clone,\n{\n    /// u \u306E\u89AA\u3092 v \u306B\u3059\u308B\n    pub\
    \ fn link(&mut self, u: usize, v: usize) {\n        Self::evert_(&mut self.nodes[u]);\n\
    \        Self::expose(&mut self.nodes[v]);\n        self.nodes[u].par = &mut self.nodes[v];\n\
    \    }\n\n    pub fn cut(&mut self, u: usize, v: usize) {\n        Self::evert_(&mut\
    \ self.nodes[u]);\n        Self::expose(&mut self.nodes[v]);\n        assert!(std::ptr::eq(self.nodes[u].par,\
    \ &self.nodes[v]));\n        self.nodes[v].lch = null_mut();\n        self.nodes[u].par\
    \ = null_mut();\n        SplayTreeNode::update(&mut self.nodes[v]);\n    }\n\n\
    \    pub fn set(&mut self, v: usize, val: M::S) {\n        let t = &mut self.nodes[v];\n\
    \        Self::evert_(t);\n        t.val = val;\n        SplayTreeNode::update(t);\n\
    \    }\n\n    pub fn get(&mut self, v: usize) -> &M::S {\n        let t = &mut\
    \ self.nodes[v];\n        Self::evert_(t);\n        &t.val\n    }\n\n    pub fn\
    \ apply(&mut self, u: usize, v: usize, f: F::S) {\n        Self::evert_(&mut self.nodes[u]);\n\
    \        Self::expose(&mut self.nodes[v]);\n        self.nodes[v].propagate(&f);\n\
    \    }\n\n    pub fn prod(&mut self, u: usize, v: usize) -> M::S {\n        Self::evert_(&mut\
    \ self.nodes[u]);\n        Self::expose(&mut self.nodes[v]);\n        self.nodes[v].prod.clone()\n\
    \    }\n\n    pub fn evert(&mut self, v: usize) {\n        Self::evert_(&mut self.nodes[v]);\n\
    \    }\n\n    pub fn root(&mut self, v: usize) -> usize {\n        let mut t =\
    \ &mut self.nodes[v];\n        Self::expose(t);\n        while let Some(x) = unsafe\
    \ { t.lch.as_mut() } {\n            SplayTreeNode::push(t);\n            t = x;\n\
    \        }\n        t.idx\n    }\n\n    pub fn lca(&mut self, u: usize, v: usize)\
    \ -> Option<usize> {\n        if self.root(u) != self.root(v) {\n            None\n\
    \        } else {\n            Self::expose(&mut self.nodes[u]);\n           \
    \ Some(Self::expose(&mut self.nodes[v]).idx)\n        }\n    }\n\n    pub fn parent(&mut\
    \ self, v: usize) -> Option<usize> {\n        Self::expose(&mut self.nodes[v]);\n\
    \        if let Some(mut p) = unsafe { self.nodes[v].lch.as_mut() } {\n      \
    \      loop {\n                SplayTreeNode::push(p);\n                if p.rch.is_null()\
    \ {\n                    return Some(p.idx);\n                }\n            \
    \    p = unsafe { p.rch.as_mut() }.unwrap();\n            }\n        } else {\n\
    \            None\n        }\n    }\n\n    pub fn kth_parent(&mut self, v: usize,\
    \ mut k: usize) -> Option<usize> {\n        let t = &mut self.nodes[v];\n    \
    \    Self::expose(t);\n        let mut t: *mut _ = t;\n        while let Some(x)\
    \ = unsafe { t.as_mut() } {\n            SplayTreeNode::push(x);\n           \
    \ if !x.rch.is_null() && unsafe { x.rch.as_mut() }.unwrap().len > k {\n      \
    \          t = x.rch;\n            } else {\n                if !x.rch.is_null()\
    \ {\n                    k -= unsafe { x.rch.as_mut() }.unwrap().len;\n      \
    \          }\n                if k == 0 {\n                    return Some(x.idx);\n\
    \                }\n                k -= 1;\n                t = x.lch;\n    \
    \        }\n        }\n        None\n    }\n\n    fn expose(t: &mut SplayTreeNode<M,\
    \ F>) -> &mut SplayTreeNode<M, F> {\n        let mut rp = null_mut();\n      \
    \  let mut cur: *mut _ = t;\n        while let Some(c) = unsafe { cur.as_mut()\
    \ } {\n            c.splay();\n            c.rch = rp;\n            SplayTreeNode::update(c);\n\
    \            rp = c;\n            cur = c.par;\n        }\n        t.splay();\n\
    \        unsafe { rp.as_mut() }.unwrap()\n    }\n\n    fn evert_(t: &mut SplayTreeNode<M,\
    \ F>) {\n        Self::expose(t);\n        SplayTreeNode::toggle(t);\n       \
    \ SplayTreeNode::push(t);\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/splay-tree-internal/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/link-cut-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-21 23:45:22+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/dynamic_tree_vertex_set_path_composite/src/main.rs
  - verify/dynamic_tree_vertex_add_path_sum/src/main.rs
documentation_of: crates/data-structure/link-cut-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/link-cut-tree/src/lib.rs
- /library/crates/data-structure/link-cut-tree/src/lib.rs.html
title: crates/data-structure/link-cut-tree/src/lib.rs
---
