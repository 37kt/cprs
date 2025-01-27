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
  code: "use std::arch::x86_64::{_pdep_u64, _pext_u64};\n\n/// \u30D3\u30C3\u30C8\u3067\
    \u8868\u73FE\u3055\u308C\u305F\u96C6\u5408 s \u306E\u90E8\u5206\u96C6\u5408\u3092\
    \u6607\u9806\u306B\u5217\u6319\npub fn bit_subsets(s: usize) -> impl Iterator<Item\
    \ = usize> {\n    std::iter::successors(Some(0usize), move |x| {\n        let\
    \ y = x.wrapping_sub(s) & s;\n        if y == 0 {\n            None\n        }\
    \ else {\n            Some(y)\n        }\n    })\n}\n\n/// \u30D3\u30C3\u30C8\u3067\
    \u8868\u73FE\u3055\u308C\u305F\u96C6\u5408 s \u306E\u90E8\u5206\u96C6\u5408\u306E\
    \u3046\u3061 popcount \u304C k \u3067\u3042\u308B\u3082\u306E\u3092\u6607\u9806\
    \u306B\u5217\u6319\npub fn bit_combinations(s: usize, k: usize) -> impl Iterator<Item\
    \ = usize> {\n    let n = popcount(s);\n    std::iter::successors((k <= n).then_some((1\
    \ << k) - 1), move |&t| {\n        if t == 0 {\n            return None;\n   \
    \     }\n        let x = lsb(t);\n        let y = t + x;\n        let t = (((t\
    \ & !y) / x) >> 1) | y;\n        if t >= 1 << n {\n            None\n        }\
    \ else {\n            Some(t)\n        }\n    })\n    .map(move |t| pdep(t, s))\n\
    }\n\n/// \u30D3\u30C3\u30C8\u306E\u7ACB\u3063\u3066\u3044\u308B\u6570\u3092\u6570\
    \u3048\u308B\npub fn popcount(x: usize) -> usize {\n    x.count_ones() as usize\n\
    }\n\n/// \u7ACB\u3063\u3066\u3044\u308B\u30D3\u30C3\u30C8\u306E\u3046\u3061\u6700\
    \u4E0A\u4F4D\u306E\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\u3092\u6C42\u3081\u308B\
    \npub fn msb_index(x: usize) -> usize {\n    63 - x.leading_zeros() as usize\n\
    }\n\n/// \u7ACB\u3063\u3066\u3044\u308B\u30D3\u30C3\u30C8\u306E\u3046\u3061\u6700\
    \u4E0B\u4F4D\u306E\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\u3092\u6C42\u3081\u308B\
    \npub fn lsb_index(x: usize) -> usize {\n    x.trailing_zeros() as usize\n}\n\n\
    /// \u7ACB\u3063\u3066\u3044\u308B\u30D3\u30C3\u30C8\u306E\u3046\u3061\u6700\u4E0A\
    \u4F4D\u306E\u30D3\u30C3\u30C8\u3092\u6C42\u3081\u308B\npub fn msb(x: usize) ->\
    \ usize {\n    1 << msb_index(x)\n}\n\n/// \u7ACB\u3063\u3066\u3044\u308B\u30D3\
    \u30C3\u30C8\u306E\u3046\u3061\u6700\u4E0B\u4F4D\u306E\u30D3\u30C3\u30C8\u3092\
    \u6C42\u3081\u308B\npub fn lsb(x: usize) -> usize {\n    x & x.wrapping_neg()\n\
    }\n\n/// mask \u306E\u30D3\u30C3\u30C8\u304C\u7ACB\u3063\u3066\u3044\u308B\u4F4D\
    \u7F6E\u306B\u3042\u308B x \u306E\u30D3\u30C3\u30C8\u3092\u53D6\u308A\u51FA\u3057\
    \u3066\u4E0B\u4F4D popcount(mask) \u30D3\u30C3\u30C8\u306B\u96C6\u3081\u308B\n\
    pub fn pext(x: usize, mask: usize) -> usize {\n    unsafe { _pext_u64(x as u64,\
    \ mask as u64) as usize }\n}\n\n/// mask \u306E\u30D3\u30C3\u30C8\u304C\u7ACB\u3063\
    \u3066\u3044\u308B\u4F4D\u7F6E\u306B x \u306E\u4E0B\u4F4D popcount(mask) \u30D3\
    \u30C3\u30C8\u3092\u914D\u7F6E\u3059\u308B\npub fn pdep(x: usize, mask: usize)\
    \ -> usize {\n    unsafe { _pdep_u64(x as u64, mask as u64) as usize }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/bitutil/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-27 02:45:37+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/bitutil/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/bitutil/src/lib.rs
- /library/crates/misc/bitutil/src/lib.rs.html
title: crates/misc/bitutil/src/lib.rs
---
