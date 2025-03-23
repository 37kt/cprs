---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/string/enumerate_palindromes/src/main.rs
    title: verify/library_checker/string/enumerate_palindromes/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://ngtkana.hatenablog.com/entry/2024/03/17/034026
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://ngtkana.hatenablog.com/entry/2024/03/17/034026\n\n\
    /// `s` \u306B\u542B\u307E\u308C\u308B\u56DE\u6587\u306E\u4E2D\u5FC3\u306F\u3001\
    \u5148\u982D\u3001\u6587\u5B57\u3001\u6587\u5B57\u306E\u9593\u3001\u672B\u5C3E\
    \u306E 2N + 1 \u500B\u3042\u308B\u3002  \n/// \u305D\u308C\u305E\u308C\u3092\u4E2D\
    \u5FC3\u3068\u3059\u308B\u6700\u9577\u306E\u56DE\u6587\u306E\u9577\u3055\u3092\
    \u8FD4\u3059\u3002\npub fn manacher<T: PartialEq>(s: &[T]) -> Vec<usize> {\n \
    \   let n = s.len();\n    let mut res = vec![0; 2 * n + 1];\n    let mut i = 1;\n\
    \    let mut j = 1;\n    while i <= 2 * n {\n        while j < i && i + j < 2\
    \ * n && s[(i - j) / 2 - 1] == s[(i + j) / 2] {\n            j += 2;\n       \
    \ }\n        res[i] = j;\n        if j == 0 {\n            i += 1;\n         \
    \   j = 1;\n            continue;\n        }\n        let mut k = 1;\n       \
    \ while k <= i && k + res[i - k] < j {\n            res[i + k] = res[i - k];\n\
    \            k += 1;\n        }\n        i += k;\n        j -= k;\n    }\n   \
    \ res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/manacher/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-23 01:06:29+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/string/enumerate_palindromes/src/main.rs
documentation_of: crates/string/manacher/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/manacher/src/lib.rs
- /library/crates/string/manacher/src/lib.rs.html
title: crates/string/manacher/src/lib.rs
---
