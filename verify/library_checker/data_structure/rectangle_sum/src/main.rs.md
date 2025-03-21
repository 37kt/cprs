---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
    title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/internal.rs
    title: crates/data_structure/wavelet_matrix/src/internal.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum\n\
    \nuse proconio::fastout;\nuse proconio::input;\nuse std::ops::Range;\nuse wavelet_matrix::WaveletMatrix2D;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        xyw: [((u32, u32), i64); n],\n    }\n\n    let (xy, w): (Vec<_>, Vec<_>)\
    \ = xyw.into_iter().unzip();\n\n    let (wm, css) =\n        WaveletMatrix2D::<_,\
    \ _, false, false, true>::new_2d_with_containers(&xy, |idx| {\n            std::iter::once(0)\n\
    \                .chain(idx.iter().scan(0, |acc, &i| {\n                    *acc\
    \ += w[i];\n                    Some(*acc)\n                }))\n            \
    \    .collect::<Vec<_>>()\n        });\n\n    for _ in 0..q {\n        input!\
    \ {\n            xl: u32,\n            yl: u32,\n            xr: u32,\n      \
    \      yr: u32,\n        }\n        let mut s = 0;\n        wm.count_with(xl..xr,\
    \ yl..yr, |d, rng, inv| {\n            let Range { start: l, end: r } = rng;\n\
    \            let t = css[d][r] - css[d][l];\n            s += if inv { -t } else\
    \ { t };\n        });\n        println!(\"{}\", s);\n    }\n}\n"
  dependsOn:
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/rectangle_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-03-10 07:35:38+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/rectangle_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/rectangle_sum/src/main.rs
- /verify/verify/library_checker/data_structure/rectangle_sum/src/main.rs.html
title: verify/library_checker/data_structure/rectangle_sum/src/main.rs
---
