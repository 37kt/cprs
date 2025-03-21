---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
    title: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Mo {\n    type Arg;\n    type Output;\n\n    /// \u533A\u9593\u306E\
    \u7AEF\u306B\u8981\u7D20 i \u3092\u8FFD\u52A0\u3059\u308B\n    #[allow(unused_variables)]\n\
    \    fn add(&mut self, i: usize) {\n        unimplemented!(\"Please implement\
    \ either add or both add_left and add_right.\");\n    }\n\n    /// \u533A\u9593\
    \u306E\u7AEF\u304B\u3089\u8981\u7D20 i \u3092\u524A\u9664\u3059\u308B\n    #[allow(unused_variables)]\n\
    \    fn remove(&mut self, i: usize) {\n        unimplemented!(\"Please implement\
    \ either remove or both remove_left and remove_right.\");\n    }\n\n    /// \u533A\
    \u9593\u306E\u5DE6\u7AEF\u306B\u8981\u7D20 i \u3092\u8FFD\u52A0\u3059\u308B  \n\
    \    /// \u307E\u305F\u306F (i+1, _) \u304B\u3089 (i, _) \u306B\u79FB\u52D5\u3059\
    \u308B\n    fn add_left(&mut self, i: usize) {\n        self.add(i);\n    }\n\n\
    \    /// \u533A\u9593\u306E\u53F3\u7AEF\u306B\u8981\u7D20 i \u3092\u8FFD\u52A0\
    \u3059\u308B  \n    /// \u307E\u305F\u306F (_, i) \u304B\u3089 (_, i+1) \u306B\
    \u79FB\u52D5\u3059\u308B\n    fn add_right(&mut self, i: usize) {\n        self.add(i);\n\
    \    }\n\n    /// \u533A\u9593\u306E\u5DE6\u7AEF\u304B\u3089\u8981\u7D20 i \u3092\
    \u524A\u9664\u3059\u308B  \n    /// \u307E\u305F\u306F (i, _) \u304B\u3089 (i+1,\
    \ _) \u306B\u79FB\u52D5\u3059\u308B\n    fn remove_left(&mut self, i: usize) {\n\
    \        self.remove(i);\n    }\n\n    /// \u533A\u9593\u306E\u53F3\u7AEF\u304B\
    \u3089\u8981\u7D20 i \u3092\u524A\u9664\u3059\u308B  \n    /// \u307E\u305F\u306F\
    \ (_, i+1) \u304B\u3089 (_, i) \u306B\u79FB\u52D5\u3059\u308B\n    fn remove_right(&mut\
    \ self, i: usize) {\n        self.remove(i);\n    }\n\n    fn initial_position(&self)\
    \ -> (usize, usize) {\n        (0, 0)\n    }\n\n    fn query(&mut self, query:\
    \ &Self::Arg) -> Self::Output;\n\n    fn solve(&mut self, queries: &[impl Query<Arg\
    \ = Self::Arg>]) -> Vec<Self::Output> {\n        let Some(n) = queries.iter().map(|q|\
    \ {\n            let (l, r) = q.point();\n            l.max(r)\n        }).max()\
    \ else {\n            return vec![];\n        };\n        let q = queries.len();\n\
    \        let w = 1.max((n as f64 / 1.0f64.max((q as f64 * 2.0 / 3.0).sqrt())).round()\
    \ as usize);\n\n        let mut ord = (0..q).collect::<Vec<_>>();\n        ord.sort_unstable_by_key(|&i|\
    \ {\n            let (l, r) = queries[i].point();\n            (l / w, if (l /\
    \ w) & 1 == 0 { r } else { !r })\n        });\n\n        let (mut l, mut r) =\
    \ self.initial_position();\n        let mut res = ord\n            .iter()\n \
    \           .map(|&i| {\n                let (ql, qr) = queries[i].point();\n\
    \                while l > ql {\n                    l -= 1;\n               \
    \     self.add_left(l);\n                }\n                while r < qr {\n \
    \                   self.add_right(r);\n                    r += 1;\n        \
    \        }\n                while l < ql {\n                    self.remove_left(l);\n\
    \                    l += 1;\n                }\n                while r > qr\
    \ {\n                    r -= 1;\n                    self.remove_right(r);\n\
    \                }\n                self.query(queries[i].argument())\n      \
    \      })\n            .collect::<Vec<_>>();\n\n        for i in 0..q {\n    \
    \        while ord[i] != i {\n                let j = ord[i];\n              \
    \  res.swap(i, j);\n                ord.swap(i, j);\n            }\n        }\n\
    \n        res\n    }\n}\n\npub trait Query {\n    type Arg;\n\n    fn point(&self)\
    \ -> (usize, usize);\n    fn argument(&self) -> &Self::Arg;\n}\n\nimpl Query for\
    \ (usize, usize) {\n    type Arg = ();\n\n    fn point(&self) -> (usize, usize)\
    \ {\n        *self\n    }\n\n    fn argument(&self) -> &Self::Arg {\n        &()\n\
    \    }\n}\n\nimpl<T> Query for (usize, usize, T) {\n    type Arg = T;\n\n    fn\
    \ point(&self) -> (usize, usize) {\n        (self.0, self.1)\n    }\n\n    fn\
    \ argument(&self) -> &Self::Arg {\n        &self.2\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/mo/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-03 03:29:08+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
documentation_of: crates/algorithm/mo/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/mo/src/lib.rs
- /library/crates/algorithm/mo/src/lib.rs.html
title: crates/algorithm/mo/src/lib.rs
---
