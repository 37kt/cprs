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
  code: "#[derive(Clone, Copy)]\nenum Inner<K, V> {\n    Empty,\n    One(K, V),\n\
    \    Two([(K, V); 2]),\n}\n\n#[derive(Clone, Copy)]\npub struct Top2<K, V>\nwhere\n\
    \    K: Eq + Copy,\n    V: Ord + Copy,\n{\n    inner: Inner<K, V>,\n}\n\nimpl<K,\
    \ V> Top2<K, V>\nwhere\n    K: Eq + Copy,\n    V: Ord + Copy,\n{\n    pub fn new()\
    \ -> Self {\n        Top2 {\n            inner: Inner::Empty,\n        }\n   \
    \ }\n\n    pub fn push(&mut self, key: K, value: V) {\n        match self.inner\
    \ {\n            Inner::Empty => {\n                self.inner = Inner::One(key,\
    \ value);\n            }\n            Inner::One(k, v) => {\n                if\
    \ k == key {\n                    self.inner = Inner::One(k, v.max(value));\n\
    \                } else {\n                    self.inner = Inner::Two([(k, v),\
    \ (key, value)]);\n                }\n            }\n            Inner::Two([(k1,\
    \ v1), (k2, v2)]) => {\n                if k1 == key {\n                    self.inner\
    \ = Inner::Two([(k1, v1.max(value)), (k2, v2)]);\n                } else if k2\
    \ == key {\n                    self.inner = Inner::Two([(k1, v1), (k2, v2.max(value))]);\n\
    \                } else if v1 < value {\n                    self.inner = Inner::Two([(key,\
    \ value), (k1, v1)]);\n                } else if v2 < value {\n              \
    \      self.inner = Inner::Two([(k1, v1), (key, value)]);\n                }\n\
    \            }\n        }\n        match self.inner {\n            Inner::Two([(k1,\
    \ v1), (k2, v2)]) if v1 < v2 => {\n                self.inner = Inner::Two([(k2,\
    \ v2), (k1, v1)]);\n            }\n            _ => {}\n        }\n    }\n\n \
    \   pub fn append(&mut self, other: &Self) {\n        match other.inner {\n  \
    \          Inner::Empty => {}\n            Inner::One(k, v) => {\n           \
    \     self.push(k, v);\n            }\n            Inner::Two([(k1, v1), (k2,\
    \ v2)]) => {\n                self.push(k1, v1);\n                self.push(k2,\
    \ v2);\n            }\n        }\n    }\n\n    /// key \u4EE5\u5916\u306E\u6700\
    \u5927\u5024\u3092\u53D6\u5F97\u3059\u308B\n    pub fn get(&self, key: K) -> Option<V>\
    \ {\n        match self.inner {\n            Inner::Empty => None,\n         \
    \   Inner::One(k, v) => {\n                if k == key {\n                   \
    \ None\n                } else {\n                    Some(v)\n              \
    \  }\n            }\n            Inner::Two([(k1, v1), (_, v2)]) => {\n      \
    \          if k1 == key {\n                    Some(v2)\n                } else\
    \ {\n                    Some(v1)\n                }\n            }\n        }\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/top2/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-18 00:43:38+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/top2/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/top2/src/lib.rs
- /library/crates/data-structure/top2/src/lib.rs.html
title: crates/data-structure/top2/src/lib.rs
---
