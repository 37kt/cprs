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
  code: "use algebraic::Monoid;\n\npub struct DisjointSparseTable<M>\nwhere\n    M:\
    \ Monoid,\n    M::S: Clone,\n{\n    t: Vec<Vec<M::S>>,\n}\n\nimpl<M> DisjointSparseTable<M>\n\
    where\n    M: Monoid,\n    M::S: Clone,\n{\n    pub fn new(a: &[M::S]) -> Self\
    \ {\n        let n = a.len() + 2;\n        let h = 64 - (n - 1).leading_zeros()\
    \ as usize;\n        let mut t = vec![vec![M::e(); n]; h];\n        for k in 1..h\
    \ {\n            let w = 1 << k;\n            for i in (w..n).step_by(w * 2) {\n\
    \                for j in (i + 1 - w..i).rev() {\n                    t[k][j -\
    \ 1] = M::op(&a[j - 1], &t[k][j]);\n                }\n                let m =\
    \ (i + w - 1).min(n - 1);\n                for j in i..m {\n                 \
    \   t[k][j + 1] = M::op(&t[k][j], &a[j - 1]);\n                }\n           \
    \ }\n        }\n        Self { t }\n    }\n\n    pub fn size(&self) -> usize {\n\
    \        self.t[0].len() - 2\n    }\n\n    pub fn get(&self, k: usize) -> M::S\
    \ {\n        assert!(k < self.size());\n        self.prod(k, k + 1)\n    }\n\n\
    \    pub fn prod(&self, l: usize, r: usize) -> M::S {\n        assert!(l <= r\
    \ && r <= self.size());\n        let r = r + 1;\n        let s = &self.t[63 -\
    \ (l ^ r).leading_zeros() as usize];\n        M::op(&s[l], &s[r])\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/disjoint-sparse-table/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/disjoint-sparse-table/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/disjoint-sparse-table/src/lib.rs
- /library/crates/data-structure/disjoint-sparse-table/src/lib.rs.html
title: crates/data-structure/disjoint-sparse-table/src/lib.rs
---
