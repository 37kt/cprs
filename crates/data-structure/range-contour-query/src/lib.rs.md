---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    cmp::Reverse,\n    collections::{BinaryHeap, VecDeque},\n\
    };\n\nuse graph::Graph;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    \n#[derive(Clone)]\npub struct RangeContourQuery {\n    n: usize,\n    sz: Vec<usize>,\n\
    \    ctr: Vec<usize>,\n    pub seq: Vec<Vec<usize>>,\n    sep: Vec<Vec<usize>>,\n\
    \    head: Vec<usize>,\n    tail: Vec<usize>,\n    link: Vec<usize>,\n    cur:\
    \ usize,\n    par: Vec<usize>,\n    ch: Vec<[usize; 2]>,\n    cv: Vec<usize>,\n\
    \    pos: Vec<Vec<(usize, usize)>>,\n    hld: HeavyLightDecomposition,\n}\n\n\
    impl RangeContourQuery {\n    pub fn new(g: &Graph<(), ()>) -> Self {\n      \
    \  let mut h = vec![vec![]; g.len()];\n        for v in 0..g.len() {\n       \
    \     for &(u, _) in &g[v] {\n                h[v].push(u);\n            }\n \
    \       }\n\n        let n = g.len();\n        let mut rcq = Self {\n        \
    \    n,\n            sz: vec![0; n * 3],\n            ctr: vec![!0; n * 3],\n\
    \            seq: vec![vec![]; n * 3],\n            sep: vec![vec![]; n * 3],\n\
    \            head: vec![!0; n * 3],\n            tail: vec![!0; n * 3],\n    \
    \        link: vec![!0; n * 3],\n            cur: n,\n            par: vec![!0;\
    \ n * 3],\n            ch: vec![[!0; 2]; n * 3],\n            cv: vec![!0; n *\
    \ 3],\n            pos: vec![vec![]; n],\n            hld: HeavyLightDecomposition::new(&g),\n\
    \        };\n        for i in 0..n {\n            rcq.head[i] = i;\n         \
    \   rcq.tail[i] = i;\n            rcq.cv[i] = i;\n        }\n        rcq.rec(0,\
    \ n, &mut h);\n        rcq.sz.truncate(rcq.cur);\n        rcq.ctr.truncate(rcq.cur);\n\
    \        rcq.seq.truncate(rcq.cur);\n        rcq.sep.truncate(rcq.cur);\n    \
    \    rcq.head.truncate(rcq.cur);\n        rcq.tail.truncate(rcq.cur);\n      \
    \  rcq.link.truncate(rcq.cur);\n        rcq.par.truncate(rcq.cur);\n        rcq.ch.truncate(rcq.cur);\n\
    \        rcq.cv.truncate(rcq.cur);\n        for i in 0..rcq.cur {\n          \
    \  for j in 0..rcq.seq[i].len() {\n                rcq.pos[rcq.seq[i][j]].push((i,\
    \ j));\n            }\n        }\n        rcq\n    }\n\n    fn search_centroid(&mut\
    \ self, v: usize, p: usize, n: usize, g: &[Vec<usize>], c: &mut usize) {\n   \
    \     self.sz[v] = 1;\n        for &u in &g[v] {\n            if u == p {\n  \
    \              continue;\n            }\n            self.search_centroid(u, v,\
    \ n, g, c);\n            if u == *c {\n                self.sz[v] = n - self.sz[*c];\n\
    \                break;\n            }\n            self.sz[v] += self.sz[u];\n\
    \        }\n        if *c == !0 && self.sz[v] * 2 > n {\n            *c = v;\n\
    \        }\n    }\n\n    fn build_seq(&mut self, h: usize, w: usize, c: usize,\
    \ g: &[Vec<usize>]) {\n        if c < self.n {\n            self.seq[w].push(c);\n\
    \            self.sep[w] = vec![0, 1];\n        }\n        let mut q = VecDeque::new();\n\
    \        let mut v = h;\n        while v != !0 {\n            q.push_back((v,\
    \ !0, 1));\n            v = self.link[v];\n        }\n        while let Some((v,\
    \ p, d)) = q.pop_front() {\n            self.seq[w].push(v);\n            if self.sep[w].len()\
    \ <= d + 1 {\n                self.sep[w].resize(d + 2, 0);\n            }\n \
    \           self.sep[w][d + 1] += 1;\n            for &u in &g[v] {\n        \
    \        if u == p {\n                    continue;\n                }\n     \
    \           q.push_back((u, v, d + 1));\n            }\n        }\n        for\
    \ i in 1..self.sep[w].len() {\n            self.sep[w][i] += self.sep[w][i - 1];\n\
    \        }\n    }\n\n    fn rec(&mut self, r: usize, n: usize, g: &mut [Vec<usize>])\
    \ -> usize {\n        let mut c = !0;\n        self.search_centroid(r, !0, n,\
    \ g, &mut c);\n\n        for i in 0..g[c].len() {\n            let u = g[c][i];\n\
    \            let tmp = self.sz[u];\n            g[u].retain(|&w| w != c);\n  \
    \          self.ctr[u] = self.rec(u, tmp, g);\n            self.sz[u] = tmp;\n\
    \        }\n\n        if g[c].len() == 0 {\n            self.build_seq(!0, c,\
    \ c, g);\n        } else if g[c].len() == 1 {\n            self.build_seq(self.head[g[c][0]],\
    \ c, c, g);\n            self.par[self.ctr[g[c][0]]] = c;\n            self.ch[c][0]\
    \ = self.ctr[g[c][0]];\n        } else {\n            let mut pq = BinaryHeap::new();\n\
    \            for &u in &g[c] {\n                self.link[u] = !0;\n         \
    \       let new = self.cur;\n                self.cur += 1;\n                self.build_seq(self.head[u],\
    \ new, !0, g);\n                self.sz[new] = self.sz[u];\n                self.ctr[new]\
    \ = new;\n                self.head[new] = self.head[u];\n                self.tail[new]\
    \ = self.tail[u];\n                self.ch[new][0] = self.ctr[u];\n          \
    \      self.par[self.ctr[u]] = new;\n                self.cv[new] = c;\n     \
    \           pq.push(Reverse((self.sz[new], new)));\n            }\n          \
    \  while pq.len() >= 2 {\n                let Reverse((_, u)) = pq.pop().unwrap();\n\
    \                let Reverse((_, v)) = pq.pop().unwrap();\n                if\
    \ pq.is_empty() {\n                    self.link[self.tail[u]] = self.head[v];\n\
    \                    self.tail[u] = self.tail[v];\n                    self.ch[c]\
    \ = [self.ctr[u], self.ctr[v]];\n                    self.par[self.ctr[u]] = c;\n\
    \                    self.par[self.ctr[v]] = c;\n                    self.build_seq(self.head[u],\
    \ c, c, g);\n                } else {\n                    let new = self.cur;\n\
    \                    self.cur += 1;\n                    self.sz[new] = self.sz[u]\
    \ + self.sz[v];\n                    self.ch[new] = [self.ctr[u], self.ctr[v]];\n\
    \                    self.par[self.ctr[u]] = new;\n                    self.par[self.ctr[v]]\
    \ = new;\n                    self.ctr[new] = new;\n                    self.head[new]\
    \ = self.head[u];\n                    self.tail[new] = self.tail[v];\n      \
    \              self.link[self.tail[u]] = self.head[v];\n                    self.cv[new]\
    \ = c;\n                    self.build_seq(self.head[new], new, !0, g);\n    \
    \                pq.push(Reverse((self.sz[new], new)));\n                }\n \
    \           }\n        }\n\n        for i in 0..g[c].len() {\n            let\
    \ u = g[c][i];\n            g[u].push(c);\n        }\n\n        c\n    }\n\n \
    \   pub fn point(&self, v: usize) -> Vec<(usize, usize)> {\n        self.pos[v].clone()\n\
    \    }\n\n    pub fn range(&self, mut v: usize, l: usize, r: usize) -> Vec<(usize,\
    \ usize, usize)> {\n        let mut res = vec![];\n        if l >= r {\n     \
    \       return res;\n        }\n        let f = |v: usize, i: usize| self.sep[v][i.min(self.sep[v].len()\
    \ - 1)];\n        res.push((v, f(v, l), f(v, r)));\n        let vv = v;\n    \
    \    while self.par[v] != !0 {\n            let p = self.par[v];\n           \
    \ let d = self.hld.dist(self.cv[vv], self.cv[p]);\n            if p < self.n &&\
    \ l <= d && d < r {\n                res.push((p, 0, 1));\n            }\n   \
    \         for &u in &self.ch[p] {\n                if u != v && u != !0 {\n  \
    \                  let d = self.hld.dist(self.cv[vv], self.cv[u]);\n         \
    \           let l = l.saturating_sub(d);\n                    let r = r.saturating_sub(d);\n\
    \                    res.push((u, f(u, l), f(u, r)));\n                }\n   \
    \         }\n            v = p;\n        }\n        res.retain(|&(_, l, r)| l\
    \ < r);\n        res\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/range-contour-query/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/range-contour-query/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/range-contour-query/src/lib.rs
- /library/crates/data-structure/range-contour-query/src/lib.rs.html
title: crates/data-structure/range-contour-query/src/lib.rs
---
