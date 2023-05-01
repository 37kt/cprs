---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/segment_add_get_min
    links:
    - https://judge.yosupo.jp/problem/segment_add_get_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/segment_add_get_min\n\
    \nuse li_chao_tree_dynamic::LiChaoTree;\nuse proconio::input;\n\nconst MAX: i64\
    \ = 1_000_000_000;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n      \
    \  n: usize,\n        q: usize,\n    }\n    let mut lct = LiChaoTree::new(-MAX,\
    \ MAX, false);\n    for _ in 0..n {\n        input! {\n            l: i64,\n \
    \           r: i64,\n            a: i64,\n            b: i64,\n        }\n   \
    \     lct.add_segment(a, b, l, r);\n    }\n    for _ in 0..q {\n        input!\
    \ {\n            ty: usize,\n        }\n        if ty == 0 {\n            input!\
    \ {\n                l: i64,\n                r: i64,\n                a: i64,\n\
    \                b: i64,\n            }\n            lct.add_segment(a, b, l,\
    \ r);\n        } else {\n            input! {\n                p: i64,\n     \
    \       }\n            if let Some(res) = lct.find(p) {\n                println!(\"\
    {}\", res);\n            } else {\n                println!(\"INFINITY\");\n \
    \           }\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/segment_add_get_min/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/segment_add_get_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/segment_add_get_min/src/main.rs
- /verify/verify/segment_add_get_min/src/main.rs.html
title: verify/segment_add_get_min/src/main.rs
---
