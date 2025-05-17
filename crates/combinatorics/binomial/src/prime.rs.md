---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
    title: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint::ModInt;\nuse prime_factorization::is_prime;\n\npub struct BinomialPrime<M:\
    \ ModInt<Value = u32>> {\n    fact: Vec<M>,\n    fact_inv: Vec<M>,\n    inv: Vec<M>,\n\
    }\n\nimpl<M: ModInt<Value = u32>> Default for BinomialPrime<M> {\n    fn default()\
    \ -> Self {\n        Self::new()\n    }\n}\n\nimpl<M: ModInt<Value = u32>> BinomialPrime<M>\
    \ {\n    pub fn new() -> Self {\n        assert!(is_prime(M::modulus()));\n\n\
    \        Self {\n            fact: vec![M::from_raw(1); 2],\n            fact_inv:\
    \ vec![M::from_raw(1); 2],\n            inv: vec![M::from_raw(1); 2],\n      \
    \  }\n    }\n\n    pub fn expand(&mut self, n: usize) {\n        let prev_n =\
    \ self.fact.len() - 1;\n        let new_n = n.max(prev_n * 2).min(M::modulus()\
    \ as usize - 1);\n        if prev_n > new_n {\n            return;\n        }\n\
    \n        let p = M::modulus() as usize;\n        self.fact.reserve(new_n + 1);\n\
    \        self.fact_inv.reserve(new_n + 1);\n        self.inv.reserve(new_n + 1);\n\
    \        for i in prev_n + 1..=new_n {\n            self.fact.push(self.fact[i\
    \ - 1] * M::from_raw(i as _));\n            self.inv\n                .push(self.inv[p\
    \ % i] * M::from_raw((p - p / i) as _));\n            self.fact_inv.push(self.fact_inv[i\
    \ - 1] * self.inv[i]);\n        }\n    }\n\n    #[inline]\n    pub fn fact(&mut\
    \ self, n: usize) -> M {\n        if n >= self.fact.len() {\n            self.expand(n);\n\
    \        }\n        self.fact[n]\n    }\n\n    #[inline]\n    pub fn fact_inv(&mut\
    \ self, n: usize) -> M {\n        if n >= self.fact_inv.len() {\n            self.expand(n);\n\
    \        }\n        self.fact_inv[n]\n    }\n\n    #[inline]\n    pub fn inv(&mut\
    \ self, n: usize) -> M {\n        assert!(n != 0);\n        if n >= self.inv.len()\
    \ {\n            self.expand(n);\n        }\n        self.inv[n]\n    }\n\n  \
    \  #[inline]\n    pub fn nck(&mut self, n: usize, k: usize) -> M {\n        if\
    \ n < k {\n            return M::from_raw(0);\n        }\n        if n >= self.fact.len()\
    \ {\n            self.expand(n);\n        }\n        self.fact[n] * self.fact_inv[k]\
    \ * self.fact_inv[n - k]\n    }\n}\n"
  dependsOn:
  - crates/combinatorics/binomial/src/lib.rs
  isVerificationFile: false
  path: crates/combinatorics/binomial/src/prime.rs
  requiredBy:
  - crates/combinatorics/binomial/src/lib.rs
  timestamp: '2025-05-16 07:00:05+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
documentation_of: crates/combinatorics/binomial/src/prime.rs
layout: document
redirect_from:
- /library/crates/combinatorics/binomial/src/prime.rs
- /library/crates/combinatorics/binomial/src/prime.rs.html
title: crates/combinatorics/binomial/src/prime.rs
---
