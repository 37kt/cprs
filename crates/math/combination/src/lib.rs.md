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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::cell::RefCell;\n\nuse ac_library::modint::ModIntBase;\n\npub struct\
    \ Combination<M: ModIntBase> {\n    inv: RefCell<Vec<M>>,\n    fact: RefCell<Vec<M>>,\n\
    \    fact_inv: RefCell<Vec<M>>,\n}\n\nimpl<M: ModIntBase> Combination<M> {\n \
    \   pub fn new() -> Self {\n        Self {\n            inv: RefCell::new(vec![M::new(0),\
    \ M::new(1)]),\n            fact: RefCell::new(vec![M::new(1); 2]),\n        \
    \    fact_inv: RefCell::new(vec![M::new(1); 2]),\n        }\n    }\n\n    fn expand(&self,\
    \ n: usize) {\n        let mut inv = self.inv.borrow_mut();\n        let mut fact\
    \ = self.fact.borrow_mut();\n        let mut fact_inv = self.fact_inv.borrow_mut();\n\
    \        let m = inv.len();\n        let mut nn = m;\n        while nn <= n {\n\
    \            nn *= 2;\n        }\n        inv.resize(nn, M::default());\n    \
    \    fact.resize(nn, M::default());\n        fact_inv.resize(nn, M::default());\n\
    \        let p = M::modulus() as usize;\n        for i in m..nn {\n          \
    \  inv[i] = -inv[p % i] * M::new((p / i) as u32);\n            fact[i] = fact[i\
    \ - 1] * M::new(i);\n            fact_inv[i] = fact_inv[i - 1] * inv[i];\n   \
    \     }\n    }\n\n    pub fn inv(&self, n: usize) -> M {\n        self.expand(n);\n\
    \        self.inv.borrow()[n]\n    }\n\n    pub fn fact(&self, n: usize) -> M\
    \ {\n        self.expand(n);\n        self.fact.borrow()[n]\n    }\n\n    pub\
    \ fn fact_inv(&self, n: usize) -> M {\n        self.expand(n);\n        self.fact_inv.borrow()[n]\n\
    \    }\n\n    pub fn nck(&self, n: usize, k: usize) -> M {\n        if n < k {\n\
    \            M::new(0)\n        } else {\n            self.expand(n);\n      \
    \      self.fact.borrow()[n] * self.fact_inv.borrow()[k] * self.fact_inv.borrow()[n\
    \ - k]\n        }\n    }\n\n    pub fn npk(&self, n: usize, k: usize) -> M {\n\
    \        if n < k {\n            M::new(0)\n        } else {\n            self.expand(n);\n\
    \            self.fact.borrow()[n] * self.fact_inv.borrow()[n - k]\n        }\n\
    \    }\n\n    pub fn nhk(&self, n: usize, k: usize) -> M {\n        if n == 0\
    \ && k == 0 {\n            M::new(1)\n        } else {\n            self.nck(n\
    \ + k - 1, k)\n        }\n    }\n\n    pub fn catalan(&self, n: usize) -> M {\n\
    \        self.expand(n * 2);\n        self.fact.borrow()[n * 2] * self.fact_inv.borrow()[n\
    \ + 1] * self.fact_inv.borrow()[n]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/combination/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-21 11:20:46+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/combination/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/combination/src/lib.rs
- /library/crates/math/combination/src/lib.rs.html
title: crates/math/combination/src/lib.rs
---
