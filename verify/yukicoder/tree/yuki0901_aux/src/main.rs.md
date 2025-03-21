---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/901
    links:
    - https://yukicoder.me/problems/no/901
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/901\n\n\
    use heavy_light_decomposition::HeavyLightDecomposition;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  uvw: [(usize, usize, i64); n - 1],\n        q: usize,\n        vs: [[usize];\
    \ q],\n    }\n\n    let hld = HeavyLightDecomposition::from_edges(&uvw, 0);\n\
    \    let mut ws = vec![0; n - 1];\n    for &(u, v, w) in &uvw {\n        ws[hld.edge_index(u,\
    \ v)] = w;\n    }\n\n    let mut sum_ws = vec![0; n];\n    for i in 0..n - 1 {\n\
    \        sum_ws[i + 1] = sum_ws[i] + ws[i];\n    }\n\n    for vs in vs {\n   \
    \     let (_, map) = hld.compress(&vs);\n        let mut res = 0;\n        for\
    \ i in 0..map.len() {\n            let s = map[i];\n            let t = map[(i\
    \ + 1) % map.len()];\n            hld.path_query(s, t, false, |l, r, _| {\n  \
    \              res += sum_ws[r] - sum_ws[l];\n            });\n        }\n   \
    \     println!(\"{}\", res / 2);\n    }\n}\n"
  dependsOn:
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/tree/yuki0901_aux/src/main.rs
  requiredBy: []
  timestamp: '2025-03-10 08:02:08+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/tree/yuki0901_aux/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/tree/yuki0901_aux/src/main.rs
- /verify/verify/yukicoder/tree/yuki0901_aux/src/main.rs.html
title: verify/yukicoder/tree/yuki0901_aux/src/main.rs
---
