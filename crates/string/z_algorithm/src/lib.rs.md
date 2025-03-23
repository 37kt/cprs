---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/string/zalgorithm/src/main.rs
    title: verify/library_checker/string/zalgorithm/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// `z[i]` = `s[i..]` \u3068 `s[..]` \u306E\u6700\u9577\u5171\u901A\u63A5\
    \u982D\u8F9E\u306E\u9577\u3055\npub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize>\
    \ {\n    let n = s.len();\n    if n == 0 {\n        return vec![];\n    }\n  \
    \  let mut z = vec![0; n];\n    let mut j = 0;\n    for i in 1..n {\n        let\
    \ mut k = (j + z[j]).saturating_sub(i).min(z[i - j]);\n        while i + k < n\
    \ && s[k] == s[i + k] {\n            k += 1;\n        }\n        z[i] = k;\n \
    \       if j + z[j] < i + z[i] {\n            j = i;\n        }\n    }\n    z[0]\
    \ = n;\n    z\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/z_algorithm/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-23 01:06:29+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/string/zalgorithm/src/main.rs
documentation_of: crates/string/z_algorithm/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/z_algorithm/src/lib.rs
- /library/crates/string/z_algorithm/src/lib.rs.html
title: crates/string/z_algorithm/src/lib.rs
---
