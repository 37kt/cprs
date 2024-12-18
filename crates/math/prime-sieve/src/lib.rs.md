---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/math/lucy-dp/src/lib.rs
    title: crates/math/lucy-dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/min_25-sieve/src/lib.rs
    title: crates/math/min_25-sieve/src/lib.rs
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
  code: "pub struct PrimeSieve {\n    primes: Vec<usize>,\n    div: Vec<usize>,\n\
    }\n\nimpl PrimeSieve {\n    pub fn new(n: usize) -> Self {\n        let n = n.max(2);\n\
    \        let mut div = (0..=n).collect::<Vec<_>>();\n        div[1] = 0;\n   \
    \     for i in 2..=n {\n            if div[i] != i {\n                continue;\n\
    \            }\n            for j in (i * 2..=n).step_by(i) {\n              \
    \  if div[j] != j {\n                    continue;\n                }\n      \
    \          div[j] = i;\n            }\n        }\n        let mut primes = vec![];\n\
    \        for i in 2..=n {\n            if div[i] == i {\n                primes.push(i);\n\
    \            }\n        }\n        Self { primes, div }\n    }\n\n    pub fn is_prime(&self,\
    \ x: usize) -> bool {\n        let n = self.primes.len() - 1;\n        assert!(x\
    \ <= n * n);\n        if x <= n {\n            self.div[x] == x\n        } else\
    \ {\n            for &p in &self.primes {\n                if p * p > x {\n  \
    \                  break;\n                }\n                if x % p == 0 {\n\
    \                    return false;\n                }\n            }\n       \
    \     true\n        }\n    }\n\n    pub fn factorize(&self, x: usize) -> Vec<(usize,\
    \ usize)> {\n        let mut res = Vec::<(usize, usize)>::new();\n        let\
    \ n = self.div.len() - 1;\n        assert!(x <= n * n);\n        if x <= n {\n\
    \            let mut y = x;\n            while y > 1 {\n                if res.len()\
    \ == 0 || res.last().unwrap().0 != self.div[y] {\n                    res.push((self.div[y],\
    \ 1));\n                } else {\n                    res.last_mut().unwrap().1\
    \ += 1;\n                }\n                y /= self.div[y];\n            }\n\
    \        } else {\n            let mut y = x;\n            for &p in &self.primes\
    \ {\n                if y % p == 0 {\n                    res.push((p, 0));\n\
    \                    while y % p == 0 {\n                        res.last_mut().unwrap().1\
    \ += 1;\n                        y /= p;\n                    }\n            \
    \    }\n            }\n            if y > 1 {\n                res.push((y, 1));\n\
    \            }\n        }\n        res\n    }\n\n    pub fn divisors(&self, x:\
    \ usize) -> Vec<usize> {\n        let mut res = vec![1];\n        for (p, k) in\
    \ self.factorize(x) {\n            for i in 0..res.len() {\n                let\
    \ mut t = res[i];\n                for _ in 0..k {\n                    t *= p;\n\
    \                    res.push(t);\n                }\n            }\n        }\n\
    \        res.sort();\n        res\n    }\n\n    pub fn primes(&self) -> Vec<usize>\
    \ {\n        self.primes.clone()\n    }\n\n    pub fn min_divisor(&self, x: usize)\
    \ -> usize {\n        self.div[x]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/prime-sieve/src/lib.rs
  requiredBy:
  - crates/math/lucy-dp/src/lib.rs
  - crates/math/min_25-sieve/src/lib.rs
  timestamp: '2023-11-07 11:26:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/prime-sieve/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/prime-sieve/src/lib.rs
- /library/crates/math/prime-sieve/src/lib.rs.html
title: crates/math/prime-sieve/src/lib.rs
---
