---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/binomial_coefficient_prime_mod/src/main.rs
    title: verify/binomial_coefficient_prime_mod/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::cell::RefCell;\n\nuse modint::ModInt;\n\npub struct Combination<M:\
    \ ModInt> {\n    inv: RefCell<Vec<M>>,\n    fact: RefCell<Vec<M>>,\n    fact_inv:\
    \ RefCell<Vec<M>>,\n}\n\nimpl<M: ModInt> Combination<M> {\n    pub fn new() ->\
    \ Self {\n        Self {\n            inv: RefCell::new(vec![M::from(0), M::from(1)]),\n\
    \            fact: RefCell::new(vec![M::from(1); 2]),\n            fact_inv: RefCell::new(vec![M::from(1);\
    \ 2]),\n        }\n    }\n\n    fn expand(&self, n: usize) {\n        let mut\
    \ inv = self.inv.borrow_mut();\n        let mut fact = self.fact.borrow_mut();\n\
    \        let mut fact_inv = self.fact_inv.borrow_mut();\n        let m = inv.len();\n\
    \        let mut nn = m;\n        while nn <= n {\n            nn *= 2;\n    \
    \    }\n        inv.resize(nn, M::default());\n        fact.resize(nn, M::default());\n\
    \        fact_inv.resize(nn, M::default());\n        let p = M::modulus() as usize;\n\
    \        for i in m..nn {\n            inv[i] = -inv[p % i] * M::from((p / i)\
    \ as u32);\n            fact[i] = fact[i - 1] * M::from(i);\n            fact_inv[i]\
    \ = fact_inv[i - 1] * inv[i];\n        }\n    }\n\n    pub fn inv(&self, n: usize)\
    \ -> M {\n        self.expand(n);\n        self.inv.borrow()[n]\n    }\n\n   \
    \ pub fn fact(&self, n: usize) -> M {\n        self.expand(n);\n        self.fact.borrow()[n]\n\
    \    }\n\n    pub fn fact_inv(&self, n: usize) -> M {\n        self.expand(n);\n\
    \        self.fact_inv.borrow()[n]\n    }\n\n    pub fn nck(&self, n: usize, k:\
    \ usize) -> M {\n        if n < k {\n            M::from(0)\n        } else {\n\
    \            self.expand(n);\n            self.fact.borrow()[n] * self.fact_inv.borrow()[k]\
    \ * self.fact_inv.borrow()[n - k]\n        }\n    }\n\n    pub fn npk(&self, n:\
    \ usize, k: usize) -> M {\n        if n < k {\n            M::from(0)\n      \
    \  } else {\n            self.expand(n);\n            self.fact.borrow()[n] *\
    \ self.fact_inv.borrow()[n - k]\n        }\n    }\n\n    pub fn nhk(&self, n:\
    \ usize, k: usize) -> M {\n        if n == 0 && k == 0 {\n            M::from(1)\n\
    \        } else {\n            self.nck(n + k - 1, k)\n        }\n    }\n\n  \
    \  pub fn catalan(&self, n: usize) -> M {\n        self.expand(n * 2);\n     \
    \   self.fact.borrow()[n * 2] * self.fact_inv.borrow()[n + 1] * self.fact_inv.borrow()[n]\n\
    \    }\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/combination/src/lib.rs
  requiredBy: []
  timestamp: '2023-07-15 18:59:53+09:00'
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
