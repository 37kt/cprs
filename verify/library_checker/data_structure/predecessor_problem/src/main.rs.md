---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wordsize_ary_tree_set/src/lib.rs
    title: crates/data_structure/wordsize_ary_tree_set/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/predecessor_problem
    links:
    - https://judge.yosupo.jp/problem/predecessor_problem
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem\n\
    \nuse proconio::fastout;\nuse proconio::input;\nuse proconio::marker::Bytes;\n\
    use wordsize_ary_tree_set::WordsizeAryTreeSet;\n\n#[fastout]\nfn main() {\n  \
    \  input! {\n        n: usize,\n        q: usize,\n        t: Bytes,\n    }\n\n\
    \    let mut st = WordsizeAryTreeSet::new(n);\n    for i in 0..n {\n        if\
    \ t[i] == b'1' {\n            st.insert(i);\n        }\n    }\n\n    for _ in\
    \ 0..q {\n        input! {\n            t: usize,\n            x: usize,\n   \
    \     }\n        match t {\n            0 => {\n                st.insert(x);\n\
    \            }\n            1 => {\n                st.remove(x);\n          \
    \  }\n            2 => {\n                println!(\"{}\", st[x] as i32);\n  \
    \          }\n            3 => {\n                println!(\"{}\", st.next(x).map_or(-1,\
    \ |x| x as isize));\n            }\n            4 => {\n                println!(\"\
    {}\", st.prev(x).map_or(-1, |x| x as isize));\n            }\n            _ =>\
    \ unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/wordsize_ary_tree_set/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/predecessor_problem/src/main.rs
  requiredBy: []
  timestamp: '2025-03-14 00:52:07+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/predecessor_problem/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/predecessor_problem/src/main.rs
- /verify/verify/library_checker/data_structure/predecessor_problem/src/main.rs.html
title: verify/library_checker/data_structure/predecessor_problem/src/main.rs
---
