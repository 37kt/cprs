---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/flow/min_cost_b_flow/src/lib.rs
    title: crates/flow/min_cost_b_flow/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "const W: usize = 64;\n\n#[derive(Clone)]\npub struct RadixHeap {\n    len:\
    \ usize,\n    last: u64,\n    buckets: [Vec<(u64, usize)>; W + 1],\n    pos: Vec<(usize,\
    \ usize)>, // i -> (bucket_id, pos)\n}\n\nimpl RadixHeap {\n    /// index \u306E\
    \u7BC4\u56F2\u306F [0, n)\n    pub fn new(n: usize) -> Self {\n        Self {\n\
    \            len: 0,\n            last: 0,\n            buckets: std::array::from_fn(|_|\
    \ vec![]),\n            pos: vec![(!0, !0); n],\n        }\n    }\n\n    /// index\
    \ \u306E\u7BC4\u56F2\u306F [0, n)\n    pub fn clear(&mut self, n: usize) {\n \
    \       self.len = 0;\n        self.last = 0;\n        self.buckets.iter_mut().for_each(|b|\
    \ b.clear());\n        self.pos.fill((!0, !0));\n        self.pos.resize(n, (!0,\
    \ !0));\n    }\n\n    pub fn len(&self) -> usize {\n        self.len\n    }\n\n\
    \    pub fn is_empty(&self) -> bool {\n        self.len == 0\n    }\n\n    ///\
    \ \u540C\u3058 index \u3067\u5927\u304D\u3044\u3082\u306E\u304C\u3042\u3063\u305F\
    \u3089\u4E0A\u66F8\u304D\u3059\u308B\n    pub fn push(&mut self, key: u64, index:\
    \ usize) {\n        assert!(self.last <= key);\n        if self.pos[index] ==\
    \ (!0, !0) {\n            self.len += 1;\n            let bi = id(key ^ self.last);\n\
    \            self.pos[index] = (bi, self.buckets[bi].len());\n            self.buckets[bi].push((key,\
    \ index));\n        } else {\n            let (bi, bj) = self.pos[index];\n  \
    \          let (old_key, _) = self.buckets[bi][bj];\n            if old_key <=\
    \ key {\n                return;\n            }\n            let new_bi = id(key\
    \ ^ self.last);\n            if new_bi == bi {\n                self.buckets[bi][bj].0\
    \ = key;\n            } else {\n                self.buckets[bi].swap_remove(bj);\n\
    \                if let Some(&(_, i)) = self.buckets[bi].get(bj) {\n         \
    \           self.pos[i].1 = bj;\n                }\n\n                self.pos[index]\
    \ = (new_bi, self.buckets[new_bi].len());\n                self.buckets[new_bi].push((key,\
    \ index));\n            }\n        }\n    }\n\n    /// (key, index)\n    pub fn\
    \ pop(&mut self) -> Option<(u64, usize)> {\n        if self.len == 0 {\n     \
    \       return None;\n        }\n\n        let bi = self.buckets.iter().position(|b|\
    \ !b.is_empty()).unwrap();\n        if bi != 0 {\n            self.last = *self.buckets[bi].iter().map(|(key,\
    \ _)| key).min().unwrap();\n            for (key, i) in std::mem::take(&mut self.buckets[bi])\
    \ {\n                let bj = id(key ^ self.last);\n                self.pos[i]\
    \ = (bj, self.buckets[bj].len());\n                self.buckets[bj].push((key,\
    \ i));\n            }\n        }\n        let (key, i) = self.buckets[0].pop().unwrap();\n\
    \        self.len -= 1;\n        self.pos[i] = (!0, !0);\n        Some((key, i))\n\
    \    }\n}\n\nfn id(x: u64) -> usize {\n    W - x.leading_zeros() as usize\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/radix_heap/src/lib.rs
  requiredBy:
  - crates/flow/min_cost_b_flow/src/lib.rs
  timestamp: '2025-03-15 09:42:03+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/radix_heap/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/radix_heap/src/lib.rs
- /library/crates/data_structure/radix_heap/src/lib.rs.html
title: crates/data_structure/radix_heap/src/lib.rs
---
