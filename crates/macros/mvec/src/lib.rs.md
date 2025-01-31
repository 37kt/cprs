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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// mvec![]\n#[macro_export]\nmacro_rules! mvec {\n    ($x:expr; $n:expr)\
    \ => {\n        vec![$x; $n]\n    };\n    ($x:expr; $n:expr $(; $m:expr)+) =>\
    \ {\n        vec![mvec![$x $(; $m)*]; $n]\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/macros/mvec/src/lib.rs
  requiredBy: []
  timestamp: '2024-11-25 10:14:10+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/macros/mvec/src/lib.rs
layout: document
redirect_from:
- /library/crates/macros/mvec/src/lib.rs
- /library/crates/macros/mvec/src/lib.rs.html
title: crates/macros/mvec/src/lib.rs
---
