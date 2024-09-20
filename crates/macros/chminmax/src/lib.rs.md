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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[macro_export]\nmacro_rules! min {\n    ($a:expr $(,)*) => {{\n        $a\n\
    \    }};\n    ($a:expr, $b:expr $(,)*) => {{\n        std::cmp::min($a, $b)\n\
    \    }};\n    ($a:expr, $($rest:expr),+ $(,)*) => {{\n        std::cmp::min($a,\
    \ $crate::min!($($rest),+))\n    }};\n}\n\n#[macro_export]\nmacro_rules! max {\n\
    \    ($a:expr $(,)*) => {{\n        $a\n    }};\n    ($a:expr, $b:expr $(,)*)\
    \ => {{\n        std::cmp::max($a, $b)\n    }};\n    ($a:expr, $($rest:expr),+\
    \ $(,)*) => {{\n        std::cmp::max($a, $crate::max!($($rest),+))\n    }};\n\
    }\n\n#[macro_export]\nmacro_rules! chmin {\n    ($base:expr, $($cmps:expr),+ $(,)*)\
    \ => {{\n        let cmp_min = $crate::min!($($cmps),+);\n        if $base > cmp_min\
    \ {\n            $base = cmp_min;\n            true\n        } else {\n      \
    \      false\n        }\n    }};\n}\n\n#[macro_export]\nmacro_rules! chmax {\n\
    \    ($base:expr, $($cmps:expr),+ $(,)*) => {{\n        let cmp_max = $crate::max!($($cmps),+);\n\
    \        if $base < cmp_max {\n            $base = cmp_max;\n            true\n\
    \        } else {\n            false\n        }\n    }};\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/macros/chminmax/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/macros/chminmax/src/lib.rs
layout: document
redirect_from:
- /library/crates/macros/chminmax/src/lib.rs
- /library/crates/macros/chminmax/src/lib.rs.html
title: crates/macros/chminmax/src/lib.rs
---
