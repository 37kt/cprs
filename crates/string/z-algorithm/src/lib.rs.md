---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/zalgorithm/src/main.rs
    title: verify/zalgorithm/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn z_algorithm<T>(s: &[T]) -> Vec<usize>\nwhere\n    T: Eq,\n{\n    let\
    \ n = s.len();\n    if n == 0 {\n        return vec![];\n    }\n    let mut z\
    \ = vec![0; n];\n    let mut j = 0;\n    for i in 1..n {\n        let mut k =\
    \ if j + z[j] <= i {\n            0\n        } else {\n            z[i - j].min(j\
    \ + z[j] - i)\n        };\n        while i + k < n && s[k] == s[i + k] {\n   \
    \         k += 1;\n        }\n        z[i] = k;\n        if j + z[j] < i + z[j]\
    \ {\n            j = i;\n        }\n    }\n    z[0] = n;\n    z\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/z-algorithm/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-27 14:07:11+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/zalgorithm/src/main.rs
documentation_of: crates/string/z-algorithm/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/z-algorithm/src/lib.rs
- /library/crates/string/z-algorithm/src/lib.rs.html
title: crates/string/z-algorithm/src/lib.rs
---
