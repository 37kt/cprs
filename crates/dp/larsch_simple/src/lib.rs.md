---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/dp/yuki0705_larsch/src/main.rs
    title: verify/yukicoder/dp/yuki0705_larsch/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://noshi91.hatenablog.com/entry/2023/02/18/005856#fn-2ea1eb4f
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://noshi91.hatenablog.com/entry/2023/02/18/005856#fn-2ea1eb4f\n\n\
    /// \u7C21\u6613\u7248 LARSCH (\u30AA\u30F3\u30E9\u30A4\u30F3\u30FB\u30E9\u30F3\
    \u30C0\u30E0\u30A2\u30AF\u30BB\u30B9)  \n/// \u8A73\u7D30\u306F\u4E0A\u8A18\u306E\
    \u8A18\u4E8B\u3092\u53C2\u7167  \n///\n/// # \u5F15\u6570\n/// * `n`: Monge \u4E0B\
    \u4E09\u89D2\u884C\u5217\u306E\u30B5\u30A4\u30BA ( (n+1) * (n+1) \u884C\u5217\
    \ )\n/// * `f`: `(i, j, x) -> A[i][j]` (i > j)\n///   - `x` \u306F `j` \u884C\u76EE\
    \u306E\u6700\u5C0F\u5024\n/// * `init`: `A[0][0]` \u306E\u5024\n///\n/// # \u8FD4\
    \u308A\u5024\n/// * `res[i]` = `i` \u884C\u76EE\u306E `(min, argmin)`\npub fn\
    \ larsch<T: PartialOrd>(\n    n: usize,\n    mut f: impl FnMut(usize, usize, &T)\
    \ -> T,\n    init: T,\n) -> Vec<(T, usize)> {\n    let mut min = (0..n + 1).map(|_|\
    \ (None, 0)).collect::<Vec<_>>();\n    min[0] = (Some(init), 0);\n\n    check(n,\
    \ 0, &mut f, &mut min);\n    solve(0, n, &mut f, &mut min);\n\n    min.into_iter().map(|(x,\
    \ j)| (x.unwrap(), j)).collect()\n}\n\nfn check<T: PartialOrd>(\n    i: usize,\n\
    \    j: usize,\n    f: &mut impl FnMut(usize, usize, &T) -> T,\n    min: &mut\
    \ [(Option<T>, usize)],\n) {\n    let x = f(i, j, min[j].0.as_ref().unwrap());\n\
    \    if min[i].0.is_none() || min[i].0.as_ref().unwrap() > &x {\n        min[i]\
    \ = (Some(x), j);\n    }\n}\n\nfn solve<T: PartialOrd>(\n    l: usize,\n    r:\
    \ usize,\n    f: &mut impl FnMut(usize, usize, &T) -> T,\n    min: &mut [(Option<T>,\
    \ usize)],\n) {\n    if l + 1 >= r {\n        return;\n    }\n\n    let m = (l\
    \ + r) / 2;\n    for j in min[l].1..=min[r].1 {\n        check(m, j, f, min);\n\
    \    }\n    solve(l, m, f, min);\n    for j in l + 1..=m {\n        check(r, j,\
    \ f, min);\n    }\n    solve(m, r, f, min);\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/dp/larsch_simple/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-20 04:28:46+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/dp/yuki0705_larsch/src/main.rs
documentation_of: crates/dp/larsch_simple/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/larsch_simple/src/lib.rs
- /library/crates/dp/larsch_simple/src/lib.rs.html
title: crates/dp/larsch_simple/src/lib.rs
---
