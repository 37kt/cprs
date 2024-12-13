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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[macro_export]\nmacro_rules! yes {\n    () => {{\n        println!(\"Yes\"\
    );\n    }};\n    ($a:expr) => {{\n        if $a {\n            println!(\"Yes\"\
    );\n        } else {\n            println!(\"No\");\n        }\n    }};\n}\n\n\
    #[macro_export]\nmacro_rules! takahashi {\n    () => {{\n        println!(\"Takahashi\"\
    );\n    }};\n    ($a:expr) => {{\n        if $a {\n            println!(\"Takahashi\"\
    );\n        } else {\n            println!(\"Aoki\");\n        }\n    }};\n}\n\
    \n#[macro_export]\nmacro_rules! alice {\n    () => {{\n        println!(\"Alice\"\
    );\n    }};\n    ($a:expr) => {{\n        if $a {\n            println!(\"Alice\"\
    );\n        } else {\n            println!(\"Bob\");\n        }\n    }};\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/macros/yes/src/lib.rs
  requiredBy: []
  timestamp: '2024-11-29 08:41:41+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/macros/yes/src/lib.rs
layout: document
redirect_from:
- /library/crates/macros/yes/src/lib.rs
- /library/crates/macros/yes/src/lib.rs.html
title: crates/macros/yes/src/lib.rs
---
