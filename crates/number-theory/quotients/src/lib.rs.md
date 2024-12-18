---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/quotients-array/src/lib.rs
    title: crates/data-structure/quotients-array/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/enumerate_quotients/src/main.rs
    title: verify/enumerate_quotients/src/main.rs
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
  code: "/// n \u3092 \\[1..n\\] \u3067\u5272\u3063\u305F\u5546\u304B\u3089\u306A\u308B\
    \u96C6\u5408\u306E\u8981\u7D20\u3092\u964D\u9806\u306B\u5217\u6319\npub fn quotients(n:\
    \ usize) -> impl Iterator<Item = usize> {\n    let mut i = n;\n    std::iter::from_fn(move\
    \ || {\n        if i > 0 {\n            let res = i;\n            i = n / (n /\
    \ i + 1);\n            Some(res)\n        } else {\n            None\n       \
    \ }\n    })\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/number-theory/quotients/src/lib.rs
  requiredBy:
  - crates/data-structure/quotients-array/src/lib.rs
  timestamp: '2024-12-18 03:30:26+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/enumerate_quotients/src/main.rs
documentation_of: crates/number-theory/quotients/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/quotients/src/lib.rs
- /library/crates/number-theory/quotients/src/lib.rs.html
title: crates/number-theory/quotients/src/lib.rs
---
