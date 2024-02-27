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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ptr::null_mut;\n\nuse algebraic::{Act, Monoid};\nuse splay_tree_internal::SplayTreeNode;\n\
    \npub struct LinkCutTree<M, F>\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F:\
    \ Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    nodes: Vec<SplayTreeNode<M,\
    \ F>>,\n}\n\nimpl<M, F> From<&[M::S]> for LinkCutTree<M, F>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    fn\
    \ from(a: &[M::S]) -> Self {\n        Self {\n            nodes: a.iter().map(|x|\
    \ SplayTreeNode::new(x.clone())).collect(),\n        }\n    }\n}\n\nimpl<M, F>\
    \ LinkCutTree<M, F>\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F: Monoid +\
    \ Act<X = M::S>,\n    F::S: Clone,\n{\n    /// u \u306E\u89AA\u3092 v \u306B\u3059\
    \u308B\n    pub fn link(&mut self, u: usize, v: usize) {\n        Self::evert(&mut\
    \ self.nodes[u]);\n        Self::expose(&mut self.nodes[v]);\n        self.nodes[u].par\
    \ = &mut self.nodes[v];\n    }\n\n    pub fn cut(&mut self, u: usize, v: usize)\
    \ {\n        Self::evert(&mut self.nodes[u]);\n        Self::expose(&mut self.nodes[v]);\n\
    \        assert!(std::ptr::eq(self.nodes[u].par, &self.nodes[v]));\n        self.nodes[v].lch\
    \ = null_mut();\n        self.nodes[u].par = null_mut();\n        SplayTreeNode::update(&mut\
    \ self.nodes[v]);\n    }\n\n    pub fn set(&mut self, v: usize, val: M::S) {\n\
    \        let t = &mut self.nodes[v];\n        Self::evert(t);\n        t.val =\
    \ val;\n        SplayTreeNode::update(t);\n    }\n\n    pub fn get(&mut self,\
    \ v: usize) -> &M::S {\n        let t = &mut self.nodes[v];\n        Self::evert(t);\n\
    \        &t.val\n    }\n\n    pub fn apply(&mut self, u: usize, v: usize, f: F::S)\
    \ {\n        Self::evert(&mut self.nodes[u]);\n        Self::expose(&mut self.nodes[v]);\n\
    \        self.nodes[v].propagate(&f);\n    }\n\n    pub fn prod(&mut self, u:\
    \ usize, v: usize) -> M::S {\n        Self::evert(&mut self.nodes[u]);\n     \
    \   Self::expose(&mut self.nodes[v]);\n        self.nodes[v].prod.clone()\n  \
    \  }\n\n    fn expose(t: &mut SplayTreeNode<M, F>) -> &mut SplayTreeNode<M, F>\
    \ {\n        let mut rp = null_mut();\n        let mut cur: *mut _ = t;\n    \
    \    while let Some(c) = unsafe { cur.as_mut() } {\n            c.splay();\n \
    \           c.rch = rp;\n            SplayTreeNode::update(c);\n            rp\
    \ = c;\n            cur = c.par;\n        }\n        t.splay();\n        unsafe\
    \ { rp.as_mut() }.unwrap()\n    }\n\n    fn evert(t: &mut SplayTreeNode<M, F>)\
    \ {\n        Self::expose(t);\n        SplayTreeNode::toggle(t);\n        SplayTreeNode::push(t);\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/splay-tree-internal/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/link-cut-tree/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-11 19:02:37+09:00'
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
