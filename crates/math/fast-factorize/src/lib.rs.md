---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/math/montgomery-modint/src/lib.rs
    title: crates/math/montgomery-modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/factorize/src/main.rs
    title: verify/factorize/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/primality_test/src/main.rs
    title: verify/primality_test/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    convert::{TryFrom, TryInto},\n    fmt::Debug,\n    mem::swap,\n\
    };\n\nuse montgomery_modint::MontgomeryModInt;\n\npub fn is_prime(n: impl TryInto<u64,\
    \ Error = impl Debug>) -> bool {\n    let n: u64 = n.try_into().unwrap();\n  \
    \  if n & 1 == 0 {\n        n == 2\n    } else if n <= 1 {\n        false\n  \
    \  } else if n < 1 << 30 {\n        miller_rabin(n, &[2, 7, 61])\n    } else {\n\
    \        miller_rabin(n, &[2, 325, 9375, 28178, 450775, 9780504, 1795265022])\n\
    \    }\n}\n\npub fn factorize<N, E, F>(n: N) -> Vec<N>\nwhere\n    N: TryInto<u64,\
    \ Error = E> + TryFrom<u64, Error = F> + Ord + Copy,\n    E: Debug,\n    F: Debug,\n\
    {\n    let n = n.try_into().unwrap();\n    let mut f = factorize_(n);\n    f.sort();\n\
    \    f.into_iter().map(|x| x.try_into().unwrap()).collect()\n}\n\npub fn factor_count<N,\
    \ E, F>(n: N) -> Vec<(N, usize)>\nwhere\n    N: TryInto<u64, Error = E> + TryFrom<u64,\
    \ Error = F> + Ord + Copy,\n    E: Debug,\n    F: Debug,\n{\n    let f = factorize(n);\n\
    \    if f.len() == 0 {\n        return vec![];\n    }\n    let mut r = vec![(f[0],\
    \ 0)];\n    for p in f {\n        if r.last().unwrap().0 == p {\n            r.last_mut().unwrap().1\
    \ += 1;\n        } else {\n            r.push((p, 1));\n        }\n    }\n   \
    \ r\n}\n\npub fn divisors<N, E, F>(n: N) -> Vec<N>\nwhere\n    N: TryInto<u64,\
    \ Error = E> + TryFrom<u64, Error = F> + Ord + Copy,\n    E: Debug,\n    F: Debug,\n\
    {\n    let n = n.try_into().unwrap();\n    if n == 0 {\n        return vec![];\n\
    \    }\n    let fc = factor_count(n);\n    let mut r = vec![1];\n    for (p, c)\
    \ in fc {\n        for i in 0..r.len() {\n            let mut x = r[i];\n    \
    \        for _ in 0..c {\n                x *= p;\n                r.push(x);\n\
    \            }\n        }\n    }\n    r.sort();\n    r.into_iter().map(|x| x.try_into().unwrap()).collect()\n\
    }\n\nfn gcd(mut a: u64, mut b: u64) -> u64 {\n    while b != 0 {\n        a %=\
    \ b;\n        swap(&mut a, &mut b);\n    }\n    a\n}\n\nfn miller_rabin(n: u64,\
    \ a: &[u64]) -> bool {\n    MontgomeryModInt::set_modulus(n);\n    let d = (n\
    \ - 1) >> (n - 1).trailing_zeros();\n    let e = MontgomeryModInt::new(1);\n \
    \   let r = MontgomeryModInt::new(n - 1);\n    for &a in a {\n        if n <=\
    \ a {\n            break;\n        }\n        let mut t = d;\n        let mut\
    \ y = MontgomeryModInt::new(a).pow(t);\n        while t != n - 1 && y != e &&\
    \ y != r {\n            y *= y;\n            t *= 2;\n        }\n        if y\
    \ != r && t % 2 == 0 {\n            return false;\n        }\n    }\n    true\n\
    }\n\nfn pollard_rho(n: u64) -> u64 {\n    if n & 1 == 0 {\n        return 2;\n\
    \    } else if is_prime(n) {\n        return n;\n    }\n    let m = 1 << (64 -\
    \ n.leading_zeros()) / 8;\n    let o = MontgomeryModInt::new(1);\n    let mut\
    \ c = o;\n    loop {\n        let f = |x: MontgomeryModInt| x * x + c;\n     \
    \   let mut x = o;\n        let mut y = MontgomeryModInt::new(2);\n        let\
    \ mut ys = o;\n        let mut q = o;\n        let mut r = 1;\n        let mut\
    \ g = 1;\n        while g == 1 {\n            x = y;\n            for _ in 0..r\
    \ {\n                y = f(y);\n            }\n            for k in (0..r).step_by(m)\
    \ {\n                if g != 1 {\n                    break;\n               \
    \ }\n                ys = y;\n                for _ in 0..m.min(r - k) {\n   \
    \                 y = f(y);\n                    q *= x - y;\n               \
    \ }\n                g = gcd(q.val(), n);\n            }\n            r <<= 1;\n\
    \        }\n        if g == n {\n            g = 1;\n            while g == 1\
    \ {\n                ys = f(ys);\n                g = gcd((x - ys).val(), n);\n\
    \            }\n        }\n        if g < n {\n            return if is_prime(g)\
    \ {\n                g\n            } else if is_prime(n / g) {\n            \
    \    n / g\n            } else {\n                pollard_rho(g)\n           \
    \ };\n        }\n        c += o;\n    }\n}\n\nfn factorize_(n: u64) -> Vec<u64>\
    \ {\n    if n <= 1 {\n        return vec![];\n    };\n    let p = pollard_rho(n);\n\
    \    if p == n {\n        return vec![p];\n    }\n    let mut r = factorize_(p);\n\
    \    r.extend(factorize_(n / p));\n    r\n}\n"
  dependsOn:
  - crates/math/montgomery-modint/src/lib.rs
  isVerificationFile: false
  path: crates/math/fast-factorize/src/lib.rs
  requiredBy: []
  timestamp: '2023-06-13 17:07:21+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/factorize/src/main.rs
  - verify/primality_test/src/main.rs
documentation_of: crates/math/fast-factorize/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/fast-factorize/src/lib.rs
- /library/crates/math/fast-factorize/src/lib.rs.html
title: crates/math/fast-factorize/src/lib.rs
---
