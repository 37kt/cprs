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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Bound, Neg, RangeBounds, Sub};\n\n/// \u4E8C\u6B21\u5143\
    \u7D2F\u7A4D\u548C\npub struct CumulativeSum2D<T>\nwhere\n    T: Clone + Default\
    \ + Add<T, Output = T> + Sub<T, Output = T>,\n{\n    v: Vec<Vec<T>>,\n    built:\
    \ bool,\n}\n\nimpl<T> CumulativeSum2D<T>\nwhere\n    T: Clone + Default + Add<T,\
    \ Output = T> + Sub<T, Output = T>,\n{\n    /// \u4E8C\u6B21\u5143\u914D\u5217\
    \u3092\u3059\u3079\u3066 0 \u3067\u521D\u671F\u5316\u3059\u308B\n    pub fn new(h:\
    \ usize, w: usize) -> Self {\n        Self {\n            v: vec![vec![T::default();\
    \ w + 1]; h + 1],\n            built: false,\n        }\n    }\n\n    /// \u5185\
    \u90E8\u8868\u73FE\u3092\u53D6\u5F97\u3059\u308B\n    pub fn inner(&self) -> &Vec<Vec<T>>\
    \ {\n        &self.v\n    }\n\n    /// a[i][j] += x\n    pub fn add(&mut self,\
    \ i: usize, j: usize, x: T) {\n        assert!(!self.built);\n        let i =\
    \ i + 1;\n        let j = j + 1;\n        if i >= self.v.len() || j >= self.v[0].len()\
    \ {\n            return;\n        }\n        self.v[i][j] = self.v[i][j].clone()\
    \ + x.clone();\n    }\n\n    /// \u7D2F\u7A4D\u548C\u3092\u8A08\u7B97\u3059\u308B\
    \n    pub fn build(&mut self) {\n        assert!(!self.built);\n        for i\
    \ in 0..self.v.len() - 1 {\n            for j in 0..self.v[0].len() {\n      \
    \          self.v[i + 1][j] = self.v[i + 1][j].clone() + self.v[i][j].clone();\n\
    \            }\n        }\n        for i in 0..self.v.len() {\n            for\
    \ j in 0..self.v[0].len() - 1 {\n                self.v[i][j + 1] = self.v[i][j\
    \ + 1].clone() + self.v[i][j].clone();\n            }\n        }\n        self.built\
    \ = true;\n    }\n\n    /// \u77E9\u5F62\u9818\u57DF\u306E\u548C\u3092\u6C42\u3081\
    \u308B\n    pub fn sum<RI, RJ>(&self, i: RI, j: RJ) -> T\n    where\n        RI:\
    \ RangeBounds<usize>,\n        RJ: RangeBounds<usize>,\n    {\n        assert!(self.built);\n\
    \        let (bi, ei) = range_to_pair(i, self.v.len() - 1);\n        let (bj,\
    \ ej) = range_to_pair(j, self.v[0].len() - 1);\n        self.v[ei][ej].clone()\
    \ + self.v[bi][bj].clone()\n            - self.v[ei][bj].clone()\n           \
    \ - self.v[bi][ej].clone()\n    }\n}\n\nimpl<T> CumulativeSum2D<T>\nwhere\n  \
    \  T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T> + Neg<Output =\
    \ T>,\n{\n    /// \u77E9\u5F62\u9818\u57DF\u306B\u52A0\u7B97\u3059\u308B\n   \
    \ pub fn imos_add<RI, RJ>(&mut self, i: RI, j: RJ, x: T)\n    where\n        RI:\
    \ RangeBounds<usize>,\n        RJ: RangeBounds<usize>,\n    {\n        assert!(!self.built);\n\
    \        let (bi, ei) = range_to_pair(i, self.v.len() - 1);\n        let (bj,\
    \ ej) = range_to_pair(j, self.v[0].len() - 1);\n        self.v[bi][bj] = self.v[bi][bj].clone()\
    \ + x.clone();\n        self.v[bi][ej] = self.v[bi][ej].clone() - x.clone();\n\
    \        self.v[ei][bj] = self.v[ei][bj].clone() - x.clone();\n        self.v[ei][ej]\
    \ = self.v[ei][ej].clone() + x;\n    }\n\n    /// 1 \u70B9\u306E\u5024\u3092\u53D6\
    \u5F97\u3059\u308B\n    pub fn imos_get(&self, i: usize, j: usize) -> T {\n  \
    \      assert!(self.built);\n        self.v[i][j].clone()\n    }\n}\n\nimpl<T>\
    \ From<Vec<Vec<T>>> for CumulativeSum2D<T>\nwhere\n    T: Clone + Default + Add<T,\
    \ Output = T> + Sub<T, Output = T>,\n{\n    /// \u4E8C\u6B21\u5143\u914D\u5217\
    \u3067\u521D\u671F\u5316\u3059\u308B\n    fn from(mut v: Vec<Vec<T>>) -> Self\
    \ {\n        assert!(v.iter().all(|x| x.len() == v[0].len()));\n        for x\
    \ in v.iter_mut() {\n            x.insert(0, T::default());\n        }\n     \
    \   v.insert(0, vec![T::default(); v[0].len()]);\n        Self { v, built: false\
    \ }\n    }\n}\n\nfn range_to_pair<R>(range: R, n: usize) -> (usize, usize)\nwhere\n\
    \    R: RangeBounds<usize>,\n{\n    let l = match range.start_bound() {\n    \
    \    Bound::Included(&l) => l,\n        Bound::Excluded(&l) => l + 1,\n      \
    \  Bound::Unbounded => 0,\n    };\n    let r = match range.end_bound() {\n   \
    \     Bound::Included(&r) => (r + 1).min(n),\n        Bound::Excluded(&r) => r.min(n),\n\
    \        Bound::Unbounded => n,\n    };\n    (l, r)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/cumulative-sum-2d/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 08:18:46+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/cumulative-sum-2d/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/cumulative-sum-2d/src/lib.rs
- /library/crates/data-structure/cumulative-sum-2d/src/lib.rs.html
title: crates/data-structure/cumulative-sum-2d/src/lib.rs
---
