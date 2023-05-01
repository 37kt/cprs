---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/scc
    links:
    - https://judge.yosupo.jp/problem/scc
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc\n\nuse\
    \ graph::Graph;\nuse itertools::Itertools;\nuse proconio::input;\nuse strongly_connected_components::strongly_connected_components;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ m: usize,\n    }\n    let mut g = Graph::<(), ()>::new(n);\n    for _ in 0..m\
    \ {\n        input! {\n            a: usize,\n            b: usize,\n        }\n\
    \        g.add_edge(a, b, ());\n    }\n    let h = strongly_connected_components(&g);\n\
    \    println!(\"{}\", h.size());\n    for i in 0..h.size() {\n        let vs =\
    \ h.vertex(i);\n        println!(\"{} {}\", vs.len(), vs.iter().join(\" \"));\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/scc/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/scc/src/main.rs
layout: document
redirect_from:
- /verify/verify/scc/src/main.rs
- /verify/verify/scc/src/main.rs.html
title: verify/scc/src/main.rs
---
