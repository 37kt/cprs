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
    PROBLEM: https://judge.yosupo.jp/problem/ordered_set
    links:
    - https://judge.yosupo.jp/problem/ordered_set
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/ordered_set\n\
    \nuse binary_trie::BinaryTrie;\nuse proconio::fastout;\nuse proconio::input;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [usize; n],\n    }\n    let xor = 0xE869120; // \u5168\u4F53 xor \u306E\
    \ test\n    let mut trie = BinaryTrie::new();\n    for &x in &a {\n        trie.insert(x\
    \ ^ xor, 1);\n    }\n    for _ in 0..q {\n        input! {\n            t: usize,\n\
    \            x: usize,\n        }\n        match t {\n            0 => {\n   \
    \             if trie.count(x ^ xor) == 0 {\n                    trie.insert(x\
    \ ^ xor, 1);\n                }\n            }\n            1 => {\n         \
    \       if trie.count(x ^ xor) == 1 {\n                    trie.remove(x ^ xor,\
    \ 1);\n                }\n            }\n            2 => {\n                println!(\"\
    {}\", trie.kth_smallest(x - 1, xor).map_or(-1, |x| x as i64));\n            }\n\
    \            3 => {\n                println!(\"{}\", trie.count_range(..=x, xor));\n\
    \            }\n            4 => {\n                let cnt = trie.count_range(x\
    \ + 1.., xor);\n                println!(\"{}\", trie.kth_largest(cnt, xor).map_or(-1,\
    \ |x| x as i64));\n            }\n            5 => {\n                let cnt\
    \ = trie.count_range(..x, xor);\n                println!(\"{}\", trie.kth_smallest(cnt,\
    \ xor).map_or(-1, |x| x as i64));\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/binary_trie/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/ordered_set/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/ordered_set/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/ordered_set/src/main.rs
- /verify/verify/library_checker/data_structure/ordered_set/src/main.rs.html
title: verify/library_checker/data_structure/ordered_set/src/main.rs
---
