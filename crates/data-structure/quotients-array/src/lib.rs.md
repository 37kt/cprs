---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/quotients/src/lib.rs
    title: crates/number-theory/quotients/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/math/lucy-dp/src/lib.rs
    title: crates/math/lucy-dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/min_25-sieve/src/lib.rs
    title: crates/math/min_25-sieve/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/sum_of_multiplicative_function/src/main.rs
    title: verify/sum_of_multiplicative_function/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::Debug,\n    ops::{Index, IndexMut},\n};\n\nuse quotients::quotients;\n\
    \n/// \u9577\u3055 n \u306E\u914D\u5217\n/// \u305F\u3060\u3057\u3001 \u230An/i\u230B\
    \ \u306E\u5024\u304C\u7B49\u3057\u3044\u8981\u7D20\u306F\u307E\u3068\u3081\u3066\
    \u7BA1\u7406\u3059\u308B\u3002\n#[derive(Clone)]\npub struct QuotientsArray<T>\
    \ {\n    n: usize,\n    sqrt_n: usize,\n    quotients: Vec<usize>,\n    data:\
    \ Vec<T>,\n}\n\nimpl<T> QuotientsArray<T> {\n    /// \u6DFB\u5B57 i \u306B\u5BFE\
    \u5FDC\u3059\u308B\u5024\u3092 f(i) \u3067\u521D\u671F\u5316\u3059\u308B\u3002\
    \n    /// \u6DFB\u5B57\u304C\u5927\u304D\u3044\u9806\u306B f \u3092\u547C\u3073\
    \u51FA\u3059\u3002\n    pub fn from_fn(n: usize, f: impl FnMut(usize) -> T) ->\
    \ Self {\n        let sqrt_n = (n as f64).sqrt().floor() as usize;\n        let\
    \ data = quotients(n).map(f).collect::<Vec<_>>();\n        let quotients = quotients(n).collect::<Vec<_>>();\n\
    \        Self {\n            n,\n            sqrt_n,\n            quotients,\n\
    \            data,\n        }\n    }\n\n    /// \u8981\u7D20\u6570\u3092\u53D6\
    \u5F97\u3059\u308B\u3002\n    pub fn n(&self) -> usize {\n        self.n\n   \
    \ }\n\n    /// \u230A\u221An\u230B \u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub\
    \ fn sqrt_n(&self) -> usize {\n        self.sqrt_n\n    }\n\n    /// \u230An/i\u230B\
    \ \u306E\u5024\u306E\u96C6\u5408\u3092\u964D\u9806\u3067\u53D6\u5F97\u3059\u308B\
    \u3002\n    pub fn quotients(&self) -> &[usize] {\n        &self.quotients\n \
    \   }\n\n    /// \u230An/i\u230B \u306E\u5024\u306E\u96C6\u5408\u306E\u8981\u7D20\
    \u6570\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn len(&self) -> usize {\n\
    \        self.data.len()\n    }\n\n    /// x \u4EE5\u4E0B\u306E \u230An/i\u230B\
    \ \u306B\u542B\u307E\u308C\u308B\u5024\u306E\u3046\u3061\u6700\u5927\u306E\u3082\
    \u306E\u306B\u5BFE\u5FDC\u3059\u308B\u6DFB\u5B57\u3092\u53D6\u5F97\u3059\u308B\
    \u3002\n    fn index(&self, x: usize) -> usize {\n        assert!(1 <= x && x\
    \ <= self.n);\n        if x <= self.sqrt_n {\n            self.data.len() - x\n\
    \        } else {\n            self.n / x - 1\n        }\n    }\n}\n\nimpl<T>\
    \ Index<usize> for QuotientsArray<T> {\n    type Output = T;\n    fn index(&self,\
    \ x: usize) -> &Self::Output {\n        let i = self.index(x);\n        unsafe\
    \ { self.data.get_unchecked(i) }\n    }\n}\n\nimpl<T> IndexMut<usize> for QuotientsArray<T>\
    \ {\n    fn index_mut(&mut self, x: usize) -> &mut Self::Output {\n        let\
    \ i = self.index(x);\n        unsafe { self.data.get_unchecked_mut(i) }\n    }\n\
    }\n\nimpl<T: Debug> Debug for QuotientsArray<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        f.write_str(\"{\")?;\n        for i in 0..self.data.len()\
    \ {\n            if i > 0 {\n                f.write_str(\", \")?;\n         \
    \   }\n            f.write_fmt(format_args!(\"{}: {:?}\", self.quotients[i], self.data[i]))?;\n\
    \        }\n        f.write_str(\"}\")\n    }\n}\n"
  dependsOn:
  - crates/number-theory/quotients/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/quotients-array/src/lib.rs
  requiredBy:
  - crates/math/min_25-sieve/src/lib.rs
  - crates/math/lucy-dp/src/lib.rs
  timestamp: '2024-12-25 10:01:29+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/sum_of_multiplicative_function/src/main.rs
documentation_of: crates/data-structure/quotients-array/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/quotients-array/src/lib.rs
- /library/crates/data-structure/quotients-array/src/lib.rs.html
title: crates/data-structure/quotients-array/src/lib.rs
---
