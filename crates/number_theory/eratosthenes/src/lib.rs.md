---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/gcd_convolution/src/lib.rs
    title: crates/convolution/gcd_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/lcm_convolution/src/lib.rs
    title: crates/convolution/lcm_convolution/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
    title: verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
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
  code: "use numeric_traits::Integer;\n\npub struct Eratosthenes {\n    n: usize,\n\
    \    fs: Vec<u8>,\n}\n\nimpl Eratosthenes {\n    pub fn new(n: usize) -> Self\
    \ {\n        if n == 0 {\n            return Self { n: 0, fs: vec![] };\n    \
    \    }\n\n        let sz = n.ceil_div(30);\n        let mut fs = vec![0xff_u8;\
    \ sz];\n        match n % 30 {\n            29.. => fs[sz - 1] = 0xff,\n     \
    \       23.. => fs[sz - 1] = 0x7f,\n            19.. => fs[sz - 1] = 0x3f,\n \
    \           17.. => fs[sz - 1] = 0x1f,\n            13.. => fs[sz - 1] = 0x0f,\n\
    \            11.. => fs[sz - 1] = 0x07,\n            7.. => fs[sz - 1] = 0x03,\n\
    \            1.. => fs[sz - 1] = 0x01,\n            _ => {}\n        }\n     \
    \   fs[0] &= !1;\n\n        'a: for i0 in 0..sz {\n            let mut f = fs[i0];\n\
    \            while f != 0 {\n                let i1 = f.lsb_index();\n       \
    \         let x1 = MOD_30[i1];\n                if (i0 * 30 + x1).pow(2) > n {\n\
    \                    break 'a;\n                }\n                let mut j0\
    \ = i0 * (30 * i0 + 2 * x1) + (x1 * x1) / 30;\n                let mut j1 = i1;\n\
    \                while j0 < sz {\n                    fs[j0] &= MUL_MASK[i1][j1];\n\
    \                    j0 += i0 * D1[j1] + D2[i1][j1];\n                    j1 =\
    \ (j1 + 1) % 8;\n                }\n                f &= f - 1;\n            }\n\
    \        }\n\n        Self { n, fs }\n    }\n\n    pub fn primes(&self) -> impl\
    \ Iterator<Item = usize> + '_ {\n        [2, 3, 5]\n            .into_iter()\n\
    \            .filter(|&p| p <= self.n)\n            .chain(self.fs.iter().enumerate().flat_map(|(i,\
    \ &f)| {\n                let mut f = f;\n                std::iter::from_fn(move\
    \ || {\n                    (f != 0).then(|| {\n                        let j\
    \ = f.lsb_index();\n                        f &= f - 1;\n                    \
    \    i * 30 + MOD_30[j]\n                    })\n                })\n        \
    \    }))\n    }\n\n    pub fn is_prime(&self, x: usize) -> bool {\n        assert!(x\
    \ <= self.n);\n        match x {\n            2 | 3 | 5 => true,\n           \
    \ _ => Self::id(x).map_or(false, |i| self.fs[i / 30] >> (i % 30) & 1 != 0),\n\
    \        }\n    }\n\n    fn id(x: usize) -> Option<usize> {\n        if x <= 6\
    \ {\n            return None;\n        }\n        let offset = x / 30 * 8;\n \
    \       let res = match x % 30 {\n            1 => 0,\n            7 => 1,\n \
    \           11 => 2,\n            13 => 3,\n            17 => 4,\n           \
    \ 19 => 5,\n            23 => 6,\n            29 => 7,\n            _ => return\
    \ None,\n        } + offset;\n        Some(res)\n    }\n}\n\nconst MOD_30: [usize;\
    \ 8] = [1, 7, 11, 13, 17, 19, 23, 29];\nconst D1: [usize; 8] = [6, 4, 2, 4, 2,\
    \ 4, 6, 2];\nconst D2: [[usize; 8]; 8] = init_d2();\nconst MUL_MASK: [[u8; 8];\
    \ 8] = init_mul_mask();\n\nconst fn init_mul_mask() -> [[u8; 8]; 8] {\n    let\
    \ mut mul = [[0; 8]; 8];\n    let mut i = 0;\n    while i < 8 {\n        let mut\
    \ j = 0;\n        while j < 8 {\n            let x = MOD_30[i] * MOD_30[j] % 30;\n\
    \            let k = match x {\n                1 => 0,\n                7 =>\
    \ 1,\n                11 => 2,\n                13 => 3,\n                17 =>\
    \ 4,\n                19 => 5,\n                23 => 6,\n                29 =>\
    \ 7,\n                _ => unreachable!(),\n            };\n            mul[i][j]\
    \ = !(1 << k);\n            j += 1;\n        }\n        i += 1;\n    }\n    mul\n\
    }\n\nconst fn init_d2() -> [[usize; 8]; 8] {\n    let mut d2 = [[0; 8]; 8];\n\
    \    let mut i = 0;\n    while i < 8 {\n        let mut j = 0;\n        while\
    \ j < 8 {\n            let x = MOD_30[i] * (MOD_30[j] + D1[j]) / 30;\n       \
    \     let y = MOD_30[i] * MOD_30[j] / 30;\n            d2[i][j] = x - y;\n   \
    \         j += 1;\n        }\n        i += 1;\n    }\n    d2\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/number_theory/eratosthenes/src/lib.rs
  requiredBy:
  - crates/convolution/lcm_convolution/src/lib.rs
  - crates/convolution/gcd_convolution/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
documentation_of: crates/number_theory/eratosthenes/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/eratosthenes/src/lib.rs
- /library/crates/number_theory/eratosthenes/src/lib.rs.html
title: crates/number_theory/eratosthenes/src/lib.rs
---
