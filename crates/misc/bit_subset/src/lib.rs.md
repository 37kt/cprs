---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
    title: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
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
  code: "pub trait BitSubsetExt {\n    fn subsets(self) -> BitSubsetIterator;\n  \
    \  fn combinations(self, k: usize) -> BitCombinationIterator;\n}\n\nimpl BitSubsetExt\
    \ for usize {\n    /// \u30D3\u30C3\u30C8\u5217\u3067\u8868\u73FE\u3055\u308C\u305F\
    \u96C6\u5408 s \u306E\u90E8\u5206\u96C6\u5408\u3092\u6607\u9806\u306B\u5217\u6319\
    \u3059\u308B\u30A4\u30C6\u30EC\u30FC\u30BF\n    fn subsets(self) -> BitSubsetIterator\
    \ {\n        BitSubsetIterator {\n            s: self,\n            x: Some(0),\n\
    \        }\n    }\n\n    /// \u30D3\u30C3\u30C8\u5217\u3067\u8868\u73FE\u3055\u308C\
    \u305F\u96C6\u5408 s \u306E\u90E8\u5206\u96C6\u5408\u306E\u5185 popcount \u304C\
    \ k \u3067\u3042\u308B\u3082\u306E\u3092\u6607\u9806\u306B\u5217\u6319\u3059\u308B\
    \u30A4\u30C6\u30EC\u30FC\u30BF\n    fn combinations(self, k: usize) -> BitCombinationIterator\
    \ {\n        BitCombinationIterator {\n            s: self,\n            x: (k\
    \ <= self.count_ones() as usize).then_some((1 << k) - 1),\n        }\n    }\n\
    }\n\n#[derive(Clone, Copy)]\npub struct BitSubsetIterator {\n    s: usize,\n \
    \   x: Option<usize>,\n}\n\nimpl Iterator for BitSubsetIterator {\n    type Item\
    \ = usize;\n\n    fn next(&mut self) -> Option<Self::Item> {\n        let res\
    \ = self.x;\n        if let Some(x) = res {\n            let y = x.wrapping_sub(self.s)\
    \ & self.s;\n            self.x = (y > 0).then_some(y);\n        }\n        res\n\
    \    }\n}\n\n#[derive(Clone, Copy)]\npub struct BitCombinationIterator {\n   \
    \ s: usize,\n    x: Option<usize>,\n}\n\nimpl Iterator for BitCombinationIterator\
    \ {\n    type Item = usize;\n\n    fn next(&mut self) -> Option<Self::Item> {\n\
    \        let res = self.x;\n        if let Some(t) = res {\n            if t ==\
    \ 0 {\n                self.x = None;\n            } else {\n                let\
    \ x = t & t.wrapping_neg();\n                let y = t + x;\n                let\
    \ t = (((t & !y) / x) >> 1) | y;\n                self.x = (t < 1 << self.s.count_ones()\
    \ as usize).then_some(t);\n            }\n        }\n        res.map(|t| unsafe\
    \ { std::arch::x86_64::_pdep_u64(t as u64, self.s as u64) as usize })\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/bit_subset/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-23 08:46:19+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
documentation_of: crates/misc/bit_subset/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/bit_subset/src/lib.rs
- /library/crates/misc/bit_subset/src/lib.rs.html
title: crates/misc/bit_subset/src/lib.rs
---
