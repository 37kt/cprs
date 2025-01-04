---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/ordered_set_binary_trie/src/main.rs
    title: verify/ordered_set_binary_trie/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/set_xor_min/src/main.rs
    title: verify/set_xor_min/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\n/// Binary Trie  \n/// \u975E\u8CA0\
    \u6574\u6570\u306E\u591A\u91CD\u96C6\u5408\u3092\u7BA1\u7406\n#[derive(Clone)]\n\
    pub struct BinaryTrie<const B: usize> {\n    xor: usize,\n    nodes: Vec<Node>,\n\
    }\n\n#[derive(Clone, Copy)]\nstruct Node {\n    children: [usize; 2],\n    count:\
    \ usize,\n}\n\nimpl Node {\n    fn new() -> Self {\n        Self {\n         \
    \   children: [!0; 2],\n            count: 0,\n        }\n    }\n}\n\nimpl<const\
    \ B: usize> BinaryTrie<B> {\n    /// \u7A7A\u306E\u96C6\u5408\u3092\u4F5C\u6210\
    \n    pub fn new() -> Self {\n        assert!(B < 64, \"B \u306F 64 \u672A\u6E80\
    \u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308A\u307E\u3059\");\n        Self\
    \ {\n            xor: 0,\n            nodes: vec![Node::new()],\n        }\n \
    \   }\n\n    /// \u96C6\u5408\u306E\u8981\u7D20\u6570\u3092\u8FD4\u3059\n    pub\
    \ fn len(&self) -> usize {\n        self.nodes[0].count\n    }\n\n    /// \u96C6\
    \u5408\u304C\u7A7A\u304B\u3069\u3046\u304B\u3092\u8FD4\u3059\n    pub fn is_empty(&self)\
    \ -> bool {\n        self.len() == 0\n    }\n\n    /// \u96C6\u5408\u306B\u8981\
    \u7D20 x \u3092 n \u500B\u8FFD\u52A0\n    pub fn insert(&mut self, x: usize, n:\
    \ usize) {\n        assert!(x < 1 << B, \"x \u306F 2^B \u672A\u6E80\u3067\u3042\
    \u308B\u5FC5\u8981\u304C\u3042\u308A\u307E\u3059\");\n        if n == 0 {\n  \
    \          return;\n        }\n        self.insert_(0, x, n, B);\n    }\n\n  \
    \  /// \u96C6\u5408\u304B\u3089\u8981\u7D20 x \u3092 n \u500B\u524A\u9664\n  \
    \  pub fn remove(&mut self, x: usize, n: usize) {\n        assert!(x < 1 << B,\
    \ \"x \u306F 2^B \u672A\u6E80\u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308A\u307E\
    \u3059\");\n        if n == 0 {\n            return;\n        }\n        self.remove_(0,\
    \ x, n, B);\n    }\n\n    /// \u96C6\u5408\u306E\u3059\u3079\u3066\u306E\u8981\
    \u7D20\u306B\u5BFE\u3057\u3066 xor \u3092\u4F5C\u7528\u3055\u305B\u308B\n    pub\
    \ fn operate_xor(&mut self, x: usize) {\n        self.xor ^= x;\n    }\n\n   \
    \ /// \u96C6\u5408\u306E\u6700\u5C0F\u5024\u3092\u8FD4\u3059\n    pub fn min(&self)\
    \ -> Option<usize> {\n        if self.is_empty() {\n            return None;\n\
    \        }\n        Some(self.min_(0, B) ^ self.xor)\n    }\n\n    /// \u96C6\u5408\
    \u306E\u6700\u5927\u5024\u3092\u8FD4\u3059\n    pub fn max(&self) -> Option<usize>\
    \ {\n        if self.is_empty() {\n            return None;\n        }\n     \
    \   Some(self.max_(0, B) ^ self.xor)\n    }\n\n    /// \u96C6\u5408\u306B\u8981\
    \u7D20 x \u304C\u4F55\u500B\u3042\u308B\u304B\u3092\u8FD4\u3059\n    pub fn count(&self,\
    \ x: usize) -> usize {\n        assert!(x < 1 << B, \"x \u306F 2^B \u672A\u6E80\
    \u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308A\u307E\u3059\");\n        let\
    \ mut v = 0;\n        for b in (0..B).rev() {\n            let c = ((x ^ self.xor)\
    \ >> b) & 1;\n            let Some(u) = self.child(v, c) else {\n            \
    \    return 0;\n            };\n            v = u;\n        }\n        self.nodes[v].count\n\
    \    }\n\n    /// \u96C6\u5408\u306E `range` \u306B\u542B\u307E\u308C\u308B\u8981\
    \u7D20\u306E\u500B\u6570\u3092\u8FD4\u3059\n    pub fn frequency(&self, range:\
    \ impl RangeBounds<usize>) -> usize {\n        let (l, r) = range_to_pair(range,\
    \ 1 << B);\n        self.lower_bound(r) - self.lower_bound(l)\n    }\n\n    ///\
    \ \u96C6\u5408\u306E `range` \u306B\u542B\u307E\u308C\u308B k \u756A\u76EE\u306B\
    \u5C0F\u3055\u3044\u8981\u7D20\u3092\u8FD4\u3059\n    pub fn kth_smallest(&self,\
    \ range: impl RangeBounds<usize>, k: usize) -> Option<usize> {\n        let (l,\
    \ r) = range_to_pair(range, 1 << B);\n        let k = k + self.lower_bound(l);\n\
    \        if k >= self.len() {\n            return None;\n        }\n        let\
    \ res = self.kth_smallest_(0, k, B) ^ self.xor;\n        if res >= r {\n     \
    \       return None;\n        }\n        Some(res)\n    }\n\n    /// \u96C6\u5408\
    \u306E `range` \u306B\u542B\u307E\u308C\u308B k \u756A\u76EE\u306B\u5927\u304D\
    \u3044\u8981\u7D20\u3092\u8FD4\u3059\n    pub fn kth_largest(&self, range: impl\
    \ RangeBounds<usize>, k: usize) -> Option<usize> {\n        let (l, r) = range_to_pair(range,\
    \ 1 << B);\n        let m = self.frequency(l..r);\n        if k >= m {\n     \
    \       return None;\n        }\n        self.kth_smallest(l..r, m - k - 1)\n\
    \    }\n\n    /// \u96C6\u5408\u306E `x` \u672A\u6E80\u306E\u8981\u7D20\u306E\u500B\
    \u6570\u3092\u8FD4\u3059\n    pub fn lower_bound(&self, x: usize) -> usize {\n\
    \        if x >= 1 << B {\n            return self.len();\n        }\n       \
    \ self.lower_bound_(0, x, B)\n    }\n\n    /// \u96C6\u5408\u306E `x` \u4EE5\u4E0B\
    \u306E\u8981\u7D20\u306E\u500B\u6570\u3092\u8FD4\u3059\n    pub fn upper_bound(&self,\
    \ x: usize) -> usize {\n        self.lower_bound(x + 1)\n    }\n\n    fn insert_(&mut\
    \ self, v: usize, x: usize, n: usize, b: usize) {\n        self.nodes[v].count\
    \ += n;\n        if b == 0 {\n            return;\n        }\n        let c =\
    \ ((x ^ self.xor) >> b - 1) & 1;\n        let u = self.add_child(v, c);\n    \
    \    self.insert_(u, x, n, b - 1);\n        self.nodes[v].children[c] = u;\n \
    \   }\n\n    fn remove_(&mut self, v: usize, x: usize, n: usize, b: usize) {\n\
    \        assert!(\n            self.nodes[v].count >= n,\n            \"\u524A\
    \u9664\u3059\u308B\u8981\u7D20\u6570\u304C\u73FE\u5728\u306E\u8981\u7D20\u6570\
    \u3092\u8D85\u3048\u3066\u3044\u307E\u3059\"\n        );\n\n        self.nodes[v].count\
    \ -= n;\n        if b == 0 {\n            return;\n        }\n        let c =\
    \ ((x ^ self.xor) >> b - 1) & 1;\n        let u = self\n            .child(v,\
    \ c)\n            .expect(\"\u524A\u9664\u3059\u308B\u8981\u7D20\u6570\u304C\u73FE\
    \u5728\u306E\u8981\u7D20\u6570\u3092\u8D85\u3048\u3066\u3044\u307E\u3059\");\n\
    \        self.remove_(u, x, n, b - 1);\n    }\n\n    fn min_(&self, v: usize,\
    \ b: usize) -> usize {\n        if b == 0 {\n            return 0;\n        }\n\
    \        let mut c = self.xor >> (b - 1) & 1;\n        if self.child(v, c).map_or(0,\
    \ |u| self.nodes[u].count) == 0 {\n            c ^= 1;\n        }\n        self.min_(self.child(v,\
    \ c).unwrap(), b - 1) | (c << b - 1)\n    }\n\n    fn max_(&self, v: usize, b:\
    \ usize) -> usize {\n        if b == 0 {\n            return 0;\n        }\n \
    \       let mut c = (!self.xor) >> (b - 1) & 1;\n        if self.child(v, c).map_or(0,\
    \ |u| self.nodes[u].count) == 0 {\n            c ^= 1;\n        }\n        self.max_(self.child(v,\
    \ c).unwrap(), b - 1) | (c << b - 1)\n    }\n\n    fn kth_smallest_(&self, v:\
    \ usize, k: usize, b: usize) -> usize {\n        if b == 0 {\n            return\
    \ 0;\n        }\n        let c = (self.xor >> b - 1) & 1;\n        let n0 = self.child(v,\
    \ c).map_or(0, |u| self.nodes[u].count);\n        if k < n0 {\n            let\
    \ u = self.child(v, c).unwrap();\n            self.kth_smallest_(u, k, b - 1)\n\
    \        } else {\n            let u = self.child(v, c ^ 1).unwrap();\n      \
    \      self.kth_smallest_(u, k - n0, b - 1) | ((c ^ 1) << b - 1)\n        }\n\
    \    }\n\n    fn lower_bound_(&self, v: usize, x: usize, b: usize) -> usize {\n\
    \        if b == 0 {\n            return 0;\n        }\n        let mut res =\
    \ 0;\n        if x >> (b - 1) & 1 == 1 {\n            if let Some(u) = self.child(v,\
    \ self.xor >> (b - 1) & 1) {\n                res += self.nodes[u].count;\n  \
    \          }\n        }\n        let c = (self.xor ^ x) >> (b - 1) & 1;\n    \
    \    if let Some(u) = self.child(v, c) {\n            res += self.lower_bound_(u,\
    \ x, b - 1);\n        }\n        res\n    }\n\n    fn child(&self, v: usize, c:\
    \ usize) -> Option<usize> {\n        let u = self.nodes[v].children[c];\n    \
    \    if u == !0 {\n            None\n        } else {\n            Some(u)\n \
    \       }\n    }\n\n    fn add_node(&mut self) -> usize {\n        let n = self.nodes.len();\n\
    \        self.nodes.push(Node::new());\n        n\n    }\n\n    fn add_child(&mut\
    \ self, v: usize, c: usize) -> usize {\n        self.child(v, c).unwrap_or_else(||\
    \ self.add_node())\n    }\n}\n\nfn range_to_pair<R: RangeBounds<usize>>(range:\
    \ R, n: usize) -> (usize, usize) {\n    let l = match range.start_bound() {\n\
    \        Bound::Unbounded => 0,\n        Bound::Excluded(&l) => (l + 1).min(n),\n\
    \        Bound::Included(&l) => l.min(n),\n    };\n    let r = match range.end_bound()\
    \ {\n        Bound::Unbounded => n,\n        Bound::Excluded(&r) => r.min(n),\n\
    \        Bound::Included(&r) => (r + 1).min(n),\n    };\n    assert!(l <= r);\n\
    \    (l, r)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/binary-trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-04 07:13:06+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/ordered_set_binary_trie/src/main.rs
  - verify/set_xor_min/src/main.rs
documentation_of: crates/data-structure/binary-trie/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/binary-trie/src/lib.rs
- /library/crates/data-structure/binary-trie/src/lib.rs.html
title: crates/data-structure/binary-trie/src/lib.rs
---
