---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/binary_trie/src/lib.rs
    title: crates/data_structure/binary_trie/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/associative_array
    links:
    - https://judge.yosupo.jp/problem/associative_array
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array\n\
    \nuse binary_trie::BinaryTrie;\nuse proconio::fastout;\nuse proconio::input;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        q: usize,\n    }\n    let mut\
    \ trie = BinaryTrie::new();\n    for _ in 0..q {\n        input! {\n         \
    \   t: usize,\n            k: usize,\n        }\n        if t == 0 {\n       \
    \     input! {\n                v: usize,\n            }\n            let p =\
    \ trie.count(k);\n            if p < v {\n                trie.insert(k, v - p);\n\
    \            } else {\n                trie.remove(k, p - v);\n            }\n\
    \        } else {\n            println!(\"{}\", trie.count(k));\n        }\n \
    \   }\n}\n"
  dependsOn:
  - crates/data_structure/binary_trie/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/associative_array_bt/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/associative_array_bt/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/associative_array_bt/src/main.rs
- /verify/verify/library_checker/data_structure/associative_array_bt/src/main.rs.html
title: verify/library_checker/data_structure/associative_array_bt/src/main.rs
---
