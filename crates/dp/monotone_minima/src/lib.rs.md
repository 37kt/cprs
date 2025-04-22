---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/lib.rs
    title: crates/convolution/minplus_convolution/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/dp/yuki0952_mm/src/main.rs
    title: verify/yukicoder/dp/yuki0952_mm/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// h * w \u884C\u5217 A \u306E\u5404\u884C\u306E argmin \u304C\u5358\u8ABF\
    \u5897\u52A0\u3067\u3042\u308B\u3068\u304D\u306E argmin \u3092\u5217\u6319\u3059\
    \u308B\u3002  \n///\n/// # \u5F15\u6570\n/// - `h`: \u884C\u5217\u306E\u884C\u6570\
    \u3002\n/// - `w`: \u884C\u5217\u306E\u5217\u6570\u3002\n/// - `select(i, j, k)`:\
    \ `A[i][j] >= A[i][k]` \u3092\u8FD4\u3059\u95A2\u6570\u3002 (`j < k` \u304C\u4FDD\
    \u8A3C\u3055\u308C\u3066\u3044\u308B)\n///\n/// # \u623B\u308A\u5024\n/// \u5404\
    \u884C\u306E argmin\npub fn monotone_minima(\n    h: usize,\n    w: usize,\n \
    \   mut select: impl FnMut(usize, usize, usize) -> bool,\n) -> Vec<usize> {\n\
    \    let mut argmin = vec![0; h];\n    dfs(0, h, 0, w, &mut select, &mut argmin);\n\
    \    argmin\n}\n\nfn dfs(\n    il: usize,\n    ir: usize,\n    jl: usize,\n  \
    \  jr: usize,\n    select: &mut impl FnMut(usize, usize, usize) -> bool,\n   \
    \ argmin: &mut [usize],\n) {\n    if il >= ir {\n        return;\n    }\n\n  \
    \  let im = (il + ir) / 2;\n    argmin[im] = jl;\n    for j in jl + 1..jr {\n\
    \        if select(im, argmin[im], j) {\n            argmin[im] = j;\n       \
    \ }\n    }\n\n    dfs(il, im, jl, argmin[im] + 1, select, argmin);\n    dfs(im\
    \ + 1, ir, argmin[im], jr, select, argmin);\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/dp/monotone_minima/src/lib.rs
  requiredBy:
  - crates/convolution/minplus_convolution/src/lib.rs
  timestamp: '2025-03-20 00:46:18+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/dp/yuki0952_mm/src/main.rs
documentation_of: crates/dp/monotone_minima/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/monotone_minima/src/lib.rs
- /library/crates/dp/monotone_minima/src/lib.rs.html
title: crates/dp/monotone_minima/src/lib.rs
---
