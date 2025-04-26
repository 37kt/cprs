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
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
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
  code: "use std::ptr::NonNull;\n\nuse crate::operator::Operator;\n\npub(crate) struct\
    \ Node<O>\nwhere\n    O: Operator,\n{\n    pub(crate) val: O::X,\n    pub(crate)\
    \ prod: O::P,\n    pub(crate) prod_rev: O::P,\n    pub(crate) act: O::F,\n   \
    \ pub(crate) par: Option<NonNull<Node<O>>>,\n    pub(crate) ch: [Option<NonNull<Node<O>>>;\
    \ 2],\n    pub(crate) len: usize,\n    pub(crate) rev: bool,\n}\n\nimpl<O> Node<O>\n\
    where\n    O: Operator,\n{\n    pub(crate) fn new(val: O::X) -> Self {\n     \
    \   Self {\n            prod: O::single(&val),\n            prod_rev: O::single(&val),\n\
    \            val,\n            act: O::unit_act(),\n            par: None,\n \
    \           ch: [None, None],\n            len: 1,\n            rev: false,\n\
    \        }\n    }\n\n    pub(crate) fn update(&mut self) {\n        self.len =\
    \ 1;\n        self.prod = O::single(&self.val);\n        self.prod_rev = O::single(&self.val);\n\
    \        if let Some(l) = self.ch[0].as_mut() {\n            let l = unsafe {\
    \ l.as_mut() };\n            l.push();\n            self.len += l.len;\n     \
    \       self.prod = O::op(&l.prod, &self.prod);\n            self.prod_rev = O::op(&self.prod_rev,\
    \ &l.prod_rev);\n        }\n        if let Some(r) = self.ch[1].as_mut() {\n \
    \           let r = unsafe { r.as_mut() };\n            r.push();\n          \
    \  self.len += r.len;\n            self.prod = O::op(&self.prod, &r.prod);\n \
    \           self.prod_rev = O::op(&r.prod_rev, &self.prod_rev);\n        }\n \
    \   }\n\n    pub(crate) fn push(&mut self) {\n        let act = std::mem::replace(&mut\
    \ self.act, O::unit_act());\n\n        if act != O::unit_act() {\n           \
    \ self.val = O::act_to_val(&self.val, &act);\n            self.prod = O::act_to_prod(&self.prod,\
    \ &act);\n            self.prod_rev = O::act_to_prod(&self.prod_rev, &act);\n\
    \            for mut ch in self.ch {\n                if let Some(ch) = ch.as_mut()\
    \ {\n                    let ch = unsafe { ch.as_mut() };\n                  \
    \  ch.act = O::compose(&ch.act, &act);\n                }\n            }\n   \
    \     }\n        if std::mem::take(&mut self.rev) {\n            self.ch.swap(0,\
    \ 1);\n            std::mem::swap(&mut self.prod, &mut self.prod_rev);\n     \
    \       for ch in self.ch.iter_mut() {\n                if let Some(ch) = ch.as_mut()\
    \ {\n                    let ch = unsafe { ch.as_mut() };\n                  \
    \  ch.rev ^= true;\n                }\n            }\n        }\n    }\n\n   \
    \ pub(crate) fn rotate(&mut self) {\n        let mut p = self.par.unwrap();\n\
    \        let p_ref = unsafe { p.as_mut() };\n        let mut g = p_ref.par;\n\
    \        self.push();\n\n        let dir = match self.pos() {\n            Pos::Left\
    \ => 0,\n            Pos::Right => 1,\n            Pos::Root => unreachable!(),\n\
    \        };\n        p_ref.ch[dir] = self.ch[dir ^ 1].take();\n        if let\
    \ Some(ch) = p_ref.ch[dir].as_mut() {\n            let ch = unsafe { ch.as_mut()\
    \ };\n            ch.par = Some(p);\n        }\n        self.ch[dir ^ 1] = Some(p);\n\
    \n        p_ref.par = NonNull::new(self);\n        self.par = g;\n        if let\
    \ Some(g) = g.as_mut() {\n            let g_ref = unsafe { g.as_mut() };\n   \
    \         if Some(p) == g_ref.ch[0] {\n                g_ref.ch[0] = NonNull::new(self);\n\
    \            } else {\n                g_ref.ch[1] = NonNull::new(self);\n   \
    \         }\n        }\n        p_ref.update();\n        self.update();\n    }\n\
    \n    pub(crate) fn splay(&mut self) {\n        while let Some(p) = self.par.as_mut()\
    \ {\n            let p_ref = unsafe { p.as_mut() };\n            if p_ref.pos()\
    \ != Pos::Root {\n                if self.pos() == p_ref.pos() {\n           \
    \         p_ref.rotate();\n                } else {\n                    self.rotate();\n\
    \                }\n            }\n            self.rotate();\n        }\n   \
    \ }\n\n    fn pos(&self) -> Pos {\n        if let Some(p) = self.par {\n     \
    \       let p_ref = unsafe { p.as_ref() };\n            if p_ref.ch[0].map_or(false,\
    \ |ch| std::ptr::eq(self, ch.as_ptr())) {\n                Pos::Left\n       \
    \     } else {\n                Pos::Right\n            }\n        } else {\n\
    \            Pos::Root\n        }\n    }\n}\n\n#[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\nenum Pos {\n    Left,\n    Right,\n    Root,\n}\n"
  dependsOn:
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/tree.rs
  isVerificationFile: false
  path: crates/data_structure/splay_tree/src/node.rs
  requiredBy:
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/tree.rs
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  timestamp: '2025-04-09 07:52:13+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
documentation_of: crates/data_structure/splay_tree/src/node.rs
layout: document
redirect_from:
- /library/crates/data_structure/splay_tree/src/node.rs
- /library/crates/data_structure/splay_tree/src/node.rs.html
title: crates/data_structure/splay_tree/src/node.rs
---
