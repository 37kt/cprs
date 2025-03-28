---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/lib.rs
    title: crates/number_theory/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
    title: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
    title: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/numeric.rs
    title: crates/number_theory/modint/static_modint/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ops.rs
    title: crates/number_theory/modint/static_modint/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
    links:
    - https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance\n\
    \nuse centroid_decomposition::CentroidDecomposition;\nuse convolution::convolution_ntt_friendly;\n\
    use graph::{GraphBuilder, UndirectedGraph};\nuse proconio::{fastout, input};\n\
    use static_modint::StaticModInt;\n\nconst P1: u32 = 998_244_353;\nconst P2: u32\
    \ = 1_004_535_809;\ntype Fp1 = StaticModInt<P1>;\ntype Fp2 = StaticModInt<P2>;\n\
    \n#[fastout]\nfn main() {\n    let m1inv_fp2 = Fp2::from_raw(P1).recip();\n\n\
    \    input! {\n        n: usize,\n        ab: [(usize, usize); n - 1],\n    }\n\
    \    let g = UndirectedGraph::from_edges(n, ab);\n    let mut res = vec![0; n];\n\
    \    let mut depth = vec![0; n];\n    CentroidDecomposition::solve(&g, |cd| {\n\
    \        depth[cd.root] = 0;\n        let mut f1 = vec![vec![]; 2];\n        let\
    \ mut f2 = vec![vec![]; 2];\n        for (i, &v) in cd.vs.iter().enumerate() {\n\
    \            depth[v] = depth[cd.par[v]] + 1;\n            let c = if i < cd.mid\
    \ { 0 } else { 1 };\n            if f1[c].len() <= depth[v] {\n              \
    \  f1[c].resize(depth[v] + 1, Fp1::from_raw(0));\n                f2[c].resize(depth[v]\
    \ + 1, Fp2::from_raw(0));\n            }\n            f1[c][depth[v]] += 1;\n\
    \            f2[c][depth[v]] += 1;\n        }\n        let g1 = convolution_ntt_friendly(&f1[0],\
    \ &f1[1]);\n        let g2 = convolution_ntt_friendly(&f2[0], &f2[1]);\n     \
    \   for (i, (e1, e2)) in g1.into_iter().zip(g2).enumerate() {\n            let\
    \ x1 = e1;\n            let x2 = (e2 - Fp2::from_raw(e1.val())) * m1inv_fp2;\n\
    \            res[i] += (x1.val() as u64) + (x2.val() as u64) * (P1 as u64);\n\
    \        }\n    });\n    res[1] += (n - 1) as u64;\n\n    for x in &res[1..] {\n\
    \        print!(\"{} \", x);\n    }\n    println!();\n}\n"
  dependsOn:
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  requiredBy: []
  timestamp: '2025-03-28 02:27:58+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
- /verify/verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs.html
title: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
---
