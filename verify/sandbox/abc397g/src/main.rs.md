---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/multivalued_optimization/src/lib.rs
    title: crates/flow/multivalued_optimization/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use multivalued_optimization::MultivaluedOptimization;\nuse proconio::{fastout,\
    \ input, marker::Usize1};\n\n#[fastout]\nfn main() {\n    input! {\n        n:\
    \ usize,\n        m: usize,\n        k: i64,\n        uv: [(Usize1, Usize1); m],\n\
    \    }\n\n    let mut opt = MultivaluedOptimization::new(vec![n; n]);\n    opt.add_unary(0,\
    \ |dv| (dv == 0).then_some(0));\n\n    for &(u, v) in &uv {\n        opt.add_binary(u,\
    \ v, |du, dv| {\n            if du >= dv {\n                Some(0)\n        \
    \    } else if du + 1 == dv {\n                Some(1)\n            } else {\n\
    \                None\n            }\n        });\n    }\n\n    let mut ok = 0;\n\
    \    let mut ng = n;\n    while ok + 1 < ng {\n        let d = (ok + ng) / 2;\n\
    \        let mut opt = opt.clone();\n        opt.add_unary(n - 1, |dv| (dv ==\
    \ d).then_some(0));\n        let (cost, _) = opt.solve();\n        if cost <=\
    \ k {\n            ok = d;\n        } else {\n            ng = d;\n        }\n\
    \    }\n    println!(\"{}\", ok);\n}\n"
  dependsOn:
  - crates/flow/multivalued_optimization/src/lib.rs
  isVerificationFile: false
  path: verify/sandbox/abc397g/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/sandbox/abc397g/src/main.rs
layout: document
redirect_from:
- /library/verify/sandbox/abc397g/src/main.rs
- /library/verify/sandbox/abc397g/src/main.rs.html
title: verify/sandbox/abc397g/src/main.rs
---
