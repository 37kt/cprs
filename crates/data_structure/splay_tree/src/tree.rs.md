---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
    title: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
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
  code: "use std::{cmp::Ordering, ptr::NonNull};\n\nuse as_half_open_range::AsHalfOpenRange;\n\
    \nuse crate::{allocator::new_ptr, node::Node, operator::Operator};\n\npub(crate)\
    \ struct Tree<O>\nwhere\n    O: Operator,\n{\n    pub(crate) root: Option<NonNull<Node<O>>>,\n\
    }\n\nimpl<O> Tree<O>\nwhere\n    O: Operator,\n{\n    pub(crate) fn len(&self)\
    \ -> usize {\n        self.root.map_or(0, |root| unsafe { root.as_ref().len })\n\
    \    }\n\n    pub(crate) fn is_empty(&self) -> bool {\n        self.root.is_none()\n\
    \    }\n\n    pub(crate) fn access(&mut self, mut i: usize) {\n        assert!(i\
    \ < self.len());\n        let mut root = self.root.unwrap();\n        loop {\n\
    \            let root_ref = unsafe { root.as_mut() };\n            root_ref.push();\n\
    \            for ch in root_ref.ch.iter_mut().flatten() {\n                let\
    \ ch_ref = unsafe { ch.as_mut() };\n                ch_ref.push();\n         \
    \   }\n\n            let l_len = root_ref.ch[0].map_or(0, |l| unsafe { l.as_ref().len\
    \ });\n            root = match i.cmp(&l_len) {\n                Ordering::Less\
    \ => root_ref.ch[0].unwrap(),\n                Ordering::Equal => {\n        \
    \            root_ref.splay();\n                    self.root = Some(root);\n\
    \                    return;\n                }\n                Ordering::Greater\
    \ => {\n                    i -= l_len + 1;\n                    root_ref.ch[1].unwrap()\n\
    \                }\n            }\n        }\n    }\n\n    pub(crate) fn append(&mut\
    \ self, mut right: Self) {\n        if self.is_empty() {\n            *self =\
    \ right;\n        } else if !right.is_empty() {\n            self.access(self.len()\
    \ - 1);\n            let self_ref = unsafe { self.root.as_mut().unwrap().as_mut()\
    \ };\n            let right_ref = unsafe { right.root.as_mut().unwrap().as_mut()\
    \ };\n            self_ref.push();\n            self_ref.ch[1] = right.root;\n\
    \            right_ref.par = self.root;\n            self_ref.update();\n    \
    \    }\n    }\n\n    pub(crate) fn split_at(&mut self, mid: usize) -> Self {\n\
    \        assert!(mid <= self.len());\n        if mid == 0 {\n            std::mem::take(self)\n\
    \        } else if mid == self.len() {\n            Self::default()\n        }\
    \ else {\n            self.access(mid - 1);\n            let self_ref = unsafe\
    \ { self.root.as_mut().unwrap().as_mut() };\n            self_ref.push();\n  \
    \          let mut right = std::mem::take(&mut self_ref.ch[1]);\n\n          \
    \  let right_ref = unsafe { right.as_mut().unwrap().as_mut() };\n            right_ref.par\
    \ = None;\n\n            self_ref.update();\n            Self { root: right }\n\
    \        }\n    }\n\n    pub(crate) fn reverse(&mut self, range: impl std::ops::RangeBounds<usize>)\
    \ {\n        let (l, r) = range.as_half_open_range(0, self.len());\n        if\
    \ l == r {\n            return;\n        }\n\n        let mut m = self.split_at(l);\n\
    \        let r = m.split_at(r - l);\n\n        let m_ref = unsafe { m.root.as_mut().unwrap().as_mut()\
    \ };\n        m_ref.rev ^= true;\n        m_ref.push();\n\n        self.append(m);\n\
    \        self.append(r);\n    }\n\n    pub(crate) fn insert(&mut self, i: usize,\
    \ x: O::X) {\n        assert!(i <= self.len());\n        let m = Tree {\n    \
    \        root: Some(new_ptr(Node::new(x))),\n        };\n        let r = self.split_at(i);\n\
    \        self.append(m);\n        self.append(r);\n    }\n\n    pub(crate) fn\
    \ remove(&mut self, i: usize) -> O::X {\n        assert!(i < self.len());\n  \
    \      let mut mr = self.split_at(i);\n        let r = mr.split_at(1);\n     \
    \   self.append(r);\n        unsafe { mr.root.as_mut().unwrap().as_mut() }.val.clone()\n\
    \    }\n\n    pub(crate) fn get(&mut self, i: usize) -> Option<&O::X> {\n    \
    \    if i >= self.len() {\n            None\n        } else {\n            self.access(i);\n\
    \            Some(&unsafe { self.root.as_mut().unwrap().as_mut() }.val)\n    \
    \    }\n    }\n\n    pub(crate) fn set(&mut self, i: usize, x: O::X) {\n     \
    \   assert!(i < self.len());\n        self.access(i);\n        let root_ref =\
    \ unsafe { self.root.as_mut().unwrap().as_mut() };\n        root_ref.val = x;\n\
    \        root_ref.update();\n    }\n\n    pub(crate) fn fold(&mut self, range:\
    \ impl std::ops::RangeBounds<usize>) -> O::P {\n        let (l, r) = range.as_half_open_range(0,\
    \ self.len());\n        if l == r {\n            return O::unit();\n        }\n\
    \        let mut m = self.split_at(l);\n        let r = m.split_at(r - l);\n \
    \       let m_ref = unsafe { m.root.as_mut().unwrap().as_mut() };\n        m_ref.update();\n\
    \        let res = m_ref.prod.clone();\n        self.append(m);\n        self.append(r);\n\
    \        res\n    }\n\n    pub(crate) fn apply(&mut self, range: impl std::ops::RangeBounds<usize>,\
    \ f: O::F) {\n        let (l, r) = range.as_half_open_range(0, self.len());\n\
    \        if l == r {\n            return;\n        }\n        let mut m = self.split_at(l);\n\
    \        let r = m.split_at(r - l);\n        let m_ref = unsafe { m.root.as_mut().unwrap().as_mut()\
    \ };\n        m_ref.act = O::compose(&m_ref.act, &f);\n        m_ref.push();\n\
    \        self.append(m);\n        self.append(r);\n    }\n}\n\nimpl<O> Default\
    \ for Tree<O>\nwhere\n    O: Operator,\n{\n    fn default() -> Self {\n      \
    \  Self { root: None }\n    }\n}\n\nimpl<O> Tree<O>\nwhere\n    O: Operator,\n\
    \    O::X: Default,\n{\n    pub(crate) fn new(n: usize) -> Self {\n        Self::from_fn(n,\
    \ |_| O::X::default())\n    }\n}\n\nimpl<O> Tree<O>\nwhere\n    O: Operator,\n\
    {\n    pub(crate) fn from_fn(n: usize, f: impl FnMut(usize) -> O::X) -> Self {\n\
    \        Self::from_iter((0..n).map(f))\n    }\n}\n\nimpl<O> FromIterator<O::X>\
    \ for Tree<O>\nwhere\n    O: Operator,\n{\n    fn from_iter<T: IntoIterator<Item\
    \ = O::X>>(iter: T) -> Self {\n        let mut iter = iter.into_iter();\n    \
    \    let mut root: NonNull<Node<O>> = match iter.next() {\n            Some(x)\
    \ => new_ptr(Node::new(x)),\n            None => return Self::default(),\n   \
    \     };\n\n        for x in iter {\n            let mut root_ref = unsafe { root.as_mut()\
    \ };\n            let mut new_root = new_ptr(Node::new(x));\n            let new_root_ref\
    \ = unsafe { new_root.as_mut() };\n            new_root_ref.ch[0] = Some(root);\n\
    \            root_ref.par = Some(new_root);\n            new_root_ref.update();\n\
    \            root = new_root;\n        }\n\n        Self { root: Some(root) }\n\
    \    }\n}\n"
  dependsOn:
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/node.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  isVerificationFile: false
  path: crates/data_structure/splay_tree/src/tree.rs
  requiredBy:
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/node.rs
  timestamp: '2025-04-09 07:52:13+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
documentation_of: crates/data_structure/splay_tree/src/tree.rs
layout: document
redirect_from:
- /library/crates/data_structure/splay_tree/src/tree.rs
- /library/crates/data_structure/splay_tree/src/tree.rs.html
title: crates/data_structure/splay_tree/src/tree.rs
---
