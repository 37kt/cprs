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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::BTreeMap;\n\n/// \u533A\u9593\u96C6\u5408\u3092\u7BA1\
    \u7406\u3059\u308B\u30C7\u30FC\u30BF\u69CB\u9020\u3002\n#[derive(Debug, Clone)]\n\
    pub struct IntervalSet<T: Clone + Ord>(BTreeMap<T, T>);\n\nimpl<T: Clone + Ord>\
    \ IntervalSet<T> {\n    /// \u7A7A\u306E IntervalSet \u3092\u69CB\u7BC9\u3059\u308B\
    \u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(1)\n    pub\
    \ fn new() -> Self {\n        Self(Default::default())\n    }\n\n    /// \u533A\
    \u9593 [l, r) \u3092\u8FFD\u52A0\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\
    \n    ///\n    /// - `l`: \u533A\u9593\u306E\u59CB\u70B9\n    /// - `r`: \u533A\
    \u9593\u306E\u7D42\u70B9\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n   \
    \ /// O(log N)\n    pub fn insert(&mut self, mut l: T, mut r: T) {\n        assert!(l\
    \ <= r);\n        if l == r {\n            return;\n        }\n\n        if let\
    \ Some((a, b)) = self.0.range(..=&l).next_back() {\n            if &l <= b {\n\
    \                l = a.clone();\n            }\n        }\n        for (a, b)\
    \ in self\n            .0\n            .range(&l..=&r)\n            .map(|(a,\
    \ b)| (a.clone(), b.clone()))\n            .collect::<Vec<_>>()\n        {\n \
    \           self.0.remove(&a);\n            r = r.max(b);\n        }\n       \
    \ self.0.insert(l, r);\n    }\n\n    /// lower \u4EE5\u4E0A\u306E\u5024\u306E\u3046\
    \u3061\u533A\u9593\u306B\u542B\u307E\u308C\u306A\u3044\u6700\u5C0F\u306E\u3082\
    \u306E\u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n\
    \    ///\n    /// - `lower`: \u4E0B\u9650\n    ///\n    /// # \u623B\u308A\u5024\
    \n    ///\n    /// - lower \u4EE5\u4E0A\u306E\u5024\u306E\u3046\u3061\u533A\u9593\
    \u306B\u542B\u307E\u308C\u306A\u3044\u6700\u5C0F\u306E\u3082\u306E\n    ///\n\
    \    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log N)\n    pub fn mex(&self,\
    \ lower: T) -> T {\n        if let Some((_, r)) = self.0.range(..=&lower).next_back()\
    \ {\n            lower.max(r.clone())\n        } else {\n            lower\n \
    \       }\n    }\n\n    /// x \u304C\u533A\u9593\u306B\u542B\u307E\u308C\u308B\
    \u304B\u3069\u3046\u304B\u3092\u5224\u5B9A\u3059\u308B\u3002\n    ///\n    ///\
    \ # \u5F15\u6570\n    ///\n    /// - `x`: \u5224\u5B9A\u3059\u308B\u5024\n   \
    \ ///\n    /// # \u623B\u308A\u5024\n    ///\n    /// - x \u304C\u533A\u9593\u306B\
    \u542B\u307E\u308C\u308B\u304B\u3069\u3046\u304B\n    ///\n    /// # \u8A08\u7B97\
    \u91CF\n    ///\n    /// O(log N)\n    pub fn contains(&self, x: T) -> bool {\n\
    \        if let Some((_, r)) = self.0.range(..=&x).next_back() {\n           \
    \ &x < r\n        } else {\n            false\n        }\n    }\n\n    /// \u533A\
    \u9593\u96C6\u5408\u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    /// # \u623B\
    \u308A\u5024\n    ///\n    /// - \u533A\u9593\u96C6\u5408\n    ///\n    /// #\
    \ \u8A08\u7B97\u91CF\n    ///\n    /// O(N)\n    pub fn intervals(&self) -> impl\
    \ Iterator<Item = (T, T)> {\n        self.0.clone().into_iter()\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/interval-set/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 08:18:46+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/interval-set/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/interval-set/src/lib.rs
- /library/crates/data-structure/interval-set/src/lib.rs.html
title: crates/data-structure/interval-set/src/lib.rs
---
