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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Bound, Neg, RangeBounds, Sub};\n\npub struct CumulativeSum2D<T>\n\
    where\n    T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T>,\n{\n\
    \    v: Vec<Vec<T>>,\n    built: bool,\n}\n\nimpl<T> CumulativeSum2D<T>\nwhere\n\
    \    T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T>,\n{\n    pub\
    \ fn new(h: usize, w: usize) -> Self {\n        Self {\n            v: vec![vec![T::default();\
    \ w + 1]; h + 1],\n            built: false,\n        }\n    }\n\n    pub fn inner(&self)\
    \ -> &Vec<Vec<T>> {\n        &self.v\n    }\n\n    pub fn add(&mut self, i: usize,\
    \ j: usize, x: T) {\n        assert!(!self.built);\n        if i >= self.v.len()\
    \ || j >= self.v[0].len() {\n            return;\n        }\n        self.v[i][j]\
    \ = self.v[i][j].clone() + x.clone();\n    }\n\n    pub fn build(&mut self) {\n\
    \        assert!(!self.built);\n        for i in 1..self.v.len() {\n         \
    \   for j in 1..self.v[i].len() {\n                self.v[i][j] =\n          \
    \          self.v[i][j].clone() + self.v[i][j - 1].clone() + self.v[i - 1][j].clone()\n\
    \                        - self.v[i - 1][j - 1].clone();\n            }\n    \
    \    }\n        self.built = true;\n    }\n\n    pub fn sum<RI, RJ>(&self, i:\
    \ RI, j: RJ) -> T\n    where\n        RI: RangeBounds<usize>,\n        RJ: RangeBounds<usize>,\n\
    \    {\n        assert!(self.built);\n        let (bi, ei) = range_to_pair(i,\
    \ self.v.len() - 1);\n        let (bj, ej) = range_to_pair(j, self.v[0].len()\
    \ - 1);\n        self.v[ei][ej].clone() + self.v[bi][bj].clone()\n           \
    \ - self.v[ei][bj].clone()\n            - self.v[bi][ej].clone()\n    }\n}\n\n\
    impl<T> CumulativeSum2D<T>\nwhere\n    T: Clone + Default + Add<T, Output = T>\
    \ + Sub<T, Output = T> + Neg<Output = T>,\n{\n    pub fn imos_add<RI, RJ>(&mut\
    \ self, i: RI, j: RJ, x: T)\n    where\n        RI: RangeBounds<usize>,\n    \
    \    RJ: RangeBounds<usize>,\n    {\n        assert!(!self.built);\n        let\
    \ (bi, ei) = range_to_pair(i, self.v.len() - 1);\n        let (bj, ej) = range_to_pair(j,\
    \ self.v[0].len() - 1);\n        self.add(bi, bj, x.clone());\n        self.add(bi,\
    \ ej, -x.clone());\n        self.add(ei, bj, -x.clone());\n        self.add(ei,\
    \ ej, x);\n    }\n\n    pub fn imos_get(&self, i: usize, j: usize) -> T {\n  \
    \      assert!(self.built);\n        self.v[i + 1][j + 1].clone()\n    }\n}\n\n\
    impl<T> From<Vec<Vec<T>>> for CumulativeSum2D<T>\nwhere\n    T: Clone + Default\
    \ + Add<T, Output = T> + Sub<T, Output = T>,\n{\n    fn from(mut v: Vec<Vec<T>>)\
    \ -> Self {\n        assert!(v.iter().all(|x| x.len() == v[0].len()));\n     \
    \   for x in v.iter_mut() {\n            x.insert(0, T::default());\n        }\n\
    \        v.insert(0, vec![T::default(); v[0].len()]);\n        Self { v, built:\
    \ false }\n    }\n}\n\nfn range_to_pair<R>(range: R, n: usize) -> (usize, usize)\n\
    where\n    R: RangeBounds<usize>,\n{\n    let l = match range.start_bound() {\n\
    \        Bound::Included(&l) => l,\n        Bound::Excluded(&l) => l + 1,\n  \
    \      Bound::Unbounded => 0,\n    };\n    let r = match range.end_bound() {\n\
    \        Bound::Included(&r) => r + 1,\n        Bound::Excluded(&r) => r,\n  \
    \      Bound::Unbounded => n,\n    };\n    (l, r)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/cumulative-sum-2d/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-25 15:51:20+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/cumulative-sum-2d/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/cumulative-sum-2d/src/lib.rs
- /library/crates/data-structure/cumulative-sum-2d/src/lib.rs.html
title: crates/data-structure/cumulative-sum-2d/src/lib.rs
---
