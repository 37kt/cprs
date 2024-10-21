---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/enumerate_palindromes/src/main.rs
    title: verify/enumerate_palindromes/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://ngtkana.hatenablog.com/entry/2024/03/17/034026
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://ngtkana.hatenablog.com/entry/2024/03/17/034026\n///\
    \ \u5148\u982D\u3001\u6587\u5B57\u306E\u9593\u3001\u672B\u5C3E\u306B\u30C0\u30DF\
    \u30FC\u6587\u5B57\u3092\u5165\u308C\u305F\u3068\u304D\u306E\u56DE\u6587\u534A\
    \u5F84\u3092\u6C42\u3081\u308B\npub fn manacher<T: Eq>(s: &[T]) -> Vec<usize>\
    \ {\n    let n = s.len();\n    let mut a = vec![0; 2 * n + 1];\n    let mut i\
    \ = 1;\n    let mut j = 1;\n    while i <= 2 * n {\n        while j < i && i +\
    \ j < 2 * n && s[(i - j) / 2 - 1] == s[(i + j) / 2] {\n            j += 2;\n \
    \       }\n        a[i] = j;\n        if j == 0 {\n            i += 1;\n     \
    \       j = 1;\n            continue;\n        }\n        let mut k = 1;\n   \
    \     while k <= i && k + a[i - k] < j {\n            a[i + k] = a[i - k];\n \
    \           k += 1;\n        }\n        i += k;\n        j -= k;\n    }\n    a\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/manacher/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 08:31:37+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/enumerate_palindromes/src/main.rs
documentation_of: crates/string/manacher/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/manacher/src/lib.rs
- /library/crates/string/manacher/src/lib.rs.html
title: crates/string/manacher/src/lib.rs
---
