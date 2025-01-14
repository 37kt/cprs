---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/combination/src/lib.rs
    title: crates/number-theory/combination/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/q_binomial_coefficient_prime_mod/src/main.rs
    title: verify/q_binomial_coefficient_prime_mod/src/main.rs
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
  code: "use std::cell::RefCell;\n\nuse combination::Combination;\nuse modint::ModInt;\n\
    \n/// q-\u985E\u4F3C\npub struct QBinomial<M: ModInt> {\n    q: M,\n    num: RefCell<Vec<M>>,\n\
    \    inv: RefCell<Vec<M>>,\n    fact: RefCell<Vec<M>>,\n    fact_inv: RefCell<Vec<M>>,\n\
    \    comb: Combination<M>,\n}\n\nimpl<M: ModInt> QBinomial<M> {\n    pub fn new(q:\
    \ M) -> Self {\n        Self {\n            q,\n            num: RefCell::new(vec![M::from(0),\
    \ M::from(1)]),\n            inv: RefCell::new(vec![M::from(0), M::from(1)]),\n\
    \            fact: RefCell::new(vec![M::from(1); 2]),\n            fact_inv: RefCell::new(vec![M::from(1);\
    \ 2]),\n            comb: Combination::new(),\n        }\n    }\n\n    fn expand(&self,\
    \ n: usize) {\n        let mut num = self.num.borrow_mut();\n        let mut inv\
    \ = self.inv.borrow_mut();\n        let mut fact = self.fact.borrow_mut();\n \
    \       let mut fact_inv = self.fact_inv.borrow_mut();\n        let m = inv.len();\n\
    \        let mut nn = m;\n        while nn <= n {\n            nn *= 2;\n    \
    \    }\n        num.reserve(nn - m);\n        for i in m..nn {\n            let\
    \ x = num[i - 1] * self.q + M::from(1);\n            if x.val() == 0 {\n     \
    \           break;\n            }\n            num.push(x);\n        }\n     \
    \   let nn = num.len();\n        inv.resize(nn, M::default());\n        fact.resize(nn,\
    \ M::default());\n        fact_inv.resize(nn, M::default());\n        for i in\
    \ m..nn {\n            fact[i] = fact[i - 1] * num[i];\n        }\n        fact_inv[nn\
    \ - 1] = fact[nn - 1].inv();\n        inv[nn - 1] = fact_inv[nn - 1] * fact[nn\
    \ - 2];\n        for i in (m..nn - 1).rev() {\n            fact_inv[i] = fact_inv[i\
    \ + 1] * num[i + 1];\n            inv[i] = fact_inv[i] * fact[i - 1];\n      \
    \  }\n    }\n\n    pub fn q(&self) -> M {\n        self.q\n    }\n\n    pub fn\
    \ num(&self, n: usize) -> M {\n        self.expand(n);\n        let num = self.num.borrow();\n\
    \        num[n % num.len()]\n    }\n\n    pub fn inv(&self, n: usize) -> M {\n\
    \        self.expand(n);\n        let inv = self.inv.borrow();\n        inv[n\
    \ % inv.len()]\n    }\n\n    pub fn fact(&self, n: usize) -> M {\n        self.expand(n);\n\
    \        let fact = self.fact.borrow();\n        if n < fact.len() {\n       \
    \     fact[n]\n        } else {\n            0.into()\n        }\n    }\n\n  \
    \  pub fn fact_inv(&self, n: usize) -> M {\n        self.expand(n);\n        let\
    \ fact_inv = self.fact_inv.borrow();\n        if n < fact_inv.len() {\n      \
    \      fact_inv[n]\n        } else {\n            0.into()\n        }\n    }\n\
    \n    pub fn binom(&self, n: usize, k: usize) -> M {\n        if n < k {\n   \
    \         return 0.into();\n        }\n        self.expand(n);\n        let m\
    \ = self.num.borrow().len();\n        if n < m {\n            return self.fact(n)\
    \ * self.fact_inv(k) * self.fact_inv(n - k);\n        }\n        self.comb.nck(n\
    \ / m, k / m) * self.binom(n % m, k % m)\n    }\n}\n"
  dependsOn:
  - crates/number-theory/combination/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/q-binomial/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/q_binomial_coefficient_prime_mod/src/main.rs
documentation_of: crates/number-theory/q-binomial/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/q-binomial/src/lib.rs
- /library/crates/number-theory/q-binomial/src/lib.rs.html
title: crates/number-theory/q-binomial/src/lib.rs
---
