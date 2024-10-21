---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::BTreeMap;\n\n#[derive(Debug, Clone)]\npub struct IntervalSet<T:\
    \ Clone + Ord>(BTreeMap<T, T>);\n\nimpl<T: Clone + Ord> IntervalSet<T> {\n   \
    \ pub fn new() -> Self {\n        Self(Default::default())\n    }\n\n    pub fn\
    \ insert(&mut self, mut l: T, mut r: T) {\n        assert!(l <= r);\n        if\
    \ l == r {\n            return;\n        }\n\n        if let Some((a, b)) = self.0.range(..=&l).next_back()\
    \ {\n            if &l <= b {\n                l = a.clone();\n            }\n\
    \        }\n        for (a, b) in self\n            .0\n            .range(&l..=&r)\n\
    \            .map(|(a, b)| (a.clone(), b.clone()))\n            .collect::<Vec<_>>()\n\
    \        {\n            self.0.remove(&a);\n            r = r.max(b);\n      \
    \  }\n        self.0.insert(l, r);\n    }\n\n    pub fn mex(&self, lower: T) ->\
    \ T {\n        if let Some((_, r)) = self.0.range(..=&lower).next_back() {\n \
    \           lower.max(r.clone())\n        } else {\n            lower\n      \
    \  }\n    }\n\n    pub fn contains(&self, x: T) -> bool {\n        if let Some((_,\
    \ r)) = self.0.range(..=&x).next_back() {\n            &x < r\n        } else\
    \ {\n            false\n        }\n    }\n\n    pub fn intervals(&self) -> impl\
    \ Iterator<Item = (T, T)> {\n        self.0.clone().into_iter()\n    }\n}\n\n\
    #[cfg(test)]\nmod tests {\n    use super::*;\n    #[test]\n    fn test() {\n \
    \       let mut set = IntervalSet::new();\n        set.insert(10, 20);\n     \
    \   set.insert(20, 30);\n        set.insert(0, 10);\n        set.insert(31, 40);\n\
    \        set.insert(30, 31);\n        set.insert(-1, 100);\n        set.insert(-1000,\
    \ -1000);\n        dbg!(&set);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/interval-set/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-23 15:04:49+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/interval-set/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/interval-set/src/lib.rs
- /library/crates/data-structure/interval-set/src/lib.rs.html
title: crates/data-structure/interval-set/src/lib.rs
---
