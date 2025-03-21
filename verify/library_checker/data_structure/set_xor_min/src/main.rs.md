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
    PROBLEM: https://judge.yosupo.jp/problem/set_xor_min
    links:
    - https://judge.yosupo.jp/problem/set_xor_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min\n\
    \nuse binary_trie::BinaryTrie;\nuse proconio::fastout;\nuse proconio::input;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        q: usize,\n    }\n    let mut\
    \ trie = BinaryTrie::new();\n    for _ in 0..q {\n        input! {\n         \
    \   t: usize,\n            x: usize,\n        }\n        match t {\n         \
    \   0 => {\n                if trie.count(x) == 0 {\n                    trie.insert(x,\
    \ 1);\n                }\n            }\n            1 => {\n                if\
    \ trie.count(x) == 1 {\n                    trie.remove(x, 1);\n             \
    \   }\n            }\n            2 => {\n                println!(\"{}\", trie.min(x).unwrap());\n\
    \            }\n            _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/binary_trie/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/set_xor_min/src/main.rs
  requiredBy: []
  timestamp: '2025-03-14 06:35:33+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/set_xor_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/set_xor_min/src/main.rs
- /verify/verify/library_checker/data_structure/set_xor_min/src/main.rs.html
title: verify/library_checker/data_structure/set_xor_min/src/main.rs
---
