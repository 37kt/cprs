---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/binary-trie/src/lib.rs
    title: crates/data-structure/binary-trie/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/set_xor_min
    links:
    - https://judge.yosupo.jp/problem/set_xor_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min\n\
    \nuse binary_trie::BinaryTrie;\nuse proconio::fastout;\nuse proconio::input;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        q: usize,\n    }\n    let mut\
    \ st = BinaryTrie::<30>::new();\n    for _ in 0..q {\n        input! {\n     \
    \       t: usize,\n            x: usize,\n        }\n        if t == 0 {\n   \
    \         if st.count(x) == 0 {\n                st.insert(x, 1);\n          \
    \  }\n        } else if t == 1 {\n            if st.count(x) == 1 {\n        \
    \        st.remove(x, 1);\n            }\n        } else {\n            st.operate_xor(x);\n\
    \            let res = st.min().unwrap();\n            st.operate_xor(x);\n  \
    \          println!(\"{}\", res);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/binary-trie/src/lib.rs
  isVerificationFile: true
  path: verify/set_xor_min/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 07:14:43+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/set_xor_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/set_xor_min/src/main.rs
- /verify/verify/set_xor_min/src/main.rs.html
title: verify/set_xor_min/src/main.rs
---
