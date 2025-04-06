---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/predecessor_problem/src/main.rs
    title: verify/library_checker/data_structure/predecessor_problem/src/main.rs
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
  code: "// TODO: Iterator ?\n\nuse std::ops::Index;\n\nuse numeric_traits::Integer;\n\
    \nconst W: usize = 64;\n\n#[derive(Clone)]\npub struct WordsizeAryTreeSet {\n\
    \    n: usize,\n    v: Vec<Vec<u64>>,\n}\n\nimpl WordsizeAryTreeSet {\n    pub\
    \ fn new(mut len: usize) -> Self {\n        let n = len;\n        let mut v =\
    \ vec![];\n        while {\n            len = len.ceil_div(W);\n            v.push(vec![0;\
    \ len]);\n            len > 1\n        } {}\n        Self { n, v }\n    }\n\n\
    \    /// \u4FDD\u6301\u3067\u304D\u308B\u8981\u7D20\u6570\n    pub fn len(&self)\
    \ -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self) -> bool {\n\
    \        self.v.last().unwrap()[0] == 0\n    }\n\n    pub fn insert(&mut self,\
    \ mut x: usize) -> bool {\n        if self[x] {\n            return false;\n \
    \       }\n        for v in &mut self.v {\n            v[x / W] |= 1 << (x % W);\n\
    \            x /= W;\n        }\n        true\n    }\n\n    pub fn remove(&mut\
    \ self, mut x: usize) -> bool {\n        if !self[x] {\n            return false;\n\
    \        }\n        for v in &mut self.v {\n            v[x / W] &= !(1 << (x\
    \ % W));\n            if v[x / W] != 0 {\n                break;\n           \
    \ }\n            x /= W;\n        }\n        true\n    }\n\n    pub fn contains(&self,\
    \ x: usize) -> bool {\n        self[x]\n    }\n\n    /// x \u4EE5\u4E0A\u3067\u6700\
    \u5C0F\u306E\u8981\u7D20\n    pub fn next(&self, mut x: usize) -> Option<usize>\
    \ {\n        let mut d = 0;\n        loop {\n            let i = x / W;\n    \
    \        let j = x % W;\n            let v = self.v.get(d)?.get(i)? & (!0 << j);\n\
    \            if let Some(to) = v.checked_lsb_index() {\n                x = i\
    \ * W + to;\n                if d == 0 {\n                    break;\n       \
    \         }\n                x *= W;\n                d -= 1;\n            } else\
    \ {\n                d += 1;\n                x = i + 1;\n            }\n    \
    \    }\n        Some(x)\n    }\n\n    /// x \u4EE5\u4E0B\u3067\u6700\u5927\u306E\
    \u8981\u7D20\n    pub fn prev(&self, mut x: usize) -> Option<usize> {\n      \
    \  x = x.min(self.n.checked_sub(1)?);\n        let mut d = 0;\n        loop {\n\
    \            let i = x / W;\n            let j = x % W;\n            let v = self.v.get(d)?.get(i)?\
    \ & !(!1 << j);\n            if v == 0 {\n                d += 1;\n          \
    \      x = i.checked_sub(1)?;\n            } else {\n                let to =\
    \ v.msb_index();\n                x = i * W + to;\n                if d == 0 {\n\
    \                    break;\n                }\n                x = (x + 1) *\
    \ W - 1;\n                d -= 1;\n            }\n        }\n        Some(x)\n\
    \    }\n}\n\nimpl Index<usize> for WordsizeAryTreeSet {\n    type Output = bool;\n\
    \n    fn index(&self, index: usize) -> &Self::Output {\n        assert!(index\
    \ < self.n, \"index out of bounds\");\n        let i = index / W;\n        let\
    \ j = index % W;\n        if self.v[0][i] >> j & 1 == 1 {\n            &true\n\
    \        } else {\n            &false\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/data_structure/wordsize_ary_tree_set/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/predecessor_problem/src/main.rs
documentation_of: crates/data_structure/wordsize_ary_tree_set/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/wordsize_ary_tree_set/src/lib.rs
- /library/crates/data_structure/wordsize_ary_tree_set/src/lib.rs.html
title: crates/data_structure/wordsize_ary_tree_set/src/lib.rs
---
