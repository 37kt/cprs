---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
    title: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
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
  code: "use std::ops::{Range, RangeBounds};\n\nuse as_half_open_range::AsHalfOpenRange;\n\
    use centroid_decomposition::CentroidDecomposition;\nuse csr_array::CsrArray;\n\
    use graph::Edge;\n\n#[derive(Clone)]\npub struct TreeContourRange {\n    pos:\
    \ CsrArray<usize>,\n    comp: Vec<usize>,\n    depth: Vec<usize>,\n    sep: CsrArray<usize>,\n\
    }\n\nimpl TreeContourRange {\n    pub fn new<W>(g: &CsrArray<impl Edge<W>>) ->\
    \ Self {\n        let n = g.len();\n\n        let mut pos = vec![];\n        let\
    \ mut p = 0;\n\n        let mut comp_id = 0;\n\n        let mut comp = vec![];\n\
    \        let mut depth = vec![];\n\n        let mut d = vec![0; n];\n        let\
    \ mut used = vec![false; n];\n\n        let mut cnt = vec![0; n + 2];\n\n    \
    \    let mut sep = vec![];\n\n        CentroidDecomposition::solve(g, |cd| {\n\
    \            d[cd.root] = 0;\n            for &v in cd.vs {\n                d[v]\
    \ = d[cd.par[v]] + 1;\n            }\n\n            for vs in [&[cd.root], &cd.vs[..cd.mid],\
    \ &cd.vs[cd.mid..]] {\n                let mut d_max = 0;\n                for\
    \ &v in vs {\n                    if !used[v] {\n                        d_max\
    \ = d[v] + 1;\n                        cnt[d[v] + 1] = 1;\n                  \
    \  }\n                }\n\n                for i in 0..=d_max {\n            \
    \        if cnt[i + 1] == 1 {\n                        comp.push(comp_id);\n \
    \                       depth.push(i);\n                    }\n              \
    \  }\n\n                cnt[0] = p;\n                for i in 0..=d_max {\n  \
    \                  cnt[i + 1] += cnt[i];\n                    sep.push((comp_id,\
    \ cnt[i]));\n                }\n                p = cnt[d_max];\n\n          \
    \      for &v in vs {\n                    if !used[v] {\n                   \
    \     pos.push((v, cnt[d[v]]));\n                    }\n                }\n\n\
    \                cnt[..=d_max + 1].fill(0);\n                comp_id += 1;\n \
    \           }\n\n            used[cd.root] = true;\n        });\n\n        Self\
    \ {\n            pos: CsrArray::new(n, pos),\n            comp,\n            depth,\n\
    \            sep: CsrArray::new(comp_id, sep),\n        }\n    }\n\n    pub fn\
    \ point(&self, v: usize) -> impl Iterator<Item = usize> + '_ {\n        self.pos[v].iter().copied()\n\
    \    }\n\n    pub fn range(\n        &self,\n        v: usize,\n        range:\
    \ impl RangeBounds<usize>,\n    ) -> impl Iterator<Item = Range<usize>> + '_ {\n\
    \        const COMPS: [&[usize]; 3] = [&[1, 2], &[!0, 1], &[!1, !0]];\n      \
    \  let (l, r) = range.as_half_open_range(0, 1 << 63);\n        self.pos[v].iter().flat_map(move\
    \ |&p| {\n            let comp = self.comp[p];\n            let depth = self.depth[p];\n\
    \            COMPS[comp % 3]\n                .iter()\n                .map(move\
    \ |&c| c.wrapping_add(comp))\n                .filter_map(move |c| {\n       \
    \             let l = l.saturating_sub(depth);\n                    let r = r.saturating_sub(depth);\n\
    \                    let l = self.sep[c][l.min(self.sep[c].len() - 1)];\n    \
    \                let r = self.sep[c][r.min(self.sep[c].len() - 1)];\n        \
    \            (l < r).then_some(l..r)\n                })\n        })\n    }\n\n\
    \    pub fn len(&self) -> usize {\n        if self.sep.is_empty() {\n        \
    \    0\n        } else {\n            *self.sep[self.sep.len() - 1].last().unwrap()\n\
    \        }\n    }\n\n    pub fn is_empty(&self) -> bool {\n        self.len()\
    \ == 0\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/misc/as_half_open_range/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/tree_contour_range/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-08 12:53:34+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
documentation_of: crates/tree/tree_contour_range/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/tree_contour_range/src/lib.rs
- /library/crates/tree/tree_contour_range/src/lib.rs.html
title: crates/tree/tree_contour_range/src/lib.rs
---
