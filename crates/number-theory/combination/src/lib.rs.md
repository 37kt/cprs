---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/bell-number/src/lib.rs
    title: crates/number-theory/bell-number/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/q-binomial/src/lib.rs
    title: crates/number-theory/q-binomial/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/stirling-first-fixed-k/src/lib.rs
    title: crates/number-theory/stirling-first-fixed-k/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/stirling-second-fixed-k/src/lib.rs
    title: crates/number-theory/stirling-second-fixed-k/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/stirling-second/src/lib.rs
    title: crates/number-theory/stirling-second/src/lib.rs
  - icon: ':warning:'
    path: crates/polynomial/lagrange-interpolation/src/lib.rs
    title: crates/polynomial/lagrange-interpolation/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/binomial_coefficient_prime_mod/src/main.rs
    title: verify/binomial_coefficient_prime_mod/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::cell::RefCell;\n\nuse modint::ModInt;\n\n/// ModInt \u3067\u4E8C\
    \u9805\u4FC2\u6570\u306B\u5FC5\u8981\u306A\u968E\u4E57\u7B49\u3092\u8A08\u7B97\
    \u3059\u308B  \n/// Mod \u306F\u7D20\u6570\u3067\u306A\u3051\u308C\u3070\u306A\
    \u3089\u306A\u3044\n/// DynamicModInt \u3067\u4F7F\u7528\u3059\u308B\u5834\u5408\
    \u306F\u3001 Mod \u3092\u5909\u66F4\u3059\u308B\u305F\u3073\u306B new \u3092\u547C\
    \u3076\u5FC5\u8981\u304C\u3042\u308B\npub struct Combination<M: ModInt> {\n  \
    \  inv: RefCell<Vec<M>>,\n    fact: RefCell<Vec<M>>,\n    fact_inv: RefCell<Vec<M>>,\n\
    }\n\nimpl<M: ModInt> Combination<M> {\n    /// \u521D\u671F\u5316\n    pub fn\
    \ new() -> Self {\n        Self {\n            inv: RefCell::new(vec![M::from(0),\
    \ M::from(1)]),\n            fact: RefCell::new(vec![M::from(1); 2]),\n      \
    \      fact_inv: RefCell::new(vec![M::from(1); 2]),\n        }\n    }\n\n    pub\
    \ fn expand(&self, n: usize) {\n        let mut inv = self.inv.borrow_mut();\n\
    \        let mut fact = self.fact.borrow_mut();\n        let mut fact_inv = self.fact_inv.borrow_mut();\n\
    \        let m = inv.len();\n        let mut nn = m;\n        while nn <= n {\n\
    \            nn *= 2;\n        }\n        inv.resize(nn, M::default());\n    \
    \    fact.resize(nn, M::default());\n        fact_inv.resize(nn, M::default());\n\
    \        let p = M::modulus() as usize;\n        for i in m..nn {\n          \
    \  inv[i] = -inv[p % i] * M::from((p / i) as u32);\n            fact[i] = fact[i\
    \ - 1] * M::from(i);\n            fact_inv[i] = fact_inv[i - 1] * inv[i];\n  \
    \      }\n    }\n\n    /// n \u306E\u9006\u5143\n    pub fn inv(&self, n: usize)\
    \ -> M {\n        self.expand(n);\n        self.inv.borrow()[n]\n    }\n\n   \
    \ /// n!\n    pub fn fact(&self, n: usize) -> M {\n        self.expand(n);\n \
    \       self.fact.borrow()[n]\n    }\n\n    /// n! \u306E\u9006\u5143\n    pub\
    \ fn fact_inv(&self, n: usize) -> M {\n        self.expand(n);\n        self.fact_inv.borrow()[n]\n\
    \    }\n\n    /// n \u500B\u304B\u3089 k \u500B\u9078\u3076\u5834\u5408\u306E\u6570\
    \n    pub fn nck(&self, n: usize, k: usize) -> M {\n        if n < k {\n     \
    \       M::from(0)\n        } else {\n            self.expand(n);\n          \
    \  self.fact.borrow()[n] * self.fact_inv.borrow()[k] * self.fact_inv.borrow()[n\
    \ - k]\n        }\n    }\n\n    /// n \u500B\u304B\u3089 k \u500B\u9078\u3093\u3067\
    \u4E26\u3079\u308B\u5834\u5408\u306E\u6570\n    pub fn npk(&self, n: usize, k:\
    \ usize) -> M {\n        if n < k {\n            M::from(0)\n        } else {\n\
    \            self.expand(n);\n            self.fact.borrow()[n] * self.fact_inv.borrow()[n\
    \ - k]\n        }\n    }\n\n    /// \u91CD\u8907\u3092\u8A31\u3057\u3066 n \u500B\
    \u304B\u3089 k \u500B\u9078\u3076\u5834\u5408\u306E\u6570  \n    /// \u307E\u305F\
    \u306F\u3001 [x^k] (1-x)^{-n}\n    pub fn nhk(&self, n: usize, k: usize) -> M\
    \ {\n        if n == 0 && k == 0 {\n            M::from(1)\n        } else {\n\
    \            self.nck(n + k - 1, k)\n        }\n    }\n\n    /// \u30AB\u30BF\u30E9\
    \u30F3\u6570\n    /// n \u500B\u306E +1 \u3068 n \u500B\u306E -1 \u3092\u3001\u7D2F\
    \u7A4D\u548C\u304C\u3059\u3079\u3066\u975E\u8CA0\u3068\u306A\u308B\u3088\u3046\
    \u306B\u4E26\u3079\u308B\u5834\u5408\u306E\u6570\n    pub fn catalan(&self, n:\
    \ usize) -> M {\n        self.expand(n * 2);\n        self.fact.borrow()[n * 2]\
    \ * self.fact_inv.borrow()[n + 1] * self.fact_inv.borrow()[n]\n    }\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/combination/src/lib.rs
  requiredBy:
  - crates/polynomial/lagrange-interpolation/src/lib.rs
  - crates/number-theory/bell-number/src/lib.rs
  - crates/number-theory/q-binomial/src/lib.rs
  - crates/number-theory/stirling-second/src/lib.rs
  - crates/number-theory/stirling-second-fixed-k/src/lib.rs
  - crates/number-theory/stirling-first-fixed-k/src/lib.rs
  timestamp: '2025-01-26 00:19:46+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/binomial_coefficient_prime_mod/src/main.rs
documentation_of: crates/number-theory/combination/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/combination/src/lib.rs
- /library/crates/number-theory/combination/src/lib.rs.html
title: crates/number-theory/combination/src/lib.rs
---
