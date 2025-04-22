---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/number_theory/enumerate_primes_ls/src/main.rs
    title: verify/library_checker/number_theory/enumerate_primes_ls/src/main.rs
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
  code: "use std::iter::FusedIterator;\n\npub struct LinearSieve {\n    n: u32,\n\
    \    ps: Vec<u32>,\n    lpf: Vec<u32>,\n}\n\nimpl LinearSieve {\n    pub fn new(n:\
    \ usize) -> Self {\n        let sz = n / 30 * 8\n            + match n % 30 {\n\
    \                29.. => 8,\n                23.. => 7,\n                19..\
    \ => 6,\n                17.. => 5,\n                13.. => 4,\n            \
    \    11.. => 3,\n                7.. => 2,\n                1.. => 1,\n      \
    \          _ => 0,\n            };\n        let n = n as u32;\n\n        let mut\
    \ ps = vec![2, 3, 5];\n        ps.retain(|&p| p <= n);\n\n        let mut lpf\
    \ = vec![1; sz];\n        for i in 1..sz {\n            let x = Self::x(i);\n\
    \            if lpf[i] == 1 {\n                lpf[i] = x;\n                ps.push(x);\n\
    \            }\n            let lpf_i = lpf[i];\n            for &y in ps.iter().skip(3).take_while(|&&y|\
    \ y <= lpf_i && x * y <= n) {\n                let j = Self::id(x * y).unwrap();\n\
    \                lpf[j] = y;\n            }\n        }\n        Self { n, ps,\
    \ lpf }\n    }\n\n    pub fn is_prime(&self, x: usize) -> bool {\n        let\
    \ x = x as u32;\n        assert!(x <= self.n);\n        match x {\n          \
    \  2 | 3 | 5 => true,\n            _ => Self::id(x).map_or(false, |i| self.lpf[i]\
    \ == x),\n        }\n    }\n\n    pub fn primes(\n        &self,\n    ) -> impl\
    \ Iterator<Item = usize> + FusedIterator + ExactSizeIterator + DoubleEndedIterator\
    \ + '_\n    {\n        self.ps.iter().copied().map(|p| p as _)\n    }\n\n    pub\
    \ fn least_factor(&self, x: usize) -> Option<usize> {\n        let x = x as u32;\n\
    \        assert!(x <= self.n);\n        match x {\n            ..=1 => None,\n\
    \            _ if x % 2 == 0 => Some(2),\n            _ if x % 3 == 0 => Some(3),\n\
    \            _ if x % 5 == 0 => Some(5),\n            _ => Some(self.lpf[Self::id(x).unwrap()]\
    \ as usize),\n        }\n    }\n\n    pub fn factors(&self, mut x: usize) -> impl\
    \ Iterator<Item = usize> + '_ {\n        std::iter::from_fn(move || {\n      \
    \      let p = self.least_factor(x)?;\n            x /= p;\n            Some(p)\n\
    \        })\n    }\n\n    fn id(x: u32) -> Option<usize> {\n        if x <= 6\
    \ {\n            return None;\n        }\n        let offset = x / 30 * 8;\n \
    \       let res = match x % 30 {\n            1 => 0,\n            7 => 1,\n \
    \           11 => 2,\n            13 => 3,\n            17 => 4,\n           \
    \ 19 => 5,\n            23 => 6,\n            29 => 7,\n            _ => return\
    \ None,\n        } + offset;\n        Some(res as usize)\n    }\n\n    fn x(id:\
    \ usize) -> u32 {\n        const CANDS: [u32; 8] = [1, 7, 11, 13, 17, 19, 23,\
    \ 29];\n        id as u32 / 8 * 30 + CANDS[id % 8]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/number_theory/linear_sieve/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-04 07:34:26+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/number_theory/enumerate_primes_ls/src/main.rs
documentation_of: crates/number_theory/linear_sieve/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/linear_sieve/src/lib.rs
- /library/crates/number_theory/linear_sieve/src/lib.rs.html
title: crates/number_theory/linear_sieve/src/lib.rs
---
