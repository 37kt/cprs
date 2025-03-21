---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/misc/simple_arena/src/lib.rs
    title: crates/misc/simple_arena/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/persistent_union_find/src/lib.rs
    title: crates/data_structure/union_find/persistent_union_find/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::cell::RefCell;\nuse std::{ops::Index, ptr::NonNull};\n\nuse simple_arena::Arena;\n\
    \n/// \u5168\u6C38\u7D9A\u914D\u5217  \n/// len \u306F\u63D0\u4F9B\u3057\u306A\
    \u3044\n#[derive(Clone, Copy)]\npub struct PersistentArray<T, const M: usize =\
    \ 8> {\n    root: Option<NonNull<Node<T, M>>>,\n}\n\nstruct Node<T, const M: usize>\
    \ {\n    val: NonNull<T>,\n    ch: [Option<NonNull<Self>>; M],\n}\n\nimpl<T, const\
    \ M: usize> FromIterator<T> for PersistentArray<T, M> {\n    fn from_iter<I: IntoIterator<Item\
    \ = T>>(iter: I) -> Self {\n        let mut iter = iter.into_iter();\n\n     \
    \   let Some(first) = iter.next() else {\n            return Self { root: None\
    \ };\n        };\n\n        let root = new_ptr(Node::new(first));\n        let\
    \ mut n = 1;\n        while let Some(val) = iter.next() {\n            let mut\
    \ vp = root;\n            let mut i = n;\n            loop {\n               \
    \ let v = unsafe { vp.as_mut() };\n                let ci = i % M;\n         \
    \       i = (i - 1) / M;\n                if i == 0 {\n                    v.ch[ci]\
    \ = Some(new_ptr(Node::new(val)));\n                    break;\n             \
    \   } else {\n                    vp = v.ch[ci].unwrap();\n                }\n\
    \            }\n            n += 1;\n        }\n\n        Self { root: Some(root)\
    \ }\n    }\n}\n\nimpl<T, const M: usize> PersistentArray<T, M> {\n    pub fn from_fn(n:\
    \ usize, f: impl FnMut(usize) -> T) -> Self {\n        Self::from_iter((0..n).map(f))\n\
    \    }\n\n    pub fn is_empty(&self) -> bool {\n        self.root.is_none()\n\
    \    }\n\n    pub fn update_with(&self, mut i: usize, f: impl FnOnce(&T) -> T)\
    \ -> Self {\n        let root = unsafe { self.root.expect(\"out of range\").as_ref()\
    \ };\n        let new_root = new_ptr(root.clone());\n        let mut vp = new_root;\n\
    \        let mut x = unsafe { vp.as_ref().val };\n        while i != 0 {\n   \
    \         let v = unsafe { vp.as_mut() };\n            let ci = i % M;\n     \
    \       i = (i - 1) / M;\n            let c = unsafe { v.ch[ci].expect(\"out of\
    \ range\").as_ref() };\n            x = c.val;\n            let new_c = new_ptr(c.clone());\n\
    \            v.ch[ci] = Some(new_c);\n            vp = new_c;\n        }\n   \
    \     unsafe { vp.as_mut().val = new_ptr(f(x.as_ref())) };\n        Self {\n \
    \           root: Some(new_root),\n        }\n    }\n\n    pub fn set(&self, i:\
    \ usize, x: T) -> Self {\n        self.update_with(i, |_| x)\n    }\n}\n\nimpl<T,\
    \ const M: usize> Index<usize> for PersistentArray<T, M> {\n    type Output =\
    \ T;\n\n    fn index(&self, mut i: usize) -> &Self::Output {\n        let mut\
    \ vp = self.root.expect(\"out of range\");\n        while i != 0 {\n         \
    \   let v = unsafe { vp.as_ref() };\n            let ci = i % M;\n           \
    \ i = (i - 1) / M;\n            vp = v.ch[ci].expect(\"out of range\");\n    \
    \    }\n\n        unsafe { vp.as_ref().val.as_ref() }\n    }\n}\n\nimpl<T, const\
    \ M: usize> Node<T, M> {\n    fn new(val: T) -> Self {\n        Self {\n     \
    \       val: new_ptr(val),\n            ch: [None; M],\n        }\n    }\n}\n\n\
    impl<T, const M: usize> Clone for Node<T, M> {\n    fn clone(&self) -> Self {\n\
    \        Self {\n            val: self.val,\n            ch: self.ch,\n      \
    \  }\n    }\n}\n\nthread_local! {\n    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024\
    \ * 1024 * 1024));\n}\n\nfn new_ptr<T>(val: T) -> NonNull<T> {\n    ARENA.with(|arena|\
    \ NonNull::new(arena.borrow_mut().alloc(val)).unwrap())\n}\n"
  dependsOn:
  - crates/misc/simple_arena/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/persistent_array/src/lib.rs
  requiredBy:
  - crates/data_structure/union_find/persistent_union_find/src/lib.rs
  timestamp: '2025-03-11 07:52:05+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/persistent_array/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/persistent_array/src/lib.rs
- /library/crates/data_structure/persistent_array/src/lib.rs.html
title: crates/data_structure/persistent_array/src/lib.rs
---
