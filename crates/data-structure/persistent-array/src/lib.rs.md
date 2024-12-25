---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/persistent-union-find/src/lib.rs
    title: crates/data-structure/persistent-union-find/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::rc::Rc;\n\n/// \u6C38\u7D9A\u914D\u5217\n#[derive(Clone)]\npub struct\
    \ PersistentArray<T: Clone, const M: usize = 8> {\n    val: Option<Rc<T>>,\n \
    \   ch: Option<Box<[Rc<Self>]>>,\n}\n\nimpl<T: Clone, const M: usize> From<Vec<T>>\
    \ for PersistentArray<T, M> {\n    /// Vec \u304B\u3089\u6C38\u7D9A\u914D\u5217\
    \u3092\u69CB\u7BC9\u3059\u308B\u3002\n    fn from(v: Vec<T>) -> Self {\n     \
    \   let mut res = Self::new();\n        for (i, x) in v.into_iter().enumerate()\
    \ {\n            res = res.set(i, x);\n        }\n        res\n    }\n}\n\nimpl<T:\
    \ Clone, const M: usize> PersistentArray<T, M> {\n    /// \u7A7A\u306E\u6C38\u7D9A\
    \u914D\u5217\u3092\u69CB\u7BC9\u3059\u308B\u3002\n    pub fn new() -> Self {\n\
    \        Self {\n            val: None,\n            ch: None,\n        }\n  \
    \  }\n\n    /// a\\[i\\] \u3092 x \u306B\u66F4\u65B0\u3059\u308B\u3002\n    pub\
    \ fn set(&self, i: usize, x: T) -> Self {\n        let mut v = self.clone();\n\
    \        if i == 0 {\n            v.val = Some(Rc::new(x));\n            v\n \
    \       } else {\n            if v.ch.is_none() {\n                v.ch = Some(vec![Rc::new(Self::new());\
    \ M].into_boxed_slice());\n            }\n            let t = v.ch.as_ref().unwrap()[i\
    \ % M].set((i - 1) / M, x);\n            v.ch.as_mut().unwrap()[i % M] = Rc::new(t);\n\
    \            v\n        }\n    }\n\n    /// a\\[i\\] \u3092\u53D6\u5F97\u3059\u308B\
    \u3002\n    pub fn get(&self, i: usize) -> Option<&T> {\n        if i == 0 {\n\
    \            self.val.as_ref().map(|v| v.as_ref())\n        } else {\n       \
    \     self.ch.as_ref().and_then(|ch| ch[i % M].get((i - 1) / M))\n        }\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/persistent-array/src/lib.rs
  requiredBy:
  - crates/data-structure/persistent-union-find/src/lib.rs
  timestamp: '2024-12-25 10:01:29+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/persistent-array/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/persistent-array/src/lib.rs
- /library/crates/data-structure/persistent-array/src/lib.rs.html
title: crates/data-structure/persistent-array/src/lib.rs
---
