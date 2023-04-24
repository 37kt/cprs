---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/z-algorithm/src/main.rs
    title: verify/z-algorithm/src/main.rs
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
  code: "pub fn z_algorithm<T>(a: &[T]) -> Vec<usize>\nwhere\n    T: Ord,\n{\n   \
    \ let n = a.len();\n    if n == 0 {\n        return vec![];\n    }\n    let mut\
    \ z = vec![0; n];\n    let mut j = 0;\n    for i in 1..n {\n        let mut k\
    \ = if j + z[j] <= i {\n            0\n        } else {\n            z[i - j].min(j\
    \ + z[j] - i)\n        };\n        while i + k < n && a[k] == a[i + k] {\n   \
    \         k += 1;\n        }\n        z[i] = k;\n        if j + z[j] < i + z[j]\
    \ {\n            j = i;\n        }\n    }\n    z[0] = n;\n    z\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/sequence/z-algorithm/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-24 11:14:51+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/z-algorithm/src/main.rs
documentation_of: crates/sequence/z-algorithm/src/lib.rs
layout: document
redirect_from:
- /library/crates/sequence/z-algorithm/src/lib.rs
- /library/crates/sequence/z-algorithm/src/lib.rs.html
title: crates/sequence/z-algorithm/src/lib.rs
---
