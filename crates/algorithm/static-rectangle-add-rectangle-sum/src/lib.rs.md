---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/static_rectangle_add_rectangle_sum/src/main.rs
    title: verify/static_rectangle_add_rectangle_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    mem::swap,\n    ops::{Add, Mul, Neg, Sub},\n};\n\npub fn\
    \ static_rectangle_add_rectangle_sum<I, T>(\n    add: &[(I, I, I, I, T)],\n  \
    \  sum: &[(I, I, I, I)],\n) -> Vec<T>\nwhere\n    I: Copy + TryInto<i64>,\n  \
    \  T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T>\
    \ + Neg<Output = T>,\n{\n    let cast = |x: I| unsafe { x.try_into().unwrap_unchecked()\
    \ };\n\n    let mut ys: Vec<i64> = vec![];\n    for &(_, _, yl, yr, _) in add\
    \ {\n        ys.push(cast(yl));\n        ys.push(cast(yr));\n    }\n    ys.sort();\n\
    \    ys.dedup();\n\n    let bs = |y: i64| match ys.binary_search(&y) {\n     \
    \   Ok(i) => i,\n        Err(i) => i,\n    };\n\n    let mut evs = vec![];\n \
    \   for i in 0..sum.len() {\n        let (xl, xr, _, _) = sum[i];\n        evs.push((cast(xl),\
    \ 0, i));\n        evs.push((cast(xr), 1, i));\n    }\n    for i in 0..add.len()\
    \ {\n        let (xl, xr, _, _, _) = add[i];\n        evs.push((cast(xl), 2, i));\n\
    \        evs.push((cast(xr), 3, i));\n    }\n    evs.sort();\n\n    let mut fxy\
    \ = FenwickTree::new(ys.len());\n    let mut fx = FenwickTree::new(ys.len());\n\
    \    let mut fy = FenwickTree::new(ys.len());\n    let mut f = FenwickTree::new(ys.len());\n\
    \    let mut res = vec![T::default(); sum.len()];\n    for (x, t, q) in evs {\n\
    \        if t & 2 != 0 {\n            let (_, _, mut yl, mut yr, w) = add[q];\n\
    \            let mut i = bs(cast(yl));\n            let mut j = bs(cast(yr));\n\
    \            if t & 1 != 0 {\n                swap(&mut i, &mut j);\n        \
    \        swap(&mut yl, &mut yr);\n            }\n            fxy.add(i, w * x\
    \ * cast(yl));\n            fxy.add(j, -w * x * cast(yr));\n            fx.add(i,\
    \ -w * x);\n            fx.add(j, w * x);\n            fy.add(i, -w * cast(yl));\n\
    \            fy.add(j, w * cast(yr));\n            f.add(i, w);\n            f.add(j,\
    \ -w);\n        } else {\n            let (_, _, mut yl, mut yr) = sum[q];\n \
    \           let mut i = bs(cast(yl));\n            let mut j = bs(cast(yr));\n\
    \            if t & 1 != 0 {\n                swap(&mut i, &mut j);\n        \
    \        swap(&mut yl, &mut yr);\n            }\n            res[q] = res[q]\n\
    \                + fxy.accum(i)\n                + fx.accum(i) * cast(yl)\n  \
    \              + fy.accum(i) * x\n                + f.accum(i) * x * cast(yl);\n\
    \            res[q] = res[q]\n                - fxy.accum(j)\n               \
    \ - fx.accum(j) * cast(yr)\n                - fy.accum(j) * x\n              \
    \  - f.accum(j) * x * cast(yr);\n        }\n    }\n\n    res\n}\n\nstruct FenwickTree<T>\n\
    where\n    T: Copy + Default + Add<Output = T>,\n{\n    v: Vec<T>,\n}\n\nimpl<T>\
    \ FenwickTree<T>\nwhere\n    T: Copy + Default + Add<Output = T>,\n{\n    fn new(n:\
    \ usize) -> Self {\n        FenwickTree {\n            v: vec![T::default(); n],\n\
    \        }\n    }\n\n    fn add(&mut self, mut k: usize, x: T) {\n        assert!(k\
    \ <= self.v.len());\n        k += 1;\n        while k <= self.v.len() {\n    \
    \        self.v[k - 1] = self.v[k - 1] + x;\n            k += k & k.wrapping_neg();\n\
    \        }\n    }\n\n    fn accum(&self, mut k: usize) -> T {\n        let mut\
    \ res = T::default();\n        while k > 0 {\n            res = res + self.v[k\
    \ - 1];\n            k &= k - 1;\n        }\n        res\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-14 13:30:35+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/static_rectangle_add_rectangle_sum/src/main.rs
documentation_of: crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
- /library/crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs.html
title: crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
---
