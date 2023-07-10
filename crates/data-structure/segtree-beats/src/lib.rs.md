---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/range_chmin_chmax_add_range_sum/src/main.rs
    title: verify/range_chmin_chmax_add_range_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\ntype T = i64;\nconst INF: T = std::i64::MAX\
    \ / 2;\n\n#[derive(Clone)]\npub struct SegtreeBeats {\n    n: usize,\n    max_v:\
    \ Vec<T>,\n    smax_v: Vec<T>,\n    max_c: Vec<T>,\n    min_v: Vec<T>,\n    smin_v:\
    \ Vec<T>,\n    min_c: Vec<T>,\n    sum: Vec<T>,\n    len: Vec<T>,\n    ladd: Vec<T>,\n\
    \    lval: Vec<T>,\n}\n\nimpl From<&[T]> for SegtreeBeats {\n    fn from(a: &[T])\
    \ -> Self {\n        let mut n = 1;\n        while n < a.len() {\n           \
    \ n *= 2;\n        }\n\n        let mut max_v = vec![0; n * 2];\n        let mut\
    \ smax_v = vec![0; n * 2];\n        let mut max_c = vec![0; n * 2];\n        let\
    \ mut min_v = vec![0; n * 2];\n        let mut smin_v = vec![0; n * 2];\n    \
    \    let mut min_c = vec![0; n * 2];\n        let mut sum = vec![0; n * 2];\n\
    \        let mut len = vec![0; n * 2];\n        let ladd = vec![0; n * 2];\n \
    \       let lval = vec![INF; n * 2];\n\n        len[0] = n as i64;\n        for\
    \ i in 0..n - 1 {\n            len[i * 2 + 1] = len[i] / 2;\n            len[i\
    \ * 2 + 2] = len[i] / 2;\n        }\n\n        for i in 0..a.len() {\n       \
    \     max_v[n - 1 + i] = a[i];\n            min_v[n - 1 + i] = a[i];\n       \
    \     sum[n - 1 + i] = a[i];\n            smax_v[n - 1 + i] = -INF;\n        \
    \    smin_v[n - 1 + i] = INF;\n            max_c[n - 1 + i] = 1;\n           \
    \ min_c[n - 1 + i] = 1;\n        }\n\n        for i in a.len()..n {\n        \
    \    max_v[n - 1 + i] = -INF;\n            smax_v[n - 1 + i] = -INF;\n       \
    \     min_v[n - 1 + i] = INF;\n            smin_v[n - 1 + i] = INF;\n        }\n\
    \n        let mut seg = SegtreeBeats {\n            n,\n            max_v,\n \
    \           smax_v,\n            max_c,\n            min_v,\n            smin_v,\n\
    \            min_c,\n            sum,\n            len,\n            ladd,\n \
    \           lval,\n        };\n\n        for i in (0..n - 1).rev() {\n       \
    \     seg.update(i);\n        }\n\n        seg\n    }\n}\n\nimpl SegtreeBeats\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self::from(vec![0; n].as_slice())\n\
    \    }\n\n    pub fn chmin<R>(&mut self, range: R, x: T)\n    where\n        R:\
    \ RangeBounds<usize>,\n    {\n        let (a, b) = self.range_to_pair(range);\n\
    \        self.chmin_(x, a, b, 0, 0, self.n);\n    }\n\n    pub fn chmax<R>(&mut\
    \ self, range: R, x: T)\n    where\n        R: RangeBounds<usize>,\n    {\n  \
    \      let (a, b) = self.range_to_pair(range);\n        self.chmax_(x, a, b, 0,\
    \ 0, self.n);\n    }\n\n    pub fn add<R>(&mut self, range: R, x: T)\n    where\n\
    \        R: RangeBounds<usize>,\n    {\n        let (a, b) = self.range_to_pair(range);\n\
    \        self.add_(x, a, b, 0, 0, self.n);\n    }\n\n    pub fn set<R>(&mut self,\
    \ range: R, x: T)\n    where\n        R: RangeBounds<usize>,\n    {\n        let\
    \ (a, b) = self.range_to_pair(range);\n        self.set_(x, a, b, 0, 0, self.n);\n\
    \    }\n\n    pub fn max<R>(&mut self, range: R) -> T\n    where\n        R: RangeBounds<usize>,\n\
    \    {\n        let (a, b) = self.range_to_pair(range);\n        self.max_(a,\
    \ b, 0, 0, self.n)\n    }\n\n    pub fn min<R>(&mut self, range: R) -> T\n   \
    \ where\n        R: RangeBounds<usize>,\n    {\n        let (a, b) = self.range_to_pair(range);\n\
    \        self.min_(a, b, 0, 0, self.n)\n    }\n\n    pub fn sum<R>(&mut self,\
    \ range: R) -> T\n    where\n        R: RangeBounds<usize>,\n    {\n        let\
    \ (a, b) = self.range_to_pair(range);\n        self.sum_(a, b, 0, 0, self.n)\n\
    \    }\n\n    fn update_node_max(&mut self, k: usize, x: T) {\n        self.sum[k]\
    \ += (x - self.max_v[k]) * self.max_c[k];\n\n        if self.max_v[k] == self.min_v[k]\
    \ {\n            self.max_v[k] = x;\n            self.min_v[k] = x;\n        }\
    \ else if self.max_v[k] == self.smin_v[k] {\n            self.max_v[k] = x;\n\
    \            self.smin_v[k] = x;\n        } else {\n            self.max_v[k]\
    \ = x;\n        }\n\n        if self.lval[k] != INF && x < self.lval[k] {\n  \
    \          self.lval[k] = x;\n        }\n    }\n\n    fn update_node_min(&mut\
    \ self, k: usize, x: T) {\n        self.sum[k] += (x - self.min_v[k]) * self.min_c[k];\n\
    \n        if self.max_v[k] == self.min_v[k] {\n            self.max_v[k] = x;\n\
    \            self.min_v[k] = x;\n        } else if self.smax_v[k] == self.min_v[k]\
    \ {\n            self.min_v[k] = x;\n            self.smax_v[k] = x;\n       \
    \ } else {\n            self.min_v[k] = x;\n        }\n\n        if self.lval[k]\
    \ != INF && self.lval[k] < x {\n            self.lval[k] = x;\n        }\n   \
    \ }\n\n    fn add_all(&mut self, k: usize, x: T) {\n        self.max_v[k] += x;\n\
    \        if self.smax_v[k] != -INF {\n            self.smax_v[k] += x;\n     \
    \   }\n        self.min_v[k] += x;\n        if self.smin_v[k] != INF {\n     \
    \       self.smin_v[k] += x;\n        }\n\n        self.sum[k] += self.len[k]\
    \ * x;\n        if self.lval[k] != INF {\n            self.lval[k] += x;\n   \
    \     } else {\n            self.ladd[k] += x;\n        }\n    }\n\n    fn set_all(&mut\
    \ self, k: usize, x: T) {\n        self.max_v[k] = x;\n        self.smax_v[k]\
    \ = -INF;\n        self.min_v[k] = x;\n        self.smin_v[k] = INF;\n       \
    \ self.max_c[k] = self.len[k];\n        self.min_c[k] = self.len[k];\n\n     \
    \   self.sum[k] = x * self.len[k];\n        self.lval[k] = x;\n        self.ladd[k]\
    \ = 0;\n    }\n\n    fn push(&mut self, k: usize) {\n        if self.n - 1 <=\
    \ k {\n            return;\n        }\n\n        if self.lval[k] != INF {\n  \
    \          self.set_all(k * 2 + 1, self.lval[k]);\n            self.set_all(k\
    \ * 2 + 2, self.lval[k]);\n            self.lval[k] = INF;\n            return;\n\
    \        }\n\n        if self.ladd[k] != 0 {\n            self.add_all(k * 2 +\
    \ 1, self.ladd[k]);\n            self.add_all(k * 2 + 2, self.ladd[k]);\n    \
    \        self.ladd[k] = 0;\n        }\n\n        if self.max_v[k] < self.max_v[k\
    \ * 2 + 1] {\n            self.update_node_max(k * 2 + 1, self.max_v[k]);\n  \
    \      }\n        if self.min_v[k * 2 + 1] < self.min_v[k] {\n            self.update_node_min(k\
    \ * 2 + 1, self.min_v[k]);\n        }\n\n        if self.max_v[k] < self.max_v[k\
    \ * 2 + 2] {\n            self.update_node_max(k * 2 + 2, self.max_v[k]);\n  \
    \      }\n        if self.min_v[k * 2 + 2] < self.min_v[k] {\n            self.update_node_min(k\
    \ * 2 + 2, self.min_v[k]);\n        }\n    }\n\n    fn update(&mut self, k: usize)\
    \ {\n        self.sum[k] = self.sum[k * 2 + 1] + self.sum[k * 2 + 2];\n\n    \
    \    if self.max_v[k * 2 + 1] < self.max_v[k * 2 + 2] {\n            self.max_v[k]\
    \ = self.max_v[k * 2 + 2];\n            self.max_c[k] = self.max_c[k * 2 + 2];\n\
    \            self.smax_v[k] = self.max_v[k * 2 + 1].max(self.smax_v[k * 2 + 2]);\n\
    \        } else if self.max_v[k * 2 + 1] > self.max_v[k * 2 + 2] {\n         \
    \   self.max_v[k] = self.max_v[k * 2 + 1];\n            self.max_c[k] = self.max_c[k\
    \ * 2 + 1];\n            self.smax_v[k] = self.smax_v[k * 2 + 1].max(self.max_v[k\
    \ * 2 + 2]);\n        } else {\n            self.max_v[k] = self.max_v[k * 2 +\
    \ 1];\n            self.max_c[k] = self.max_c[k * 2 + 1] + self.max_c[k * 2 +\
    \ 2];\n            self.smax_v[k] = self.smax_v[k * 2 + 1].max(self.smax_v[k *\
    \ 2 + 2]);\n        }\n\n        if self.min_v[k * 2 + 1] < self.min_v[k * 2 +\
    \ 2] {\n            self.min_v[k] = self.min_v[k * 2 + 1];\n            self.min_c[k]\
    \ = self.min_c[k * 2 + 1];\n            self.smin_v[k] = self.smin_v[k * 2 + 1].min(self.min_v[k\
    \ * 2 + 2]);\n        } else if self.min_v[k * 2 + 1] > self.min_v[k * 2 + 2]\
    \ {\n            self.min_v[k] = self.min_v[k * 2 + 2];\n            self.min_c[k]\
    \ = self.min_c[k * 2 + 2];\n            self.smin_v[k] = self.min_v[k * 2 + 1].min(self.smin_v[k\
    \ * 2 + 2]);\n        } else {\n            self.min_v[k] = self.min_v[k * 2 +\
    \ 1];\n            self.min_c[k] = self.min_c[k * 2 + 1] + self.min_c[k * 2 +\
    \ 2];\n            self.smin_v[k] = self.smin_v[k * 2 + 1].min(self.smin_v[k *\
    \ 2 + 2]);\n        }\n    }\n\n    fn chmin_(&mut self, x: T, a: usize, b: usize,\
    \ k: usize, l: usize, r: usize) {\n        if b <= l || r <= a || self.max_v[k]\
    \ <= x {\n            return;\n        }\n        if a <= l && r <= b && self.smax_v[k]\
    \ < x {\n            self.update_node_max(k, x);\n            return;\n      \
    \  }\n\n        self.push(k);\n        self.chmin_(x, a, b, k * 2 + 1, l, (l +\
    \ r) / 2);\n        self.chmin_(x, a, b, k * 2 + 2, (l + r) / 2, r);\n       \
    \ self.update(k);\n    }\n\n    fn chmax_(&mut self, x: T, a: usize, b: usize,\
    \ k: usize, l: usize, r: usize) {\n        if b <= l || r <= a || x <= self.min_v[k]\
    \ {\n            return;\n        }\n        if a <= l && r <= b && x < self.smin_v[k]\
    \ {\n            self.update_node_min(k, x);\n            return;\n        }\n\
    \n        self.push(k);\n        self.chmax_(x, a, b, k * 2 + 1, l, (l + r) /\
    \ 2);\n        self.chmax_(x, a, b, k * 2 + 2, (l + r) / 2, r);\n        self.update(k);\n\
    \    }\n\n    fn add_(&mut self, x: T, a: usize, b: usize, k: usize, l: usize,\
    \ r: usize) {\n        if b <= l || r <= a {\n            return;\n        }\n\
    \        if a <= l && r <= b {\n            self.add_all(k, x);\n            return;\n\
    \        }\n\n        self.push(k);\n        self.add_(x, a, b, k * 2 + 1, l,\
    \ (l + r) / 2);\n        self.add_(x, a, b, k * 2 + 2, (l + r) / 2, r);\n    \
    \    self.update(k);\n    }\n\n    fn set_(&mut self, x: T, a: usize, b: usize,\
    \ k: usize, l: usize, r: usize) {\n        if b <= l || r <= a {\n           \
    \ return;\n        }\n        if a <= l && r <= b {\n            self.set_all(k,\
    \ x);\n            return;\n        }\n\n        self.push(k);\n        self.set_(x,\
    \ a, b, k * 2 + 1, l, (l + r) / 2);\n        self.set_(x, a, b, k * 2 + 2, (l\
    \ + r) / 2, r);\n        self.update(k);\n    }\n\n    fn max_(&mut self, a: usize,\
    \ b: usize, k: usize, l: usize, r: usize) -> T {\n        if b <= l || r <= a\
    \ {\n            return -INF;\n        }\n        if a <= l && r <= b {\n    \
    \        return self.max_v[k];\n        }\n\n        self.push(k);\n        let\
    \ lv = self.max_(a, b, k * 2 + 1, l, (l + r) / 2);\n        let rv = self.max_(a,\
    \ b, k * 2 + 2, (l + r) / 2, r);\n        lv.max(rv)\n    }\n\n    fn min_(&mut\
    \ self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {\n        if b\
    \ <= l || r <= a {\n            return INF;\n        }\n        if a <= l && r\
    \ <= b {\n            return self.min_v[k];\n        }\n\n        self.push(k);\n\
    \        let lv = self.min_(a, b, k * 2 + 1, l, (l + r) / 2);\n        let rv\
    \ = self.min_(a, b, k * 2 + 2, (l + r) / 2, r);\n        lv.min(rv)\n    }\n\n\
    \    fn sum_(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T\
    \ {\n        if b <= l || r <= a {\n            return 0;\n        }\n       \
    \ if a <= l && r <= b {\n            return self.sum[k];\n        }\n\n      \
    \  self.push(k);\n        let lv = self.sum_(a, b, k * 2 + 1, l, (l + r) / 2);\n\
    \        let rv = self.sum_(a, b, k * 2 + 2, (l + r) / 2, r);\n        lv + rv\n\
    \    }\n\n    fn range_to_pair<R>(&self, range: R) -> (usize, usize)\n    where\n\
    \        R: RangeBounds<usize>,\n    {\n        let l = match range.start_bound()\
    \ {\n            Bound::Included(&l) => l,\n            Bound::Excluded(&l) =>\
    \ l + 1,\n            Bound::Unbounded => 0,\n        };\n        let r = match\
    \ range.end_bound() {\n            Bound::Included(&r) => r + 1,\n           \
    \ Bound::Excluded(&r) => r,\n            Bound::Unbounded => self.n,\n       \
    \ };\n        (l, r)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/segtree-beats/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-21 11:20:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/range_chmin_chmax_add_range_sum/src/main.rs
documentation_of: crates/data-structure/segtree-beats/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/segtree-beats/src/lib.rs
- /library/crates/data-structure/segtree-beats/src/lib.rs.html
title: crates/data-structure/segtree-beats/src/lib.rs
---
