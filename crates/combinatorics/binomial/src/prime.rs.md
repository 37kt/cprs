---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  _extendedRequiredBy:
  - icon: ':x:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
    title: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint::ModInt;\nuse numeric_traits::Integer;\nuse prime_factorization::is_prime;\n\
    \npub struct BinomialPrime<M: ModInt<Value = u32>> {\n    fact: Vec<M>,\n    fact_inv:\
    \ Vec<M>,\n    inv: Vec<M>,\n}\n\nimpl<M: ModInt<Value = u32>> Default for BinomialPrime<M>\
    \ {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> BinomialPrime<M> {\n    pub fn new() -> Self {\n        assert!(is_prime(M::modulus()));\n\
    \n        Self {\n            fact: vec![M::from_raw(1); 2],\n            fact_inv:\
    \ vec![M::from_raw(1); 2],\n            inv: vec![M::from_raw(1); 2],\n      \
    \  }\n    }\n\n    pub fn expand(&mut self, n: usize) {\n        let prev_n =\
    \ self.fact.len() - 1;\n        let new_n = n.ceil_pow2();\n\n        self.fact.resize(new_n\
    \ + 1, M::from_raw(0));\n        self.fact_inv.resize(new_n + 1, M::from_raw(0));\n\
    \        self.inv.resize(new_n + 1, M::from_raw(0));\n\n        for i in prev_n\
    \ + 1..=new_n {\n            self.fact[i] = self.fact[i - 1] * M::from_raw(i as\
    \ _);\n        }\n        self.fact_inv[new_n] = self.fact[new_n].recip();\n \
    \       self.inv[new_n] = self.fact_inv[new_n] * self.fact[new_n - 1];\n     \
    \   for i in (prev_n + 1..new_n).rev() {\n            self.fact_inv[i] = self.fact_inv[i\
    \ + 1] * M::from_raw((i + 1) as _);\n            self.inv[i] = self.fact_inv[i]\
    \ * self.fact[i - 1];\n        }\n    }\n\n    pub fn fact(&mut self, n: usize)\
    \ -> M {\n        if n >= self.fact.len() {\n            self.expand(n);\n   \
    \     }\n        self.fact[n]\n    }\n\n    pub fn fact_inv(&mut self, n: usize)\
    \ -> M {\n        if n >= self.fact_inv.len() {\n            self.expand(n);\n\
    \        }\n        self.fact_inv[n]\n    }\n\n    pub fn inv(&mut self, n: usize)\
    \ -> M {\n        assert!(n != 0);\n        if n >= self.inv.len() {\n       \
    \     self.expand(n);\n        }\n        self.inv[n]\n    }\n\n    pub fn nck(&mut\
    \ self, n: usize, k: usize) -> M {\n        if n < k {\n            return M::from_raw(0);\n\
    \        }\n        if n >= self.fact.len() {\n            self.expand(n);\n \
    \       }\n        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]\n  \
    \  }\n}\n"
  dependsOn:
  - crates/combinatorics/binomial/src/lib.rs
  isVerificationFile: false
  path: crates/combinatorics/binomial/src/prime.rs
  requiredBy:
  - crates/combinatorics/binomial/src/lib.rs
  timestamp: '2025-05-12 06:37:24+00:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
documentation_of: crates/combinatorics/binomial/src/prime.rs
layout: document
redirect_from:
- /library/crates/combinatorics/binomial/src/prime.rs
- /library/crates/combinatorics/binomial/src/prime.rs.html
title: crates/combinatorics/binomial/src/prime.rs
---
