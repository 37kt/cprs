---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/deque_operate_all_composite/src/main.rs
    title: verify/library_checker/data_structure/deque_operate_all_composite/src/main.rs
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
  code: "use algebraic_traits::Monoid;\n\npub struct FoldableDeque<M: Monoid> {\n\
    \    fv: Vec<M::Value>,\n    bv: Vec<M::Value>,\n    fs: Vec<M::Value>,\n    bs:\
    \ Vec<M::Value>,\n}\n\nimpl<M: Monoid> FromIterator<M::Value> for FoldableDeque<M>\
    \ {\n    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {\n \
    \       let mut v = iter.into_iter().collect::<Vec<_>>();\n        let len = v.len();\n\
    \        let mut dq = Self::new();\n        for x in v.drain(len / 2..) {\n  \
    \          dq.push_back(x);\n        }\n        for x in v.drain(..).rev() {\n\
    \            dq.push_front(x);\n        }\n        dq\n    }\n}\n\nimpl<M: Monoid>\
    \ Default for FoldableDeque<M> {\n    fn default() -> Self {\n        Self::new()\n\
    \    }\n}\n\nimpl<M: Monoid> FoldableDeque<M> {\n    pub fn new() -> Self {\n\
    \        Self {\n            fv: vec![],\n            bv: vec![],\n          \
    \  fs: vec![M::unit()],\n            bs: vec![M::unit()],\n        }\n    }\n\n\
    \    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {\n  \
    \      Self::from_iter((0..n).map(f))\n    }\n\n    pub fn clear(&mut self) {\n\
    \        self.fv.clear();\n        self.bv.clear();\n        self.fs.clear();\n\
    \        self.bs.clear();\n        self.fs.push(M::unit());\n        self.bs.push(M::unit());\n\
    \    }\n\n    pub fn len(&self) -> usize {\n        self.fv.len() + self.bv.len()\n\
    \    }\n\n    pub fn is_empty(&self) -> bool {\n        self.fv.is_empty() &&\
    \ self.bv.is_empty()\n    }\n\n    pub fn push_front(&mut self, x: M::Value) {\n\
    \        self.fs.push(M::op(&x, self.fs.last().unwrap()));\n        self.fv.push(x);\n\
    \    }\n\n    pub fn push_back(&mut self, x: M::Value) {\n        self.bs.push(M::op(self.bs.last().unwrap(),\
    \ &x));\n        self.bv.push(x);\n    }\n\n    pub fn pop_front(&mut self) ->\
    \ Option<M::Value> {\n        match self.len() {\n            0 => None,\n   \
    \         1 => {\n                if self.bv.len() == 1 {\n                  \
    \  self.bs.pop();\n                    self.bv.pop()\n                } else {\n\
    \                    self.fs.pop();\n                    self.fv.pop()\n     \
    \           }\n            }\n            _ => {\n                if self.fv.is_empty()\
    \ {\n                    self.rebuild();\n                }\n                self.fs.pop();\n\
    \                self.fv.pop()\n            }\n        }\n    }\n\n    pub fn\
    \ pop_back(&mut self) -> Option<M::Value> {\n        match self.len() {\n    \
    \        0 => None,\n            1 => {\n                if self.bv.len() == 1\
    \ {\n                    self.bs.pop();\n                    self.bv.pop()\n \
    \               } else {\n                    self.fs.pop();\n               \
    \     self.fv.pop()\n                }\n            }\n            _ => {\n  \
    \              if self.bv.is_empty() {\n                    self.rebuild();\n\
    \                }\n                self.bs.pop();\n                self.bv.pop()\n\
    \            }\n        }\n    }\n\n    pub fn front(&self) -> Option<&M::Value>\
    \ {\n        self.fv.last().or(self.bv.first())\n    }\n\n    pub fn back(&self)\
    \ -> Option<&M::Value> {\n        self.bv.last().or(self.fv.first())\n    }\n\n\
    \    pub fn fold_all(&self) -> M::Value {\n        M::op(self.fs.last().unwrap(),\
    \ self.bs.last().unwrap())\n    }\n\n    fn rebuild(&mut self) {\n        let\
    \ len = self.len();\n        let mut v = self\n            .fv\n            .drain(..)\n\
    \            .rev()\n            .chain(self.bv.drain(..))\n            .collect::<Vec<_>>();\n\
    \        self.clear();\n        for x in v.drain(len / 2..) {\n            self.push_back(x);\n\
    \        }\n        for x in v.drain(..).rev() {\n            self.push_front(x);\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/data_structure/foldable_deque/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/deque_operate_all_composite/src/main.rs
documentation_of: crates/data_structure/foldable_deque/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/foldable_deque/src/lib.rs
- /library/crates/data_structure/foldable_deque/src/lib.rs.html
title: crates/data_structure/foldable_deque/src/lib.rs
---
