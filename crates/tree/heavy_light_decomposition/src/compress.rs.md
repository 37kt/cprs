---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
    title: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_tree_dp/src/lib.rs
    title: crates/tree/dynamic_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static_top_tree/src/lib.rs
    title: crates/tree/static_top_tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/jump_on_tree/src/main.rs
    title: verify/library_checker/tree/jump_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/lca/src/main.rs
    title: verify/library_checker/tree/lca/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/tree/yuki0901_aux/src/main.rs
    title: verify/yukicoder/tree/yuki0901_aux/src/main.rs
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
  code: "use crate::HeavyLightDecomposition;\n\nimpl HeavyLightDecomposition {\n \
    \   /// \u6307\u5B9A\u3055\u308C\u305F\u9802\u70B9\u305F\u3061\u306E\u6700\u5C0F\
    \u5171\u901A\u7956\u5148\u95A2\u4FC2\u3092\u4FDD\u3063\u3066\u6728\u3092\u5727\
    \u7E2E\u3057\u3066\u3067\u304D\u308B\u88DC\u52A9\u7684\u306A\u6728  \n    ///\
    \ `(par, map)` \u3092\u8FD4\u3059  \n    /// `par[i]`: \u5727\u7E2E\u5F8C\u306E\
    \u9802\u70B9 `i` \u306E\u89AA  \n    /// `par[0] == 0`, `i > 0` \u306B\u3064\u3044\
    \u3066 `par[i] < i`  \n    /// `map[i]`: \u5727\u7E2E\u524D\u306E\u9802\u70B9\u305F\
    \u3061\u306E\u3046\u3061 `i` \u306B\u5BFE\u5FDC\u3059\u308B\u3082\u306E\n    pub\
    \ fn compress(&self, vs: &[usize]) -> (Vec<usize>, Vec<usize>) {\n        if vs.is_empty()\
    \ {\n            return (vec![], vec![]);\n        }\n\n        let mut v = Vec::with_capacity(vs.len()\
    \ * 2 - 1);\n        v.extend(vs);\n        v.sort_unstable_by_key(|&v| self.down[v]);\n\
    \        for i in 0..v.len() - 1 {\n            v.push(self.lca(v[i], v[i + 1]));\n\
    \        }\n        v.sort_unstable_by_key(|&v| self.down[v]);\n        v.dedup();\n\
    \n        let sz = v.len();\n        let mut par = vec![0; sz];\n        let mut\
    \ st = vec![0];\n        for i in 1..sz {\n            while !self.in_subtree(v[*st.last().unwrap()],\
    \ v[i]) {\n                st.pop();\n            }\n            par[i] = *st.last().unwrap();\n\
    \            st.push(i);\n        }\n\n        (par, v)\n    }\n}\n"
  dependsOn:
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/heavy_light_decomposition/src/compress.rs
  requiredBy:
  - crates/tree/static_top_tree/src/lib.rs
  - crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  timestamp: '2025-04-22 05:57:06+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/lca/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - verify/library_checker/tree/jump_on_tree/src/main.rs
  - verify/yukicoder/tree/yuki0901_aux/src/main.rs
documentation_of: crates/tree/heavy_light_decomposition/src/compress.rs
layout: document
redirect_from:
- /library/crates/tree/heavy_light_decomposition/src/compress.rs
- /library/crates/tree/heavy_light_decomposition/src/compress.rs.html
title: crates/tree/heavy_light_decomposition/src/compress.rs
---
