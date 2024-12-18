---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/quotients/src/lib.rs
    title: crates/number-theory/quotients/src/lib.rs
  _extendedRequiredBy: []
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
  code: "use std::{\n    fmt::Debug,\n    ops::{Index, IndexMut},\n};\n\nuse quotients::quotients;\n\
    \n#[derive(Clone)]\npub struct QuotientsArray<T> {\n    n: usize,\n    sqrt_n:\
    \ usize,\n    quotients: Vec<usize>,\n    data: Vec<T>,\n}\n\nimpl<T> QuotientsArray<T>\
    \ {\n    pub fn from_fn(n: usize, f: impl FnMut(usize) -> T) -> Self {\n     \
    \   let sqrt_n = (n as f64).sqrt().floor() as usize;\n        let data = quotients(n).map(f).collect::<Vec<_>>();\n\
    \        let quotients = quotients(n).collect::<Vec<_>>();\n        Self {\n \
    \           n,\n            sqrt_n,\n            quotients,\n            data,\n\
    \        }\n    }\n\n    pub fn n(&self) -> usize {\n        self.n\n    }\n\n\
    \    pub fn sqrt_n(&self) -> usize {\n        self.sqrt_n\n    }\n\n    pub fn\
    \ quotients(&self) -> &[usize] {\n        &self.quotients\n    }\n\n    pub fn\
    \ len(&self) -> usize {\n        self.data.len()\n    }\n\n    fn index(&self,\
    \ x: usize) -> usize {\n        assert!(1 <= x && x <= self.n);\n        if x\
    \ <= self.sqrt_n {\n            self.data.len() - x\n        } else {\n      \
    \      self.n / x - 1\n        }\n    }\n}\n\nimpl<T> Index<usize> for QuotientsArray<T>\
    \ {\n    type Output = T;\n    fn index(&self, x: usize) -> &Self::Output {\n\
    \        let i = self.index(x);\n        unsafe { self.data.get_unchecked(i) }\n\
    \    }\n}\n\nimpl<T> IndexMut<usize> for QuotientsArray<T> {\n    fn index_mut(&mut\
    \ self, x: usize) -> &mut Self::Output {\n        let i = self.index(x);\n   \
    \     unsafe { self.data.get_unchecked_mut(i) }\n    }\n}\n\nimpl<T: Debug> Debug\
    \ for QuotientsArray<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        f.write_str(\"{\")?;\n        for i in 0..self.data.len()\
    \ {\n            if i > 0 {\n                f.write_str(\", \")?;\n         \
    \   }\n            f.write_fmt(format_args!(\"{}: {:?}\", self.quotients[i], self.data[i]))?;\n\
    \        }\n        f.write_str(\"}\")\n    }\n}\n"
  dependsOn:
  - crates/number-theory/quotients/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/quotients-array/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-18 03:30:26+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/quotients-array/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/quotients-array/src/lib.rs
- /library/crates/data-structure/quotients-array/src/lib.rs.html
title: crates/data-structure/quotients-array/src/lib.rs
---
