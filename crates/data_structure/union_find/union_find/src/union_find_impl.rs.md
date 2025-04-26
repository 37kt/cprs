---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find.rs
    title: crates/data_structure/union_find/union_find/src/union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find.rs
    title: crates/data_structure/union_find/union_find/src/union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind/src/main.rs
    title: verify/library_checker/data_structure/unionfind/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic_traits::{Commutative, Group, Semigroup, Unital};\n\nstruct\
    \ Node<Potential, Aggregate>\nwhere\n    Potential: Group,\n    Potential::Value:\
    \ Clone,\n    Aggregate: Commutative + Semigroup,\n    Aggregate::Value: Clone,\n\
    {\n    par_sz: i32,\n    pot: Potential::Value,\n    sum: Aggregate::Value,\n\
    }\n\nimpl<Potential, Aggregate> Clone for Node<Potential, Aggregate>\nwhere\n\
    \    Potential: Group,\n    Potential::Value: Clone,\n    Aggregate: Commutative\
    \ + Semigroup,\n    Aggregate::Value: Clone,\n{\n    fn clone(&self) -> Self {\n\
    \        Self {\n            par_sz: self.par_sz,\n            pot: self.pot.clone(),\n\
    \            sum: self.sum.clone(),\n        }\n    }\n}\n\npub(crate) struct\
    \ UnionFindImpl<Potential, Aggregate, const UNDOABLE: bool>\nwhere\n    Potential:\
    \ Group,\n    Potential::Value: Clone,\n    Aggregate: Commutative + Semigroup,\n\
    \    Aggregate::Value: Clone,\n{\n    nodes: Vec<Node<Potential, Aggregate>>,\n\
    \    history: Vec<(u32, Node<Potential, Aggregate>)>,\n}\n\nimpl<Potential, Aggregate,\
    \ const UNDOABLE: bool> Clone\n    for UnionFindImpl<Potential, Aggregate, UNDOABLE>\n\
    where\n    Potential: Group,\n    Potential::Value: Clone,\n    Aggregate: Commutative\
    \ + Semigroup,\n    Aggregate::Value: Clone,\n{\n    fn clone(&self) -> Self {\n\
    \        Self {\n            nodes: self.nodes.clone(),\n            history:\
    \ self.history.clone(),\n        }\n    }\n}\n\nimpl<Potential, Aggregate, const\
    \ UNDOABLE: bool> FromIterator<Aggregate::Value>\n    for UnionFindImpl<Potential,\
    \ Aggregate, UNDOABLE>\nwhere\n    Potential: Group,\n    Potential::Value: Clone,\n\
    \    Aggregate: Commutative + Semigroup,\n    Aggregate::Value: Clone,\n{\n  \
    \  fn from_iter<T: IntoIterator<Item = Aggregate::Value>>(iter: T) -> Self {\n\
    \        let nodes = iter\n            .into_iter()\n            .map(|sum| Node\
    \ {\n                par_sz: -1,\n                pot: Potential::unit(),\n  \
    \              sum,\n            })\n            .collect();\n        Self {\n\
    \            nodes,\n            history: vec![],\n        }\n    }\n}\n\nimpl<Potential,\
    \ Aggregate, const UNDOABLE: bool> UnionFindImpl<Potential, Aggregate, UNDOABLE>\n\
    where\n    Potential: Group,\n    Potential::Value: Clone,\n    Aggregate: Commutative\
    \ + Semigroup,\n    Aggregate::Value: Clone,\n{\n    pub(crate) fn from_fn(n:\
    \ usize, f: impl FnMut(usize) -> Aggregate::Value) -> Self {\n        Self::from_iter((0..n).map(f))\n\
    \    }\n}\n\nimpl<Potential, Aggregate, const UNDOABLE: bool> UnionFindImpl<Potential,\
    \ Aggregate, UNDOABLE>\nwhere\n    Potential: Group,\n    Potential::Value: Clone,\n\
    \    Aggregate: Commutative + Semigroup + Unital,\n    Aggregate::Value: Clone,\n\
    {\n    pub(crate) fn new(n: usize) -> Self {\n        Self::from_fn(n, |_| Aggregate::unit())\n\
    \    }\n}\n\nimpl<Potential, Aggregate, const UNDOABLE: bool> UnionFindImpl<Potential,\
    \ Aggregate, UNDOABLE>\nwhere\n    Potential: Group,\n    Potential::Value: Clone,\n\
    \    Aggregate: Commutative + Semigroup,\n    Aggregate::Value: Clone,\n{\n  \
    \  pub(crate) fn len(&self) -> usize {\n        self.nodes.len()\n    }\n\n  \
    \  pub(crate) fn is_empty(&self) -> bool {\n        self.nodes.is_empty()\n  \
    \  }\n\n    pub(crate) fn merge_with(\n        &mut self,\n        x: usize,\n\
    \        y: usize,\n        mut d: Potential::Value,\n        mut f: impl FnMut(usize,\
    \ usize),\n    ) -> bool {\n        let (mut x, px) = self.find(x);\n        let\
    \ (mut y, py) = self.find(y);\n\n        if UNDOABLE {\n            self.history.push((x\
    \ as u32, self.nodes[x].clone()));\n            self.history.push((y as u32, self.nodes[y].clone()));\n\
    \        }\n\n        if x == y {\n            return false;\n        }\n\n  \
    \      d = Potential::op(&Potential::op(&px, &d), &Potential::inv(&py));\n\n \
    \       if -self.nodes[x].par_sz < -self.nodes[y].par_sz {\n            std::mem::swap(&mut\
    \ x, &mut y);\n            d = Potential::inv(&d);\n        }\n\n        self.nodes[x].par_sz\
    \ += self.nodes[y].par_sz;\n        self.nodes[x].sum = Aggregate::op(&self.nodes[x].sum,\
    \ &self.nodes[y].sum);\n        self.nodes[y].par_sz = x as i32;\n        self.nodes[y].pot\
    \ = d;\n\n        f(x, y);\n\n        true\n    }\n\n    pub(crate) fn find(&mut\
    \ self, x: usize) -> (usize, Potential::Value) {\n        assert!(x < self.nodes.len());\n\
    \n        if UNDOABLE {\n            let mut x = x as i32;\n            let mut\
    \ pot = Potential::unit();\n            loop {\n                let p = self.nodes[x\
    \ as usize].par_sz;\n                if p < 0 {\n                    break;\n\
    \                }\n                pot = Potential::op(&self.nodes[x as usize].pot,\
    \ &pot);\n                x = p;\n            }\n            (x as usize, pot)\n\
    \        } else {\n            let mut x = x as i32;\n            let mut pot\
    \ = Potential::unit();\n            loop {\n                let p = self.nodes[x\
    \ as usize].par_sz;\n                if p < 0 {\n                    break;\n\
    \                }\n                self.nodes[x as usize].pot =\n           \
    \         Potential::op(&self.nodes[p as usize].pot, &self.nodes[x as usize].pot);\n\
    \                pot = Potential::op(&self.nodes[x as usize].pot, &pot);\n   \
    \             let pp = self.nodes[p as usize].par_sz;\n                if pp >=\
    \ 0 {\n                    self.nodes[x as usize].par_sz = pp;\n             \
    \       x = pp;\n                } else {\n                    x = p;\n      \
    \          }\n            }\n            (x as usize, pot)\n        }\n    }\n\
    \n    pub(crate) fn root(&mut self, x: usize) -> usize {\n        self.find(x).0\n\
    \    }\n\n    pub(crate) fn potential(&mut self, x: usize) -> Potential::Value\
    \ {\n        self.find(x).1\n    }\n\n    pub(crate) fn size(&mut self, x: usize)\
    \ -> usize {\n        let x = self.root(x);\n        -self.nodes[x].par_sz as\
    \ usize\n    }\n\n    pub(crate) fn same(&mut self, x: usize, y: usize) -> bool\
    \ {\n        self.root(x) == self.root(y)\n    }\n\n    pub(crate) fn diff(&mut\
    \ self, x: usize, y: usize) -> Option<Potential::Value> {\n        let (x, px)\
    \ = self.find(x);\n        let (y, py) = self.find(y);\n        (x == y).then_some(Potential::op(&Potential::inv(&px),\
    \ &py))\n    }\n\n    pub(crate) fn component_sum(&mut self, x: usize) -> Aggregate::Value\
    \ {\n        let x = self.root(x);\n        self.nodes[x].sum.clone()\n    }\n\
    }\n\nimpl<Potential, Aggregate> UnionFindImpl<Potential, Aggregate, true>\nwhere\n\
    \    Potential: Group,\n    Potential::Value: Clone,\n    Aggregate: Commutative\
    \ + Semigroup,\n    Aggregate::Value: Clone,\n{\n    pub(crate) fn undo(&mut self)\
    \ {\n        for _ in 0..2 {\n            let (x, node) = self.history.pop().unwrap();\n\
    \            self.nodes[x as usize] = node;\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  isVerificationFile: false
  path: crates/data_structure/union_find/union_find/src/union_find_impl.rs
  requiredBy:
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find.rs
  timestamp: '2025-03-07 01:17:39+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
  - verify/library_checker/data_structure/unionfind/src/main.rs
documentation_of: crates/data_structure/union_find/union_find/src/union_find_impl.rs
layout: document
redirect_from:
- /library/crates/data_structure/union_find/union_find/src/union_find_impl.rs
- /library/crates/data_structure/union_find/union_find/src/union_find_impl.rs.html
title: crates/data_structure/union_find/union_find/src/union_find_impl.rs
---
