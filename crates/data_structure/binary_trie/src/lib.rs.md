---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/associative_array_bt/src/main.rs
    title: verify/library_checker/data_structure/associative_array_bt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/ordered_set/src/main.rs
    title: verify/library_checker/data_structure/ordered_set/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/set_xor_min/src/main.rs
    title: verify/library_checker/data_structure/set_xor_min/src/main.rs
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
  code: "// TODO: \u30DE\u30FC\u30B8\n\nuse std::ops::{Bound, RangeBounds};\n\nconst\
    \ W: u8 = usize::BITS as u8;\n\n#[derive(Clone)]\npub struct BinaryTrie {\n  \
    \  nodes: Vec<Node>,\n}\n\nimpl Default for BinaryTrie {\n    fn default() ->\
    \ Self {\n        Self::new()\n    }\n}\n\nimpl BinaryTrie {\n    pub fn new()\
    \ -> Self {\n        Self {\n            nodes: vec![Node::new(0)],\n        }\n\
    \    }\n\n    /// x \u3092 n \u500B\u8FFD\u52A0\u3059\u308B  \n    pub fn insert(&mut\
    \ self, mut x: usize, n: usize) {\n        if n == 0 {\n            return;\n\
    \        }\n\n        let mut v = 0;\n        let mut d = W;\n        while {\n\
    \            self.nodes[v].cnt += n;\n            d != 0\n        } {\n      \
    \      x &= !0 >> (W - d);\n            let c = x >> (d - 1) & 1;\n          \
    \  if let Some(link) = self.nodes[v].link[c] {\n                let common_prefix\
    \ = (W - (x ^ link.seq).leading_zeros() as u8).max(link.d);\n                if\
    \ common_prefix == link.d {\n                    v = link.ch as _;\n         \
    \           d = common_prefix;\n                } else {\n                   \
    \ let u = link.ch as usize;\n                    let w = self.new_node(self.nodes[u].cnt);\n\
    \                    let nx = x & !((1 << common_prefix) - 1);\n             \
    \       self.nodes[v].link[c] = Some(Link::new(w, nx, common_prefix));\n     \
    \               let x2 = link.seq & (!0 >> (W - common_prefix));\n           \
    \         let c2 = link.seq >> (common_prefix - 1) & 1;\n                    self.nodes[w].cnt\
    \ = self.nodes[u].cnt;\n                    self.nodes[w].link[c2] = Some(Link::new(u,\
    \ x2, link.d));\n                    v = w;\n                    d = common_prefix;\n\
    \                }\n            } else {\n                let u = self.new_node(0);\n\
    \                self.nodes[v].link[c] = Some(Link::new(u, x, 0));\n         \
    \       v = u;\n                d = 0;\n            }\n        }\n    }\n\n  \
    \  /// x \u3092 n \u500B\u524A\u9664\u3059\u308B  \n    /// n \u500B\u4EE5\u4E0A\
    \u5B58\u5728\u3057\u306A\u3044\u5834\u5408\u306E\u52D5\u4F5C\u306F\u672A\u5B9A\
    \u7FA9\n    pub fn remove(&mut self, mut x: usize, n: usize) {\n        if n ==\
    \ 0 {\n            return;\n        }\n\n        let mut v = 0;\n        let mut\
    \ d = W;\n        while {\n            self.nodes[v].cnt -= n;\n            d\
    \ != 0\n        } {\n            x &= !0 >> (W - d);\n            let c = x >>\
    \ (d - 1) & 1;\n            let link = self.nodes[v].link[c].unwrap();\n     \
    \       v = link.ch as _;\n            d = link.d;\n        }\n    }\n\n    ///\
    \ x \u306E\u500B\u6570\u3092\u8FD4\u3059  \n    pub fn count(&self, mut x: usize)\
    \ -> usize {\n        let mut v = 0;\n        let mut d = W;\n        while d\
    \ > 0 {\n            x &= !0 >> (W - d);\n            let c = x >> (d - 1) & 1;\n\
    \            if let Some(link) = self.nodes[v].link[c] {\n                let\
    \ common_prefix = (W - (x ^ link.seq).leading_zeros() as u8).max(link.d);\n  \
    \              if common_prefix == link.d {\n                    v = link.ch as\
    \ _;\n                    d = link.d;\n                } else {\n            \
    \        return 0;\n                }\n            } else {\n                return\
    \ 0;\n            }\n        }\n        self.nodes[v].cnt\n    }\n\n    /// \u591A\
    \u91CD\u96C6\u5408\u306B\u542B\u307E\u308C\u308B\u8981\u7D20\u6570\u3092\u8FD4\
    \u3059\n    pub fn count_all(&self) -> usize {\n        self.nodes[0].cnt\n  \
    \  }\n\n    /// min_{x \u2208 S} (x ^ xor) \u3092\u8FD4\u3059\n    pub fn min(&self,\
    \ xor: usize) -> Option<usize> {\n        if self.nodes[0].cnt == 0 {\n      \
    \      return None;\n        }\n        let mut v = 0;\n        let mut d = W;\n\
    \        let mut res = xor;\n        while d > 0 {\n            let c = xor >>\
    \ (d - 1) & 1;\n            let link = self.nodes[v].link[c]\n               \
    \ .filter(|link| self.nodes[link.ch as usize].cnt > 0)\n                .or(self.nodes[v].link[c\
    \ ^ 1])\n                .unwrap();\n            res ^= link.seq;\n          \
    \  v = link.ch as _;\n            d = link.d;\n        }\n        Some(res)\n\
    \    }\n\n    /// max_{x \u2208 S} (x ^ xor) \u3092\u8FD4\u3059\n    pub fn max(&self,\
    \ xor: usize) -> Option<usize> {\n        self.min(xor ^ !0).map(|x| x ^ !0)\n\
    \    }\n\n    /// {y \u2208 S | y ^ xor \u2208 range} \u306E\u500B\u6570\u3092\
    \u8FD4\u3059\n    pub fn count_range(&self, range: impl RangeBounds<usize>, xor:\
    \ usize) -> usize {\n        let l = match range.start_bound() {\n           \
    \ Bound::Unbounded => self.count_all(),\n            Bound::Included(&start) =>\
    \ self.count_geq(start, xor),\n            Bound::Excluded(&start) => self.count_geq(start\
    \ + 1, xor),\n        };\n        let r = match range.end_bound() {\n        \
    \    Bound::Unbounded => 0,\n            Bound::Included(&end) => end.checked_add(1).map_or(0,\
    \ |end| self.count_geq(end, xor)),\n            Bound::Excluded(&end) => self.count_geq(end,\
    \ xor),\n        };\n        l - r\n    }\n\n    /// x ^ xor \u3067 k \u756A\u76EE\
    \u306B\u5C0F\u3055\u3044\u5024\u3092\u8FD4\u3059\n    pub fn kth_smallest(&self,\
    \ mut k: usize, xor: usize) -> Option<usize> {\n        if k >= self.count_all()\
    \ {\n            return None;\n        }\n        let mut v = 0;\n        let\
    \ mut d = W;\n        let mut res = xor;\n        while d > 0 {\n            let\
    \ c = xor >> (d - 1) & 1;\n            let link = self.nodes[v].link[c]\n    \
    \            .filter(|link| {\n                    let cnt = self.nodes[link.ch\
    \ as usize].cnt;\n                    if k >= cnt {\n                        k\
    \ -= cnt;\n                        false\n                    } else {\n     \
    \                   true\n                    }\n                })\n        \
    \        .or(self.nodes[v].link[c ^ 1])\n                .unwrap();\n        \
    \    res ^= link.seq;\n            v = link.ch as _;\n            d = link.d;\n\
    \        }\n        Some(res)\n    }\n\n    /// x ^ xor \u3067 k \u756A\u76EE\u306B\
    \u5927\u304D\u3044\u5024\u3092\u8FD4\u3059\n    pub fn kth_largest(&self, k: usize,\
    \ xor: usize) -> Option<usize> {\n        if k >= self.count_all() {\n       \
    \     return None;\n        }\n        self.kth_smallest(self.count_all() - k\
    \ - 1, xor)\n    }\n\n    fn new_node(&mut self, cnt: usize) -> usize {\n    \
    \    self.nodes.push(Node::new(cnt));\n        self.nodes.len() - 1\n    }\n\n\
    \    /// {y \u2208 S | y ^ xor \u2265 x} \u306E\u500B\u6570\u3092\u8FD4\u3059\n\
    \    fn count_geq(&self, mut x: usize, mut xor: usize) -> usize {\n        let\
    \ mut v = 0;\n        let mut d = W;\n        let mut res = 0;\n        while\
    \ d > 0 {\n            x &= !0 >> (W - d);\n            xor &= !0 >> (W - d);\n\
    \            let c = (x ^ xor) >> (d - 1) & 1;\n            if x >> (d - 1) &\
    \ 1 == 0 {\n                if let Some(link) = self.nodes[v].link[c ^ 1] {\n\
    \                    res += self.nodes[link.ch as usize].cnt;\n              \
    \  }\n            }\n            if let Some(link) = self.nodes[v].link[c] {\n\
    \                let mask = !((1 << link.d) - 1);\n                if (x ^ xor)\
    \ & mask == link.seq {\n                    v = link.ch as usize;\n          \
    \          d = link.d;\n                } else {\n                    if x < link.seq\
    \ ^ xor {\n                        res += self.nodes[link.ch as usize].cnt;\n\
    \                    }\n                    return res;\n                }\n \
    \           } else {\n                return res;\n            }\n        }\n\
    \        res += self.nodes[v].cnt;\n        res\n    }\n}\n\n#[derive(Clone, Copy,\
    \ Debug)]\nstruct Node {\n    cnt: usize,\n    link: [Option<Link>; 2],\n}\n\n\
    impl Node {\n    fn new(cnt: usize) -> Self {\n        Self {\n            cnt,\n\
    \            link: [None; 2],\n        }\n    }\n}\n\n#[derive(Clone, Copy, Debug)]\n\
    struct Link {\n    ch: u32,\n    seq: usize,\n    d: u8,\n    _f: bool, // \u3053\
    \u308C\u3092\u3064\u3051\u308B\u3068 niche optimization \u304C\u52B9\u3044\u3066\
    \u30E1\u30E2\u30EA\u304C\u5C0F\u3055\u304F\u306A\u308B\n}\n\nimpl Link {\n   \
    \ fn new(ch: usize, seq: usize, d: u8) -> Self {\n        Self {\n           \
    \ ch: ch as u32,\n            seq,\n            d,\n            _f: false,\n \
    \       }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/binary_trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/ordered_set/src/main.rs
  - verify/library_checker/data_structure/associative_array_bt/src/main.rs
  - verify/library_checker/data_structure/set_xor_min/src/main.rs
documentation_of: crates/data_structure/binary_trie/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/binary_trie/src/lib.rs
- /library/crates/data_structure/binary_trie/src/lib.rs.html
title: crates/data_structure/binary_trie/src/lib.rs
---
