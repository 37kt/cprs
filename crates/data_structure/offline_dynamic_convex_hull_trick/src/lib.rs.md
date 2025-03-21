---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/line_add_get_min/src/main.rs
    title: verify/library_checker/data_structure/line_add_get_min/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use numeric_traits::{Inf, Integer};\n\npub struct OfflineDynamicConvexHullTrick\
    \ {\n    lines: Vec<(i64, i64)>,\n    add_time: Vec<usize>,\n    remove_time:\
    \ Vec<usize>,\n    points: Vec<i64>,\n    get_time: Vec<usize>,\n    time: usize,\n\
    \    last_is_get: bool,\n\n    st: Vec<usize>,\n    res: Vec<i64>,\n}\n\nimpl\
    \ OfflineDynamicConvexHullTrick {\n    pub fn new() -> Self {\n        Self {\n\
    \            lines: vec![],\n            add_time: vec![],\n            remove_time:\
    \ vec![],\n            points: vec![],\n            get_time: vec![],\n      \
    \      time: 0,\n            last_is_get: false,\n\n            st: vec![],\n\
    \            res: vec![],\n        }\n    }\n\n    /// \u76F4\u7DDA\u3092\u8FFD\
    \u52A0\u3057\u3066\u3001\u305D\u306E `id` \u3092\u8FD4\u3059  \n    pub fn add_line(&mut\
    \ self, a: i64, b: i64) -> usize {\n        if std::mem::replace(&mut self.last_is_get,\
    \ false) {\n            self.time += 1;\n        }\n        let id = self.lines.len();\n\
    \        self.lines.push((a, b));\n        self.add_time.push(self.time);\n  \
    \      self.remove_time.push(usize::inf());\n        id\n    }\n\n    /// \u76F4\
    \u7DDA `id` \u3092\u524A\u9664\u3059\u308B\n    pub fn remove_line(&mut self,\
    \ id: usize) {\n        assert!(id < self.lines.len(), \"line {} is not added\"\
    , id);\n        if std::mem::replace(&mut self.last_is_get, false) {\n       \
    \     self.time += 1;\n        }\n        assert_eq!(\n            std::mem::replace(&mut\
    \ self.remove_time[id], self.time),\n            usize::inf(),\n            \"\
    line {} is already removed\",\n            id\n        );\n    }\n\n    /// min(ax\
    \ + b) \u3092\u6C42\u3081\u308B\u30AF\u30A8\u30EA\u3092\u767A\u884C\u3057\u3066\
    \u3001\u30AF\u30A8\u30EA\u306E `id` \u3092\u8FD4\u3059  \n    /// \u7B54\u3048\
    \u306F `solve` \u3067\u8FD4\u3063\u3066\u304F\u308B\u914D\u5217\u306E `id` \u756A\
    \u76EE\u306B\u5165\u308B\n    pub fn min(&mut self, x: i64) -> usize {\n     \
    \   self.last_is_get = true;\n        let id = self.points.len();\n        self.points.push(x);\n\
    \        self.get_time.push(self.time);\n        id\n    }\n\n    /// \u3059\u3079\
    \u3066\u306E `min` \u30AF\u30A8\u30EA\u306E\u7B54\u3048\u3092\u8FD4\u3059  \n\
    \    pub fn solve(mut self) -> Vec<i64> {\n        if self.points.is_empty() {\n\
    \            return vec![];\n        }\n        if self.last_is_get {\n      \
    \      self.time += 1;\n        }\n        let lg = self.time.ceil_log2();\n \
    \       let n = 1 << lg;\n        for t in &mut self.remove_time {\n         \
    \   if *t >= self.time {\n                *t = n;\n            }\n        }\n\n\
    \        let mut lines = (0..self.lines.len()).collect::<Vec<_>>();\n        lines.sort_by_key(|&i|\
    \ {\n            let (a, b) = self.lines[i];\n            (-a, b)\n        });\n\
    \        let mut points = (0..self.points.len()).collect::<Vec<_>>();\n      \
    \  points.sort_by_key(|&i| -self.points[i]);\n\n        self.res.resize(self.points.len(),\
    \ i64::inf());\n        self.dfs(0, n, lines, points);\n        self.res\n   \
    \ }\n\n    fn eval(&self, i: usize, x: i64) -> i64 {\n        let (a, b) = self.lines[i];\n\
    \        a * x + b\n    }\n\n    fn f(&self, i: usize, j: usize) -> i64 {\n  \
    \      let (a, b) = self.lines[i];\n        let (c, d) = self.lines[j];\n    \
    \    (d - b).floor_div(a - c)\n    }\n\n    fn check(&self, i: usize, j: usize,\
    \ k: usize) -> bool {\n        self.f(i, j) < self.f(j, k)\n    }\n\n    fn dfs(&mut\
    \ self, l: usize, r: usize, lines: Vec<usize>, points: Vec<usize>) {\n       \
    \ if lines.is_empty() || points.is_empty() {\n            return;\n        }\n\
    \        let m = (l + r) / 2;\n        self.st.clear();\n\n        let mut lines_l\
    \ = vec![];\n        let mut points_l = vec![];\n        let mut lines_r = vec![];\n\
    \        let mut points_r = vec![];\n        for li in lines {\n            let\
    \ tl = self.add_time[li];\n            let tr = self.remove_time[li];\n      \
    \      if tl <= l && r <= tr {\n                if let Some(&lj) = self.st.last()\
    \ {\n                    if self.lines[lj].0 == self.lines[li].0 {\n         \
    \               continue;\n                    }\n                }\n        \
    \        while let &[.., lk, lj] = &self.st[..] {\n                    if self.check(lk,\
    \ lj, li) {\n                        break;\n                    } else {\n  \
    \                      self.st.pop();\n                    }\n               \
    \ }\n                self.st.push(li);\n                continue;\n          \
    \  }\n            if l + 1 < r {\n                if tl < m {\n              \
    \      lines_l.push(li);\n                }\n                if m < tr {\n   \
    \                 lines_r.push(li);\n                }\n            }\n      \
    \  }\n\n        for pi in points {\n            let x = self.points[pi];\n   \
    \         while let &[.., li, lj] = &self.st[..] {\n                if self.eval(li,\
    \ x) <= self.eval(lj, x) {\n                    self.st.pop();\n             \
    \   } else {\n                    break;\n                }\n            }\n \
    \           if let Some(&li) = self.st.last() {\n                self.res[pi]\
    \ = self.res[pi].min(self.eval(li, x));\n            }\n            if l + 1 <\
    \ r {\n                if self.get_time[pi] < m {\n                    points_l.push(pi);\n\
    \                } else {\n                    points_r.push(pi);\n          \
    \      }\n            }\n        }\n        if l + 1 < r {\n            self.dfs(l,\
    \ m, lines_l, points_l);\n            self.dfs(m, r, lines_r, points_r);\n   \
    \     }\n    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/line_add_get_min/src/main.rs
documentation_of: crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
- /library/crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs.html
title: crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
---
