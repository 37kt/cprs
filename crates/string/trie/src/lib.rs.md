---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/trie_yuki1269/src/main.rs
    title: verify/trie_yuki1269/src/main.rs
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
  code: "/// Trie + Aho-Corasick  \n/// \u6587\u5B57\u306F \\[0, SIGMA) \u306E\u6574\
    \u6570\u3068\u3059\u308B\u3002\n#[derive(Clone)]\npub struct Trie<const SIGMA:\
    \ usize = 26> {\n    children: Vec<[usize; SIGMA]>, // Trie \u306E\u5B50\u30CE\
    \u30FC\u30C9\n    parent: Vec<usize>,            // Trie \u306E\u89AA\u30CE\u30FC\
    \u30C9\n\n    built: bool, // suffix link \u304C\u69CB\u7BC9\u3055\u308C\u3066\
    \u3044\u308B\u304B\n\n    suffix_link: Vec<usize>,   // Aho-Corasick \u306E suffix\
    \ link\n    next: Vec<[usize; SIGMA]>, // Aho-Corasick \u306E next\n\n    words:\
    \ Vec<usize>,     // \u5358\u8A9E\u306B\u5BFE\u5FDC\u3059\u308B index\n    bfs_order:\
    \ Vec<usize>, // node \u306E BFS \u9806\n    count: Vec<usize>,     // node \u306B\
    \u5BFE\u5FDC\u3059\u308B\u6587\u5B57\u5217\u306E suffix \u304C\u3044\u304F\u3064\
    \u306E\u5358\u8A9E\u3068\u4E00\u81F4\u3059\u308B\u304B\n}\n\nimpl<const SIGMA:\
    \ usize> Trie<SIGMA> {\n    /// \u7A7A\u306E Trie \u3092\u4F5C\u6210\u3059\u308B\
    \u3002\n    pub fn new() -> Self {\n        let mut res = Self {\n           \
    \ children: vec![],\n            parent: vec![],\n            built: false,\n\
    \            suffix_link: vec![],\n            next: vec![],\n            words:\
    \ vec![],\n            bfs_order: vec![],\n            count: vec![],\n      \
    \  };\n        res.add_node();\n        res\n    }\n\n    /// \u5358\u8A9E s \u3092\
    \u8FFD\u52A0\u3059\u308B\u3002  \n    /// \u8FFD\u52A0\u3055\u308C\u305F\u5358\
    \u8A9E\u304C\u5BFE\u5FDC\u3059\u308B\u30CE\u30FC\u30C9\u3092\u8FD4\u3059\u3002\
    \  \n    /// build \u304C\u547C\u3070\u308C\u3066\u3044\u306A\u3044\u5FC5\u8981\
    \u304C\u3042\u308B\u3002\n    pub fn add(&mut self, s: &[usize]) -> usize {\n\
    \        assert!(!self.built);\n\n        let mut v = 0;\n        for &c in s\
    \ {\n            v = self.add_child(v, c);\n        }\n        self.words.push(v);\n\
    \        self.count[v] += 1;\n        v\n    }\n\n    /// suffix link \u3092\u69CB\
    \u7BC9\u3059\u308B\u3002  \n    /// build \u304C\u547C\u3070\u308C\u3066\u3044\
    \u306A\u3044\u5FC5\u8981\u304C\u3042\u308B\u3002\n    pub fn build(&mut self)\
    \ {\n        assert!(!self.built);\n\n        let mut p = 0;\n        let mut\
    \ q = 0;\n        self.bfs_order.resize(self.len(), !0);\n        self.bfs_order[q]\
    \ = 0;\n        q += 1;\n        self.next[0] = [0; SIGMA];\n        while p <\
    \ q {\n            let v = self.bfs_order[p];\n            p += 1;\n         \
    \   if v != 0 {\n                self.next[v] = self.next[self.suffix_link[v]];\n\
    \            }\n            for c in 0..SIGMA {\n                let Some(u) =\
    \ self.child(v, c) else { continue; };\n                self.suffix_link[u] =\
    \ self.next[v][c];\n                self.next[v][c] = u;\n                self.bfs_order[q]\
    \ = u;\n                q += 1;\n            }\n        }\n\n        for &v in\
    \ &self.bfs_order {\n            if v != 0 {\n                self.count[v] +=\
    \ self.count[self.suffix_link[v]];\n            }\n        }\n\n        self.built\
    \ = true;\n    }\n\n    /// \u30CE\u30FC\u30C9\u306E\u6570\u3092\u8FD4\u3059\u3002\
    \n    pub fn len(&self) -> usize {\n        self.children.len()\n    }\n\n   \
    \ /// \u30CE\u30FC\u30C9 v \u306E\u6587\u5B57 c \u306B\u5BFE\u5FDC\u3059\u308B\
    \u5B50\u30CE\u30FC\u30C9\u3092\u8FD4\u3059\u3002  \n    /// \u5B58\u5728\u3057\
    \u306A\u3044\u5834\u5408\u306F None \u3092\u8FD4\u3059\u3002\n    pub fn child(&self,\
    \ v: usize, c: usize) -> Option<usize> {\n        let res = self.children[v][c];\n\
    \        if res == !0 {\n            None\n        } else {\n            Some(res)\n\
    \        }\n    }\n\n    /// \u30CE\u30FC\u30C9 v \u306E\u89AA\u30CE\u30FC\u30C9\
    \u3092\u8FD4\u3059\u3002  \n    /// \u6839\u30CE\u30FC\u30C9\u306E\u5834\u5408\
    \u306F None \u3092\u8FD4\u3059\u3002\n    pub fn parent(&self, v: usize) -> Option<usize>\
    \ {\n        let res = self.parent[v];\n        if res == !0 {\n            None\n\
    \        } else {\n            Some(res)\n        }\n    }\n\n    /// \u30CE\u30FC\
    \u30C9 v \u306E suffix link \u306E\u5148\u306E\u30CE\u30FC\u30C9\u3092\u8FD4\u3059\
    \u3002  \n    /// \u6839\u30CE\u30FC\u30C9\u306E\u5834\u5408\u306F None \u3092\
    \u8FD4\u3059\u3002  \n    /// build \u304C\u547C\u3070\u308C\u3066\u3044\u308B\
    \u5FC5\u8981\u304C\u3042\u308B\u3002\n    pub fn suffix_link(&self, v: usize)\
    \ -> Option<usize> {\n        assert!(self.built);\n\n        let res = self.suffix_link[v];\n\
    \        if res == !0 {\n            None\n        } else {\n            Some(res)\n\
    \        }\n    }\n\n    /// \u30CE\u30FC\u30C9 v \u306B\u5BFE\u5FDC\u3059\u308B\
    \u6587\u5B57\u5217\u306B\u6587\u5B57 c \u3092\u8FFD\u52A0\u3057\u305F\u6587\u5B57\
    \u5217\u3068\n    /// suffix \u304C\u4E00\u81F4\u3059\u308B\u30CE\u30FC\u30C9\u306E\
    \u3046\u3061\u6700\u9577\u306E\u3082\u306E\u3092\u8FD4\u3059\u3002  \n    ///\
    \ build \u304C\u547C\u3070\u308C\u3066\u3044\u308B\u5FC5\u8981\u304C\u3042\u308B\
    \u3002\n    pub fn next(&self, v: usize, c: usize) -> usize {\n        assert!(self.built);\n\
    \        self.next[v][c]\n    }\n\n    /// \u8FFD\u52A0\u3055\u308C\u305F\u5358\
    \u8A9E\u304C\u5BFE\u5FDC\u3059\u308B\u30CE\u30FC\u30C9\u3092\u8FD4\u3059\u3002\
    \n    pub fn words(&self) -> &[usize] {\n        &self.words\n    }\n\n    ///\
    \ \u30CE\u30FC\u30C9\u306E BFS \u9806\u3092\u8FD4\u3059\u3002  \n    /// build\
    \ \u304C\u547C\u3070\u308C\u3066\u3044\u308B\u5FC5\u8981\u304C\u3042\u308B\u3002\
    \n    pub fn bfs_order(&self) -> &[usize] {\n        assert!(self.built);\n  \
    \      &self.bfs_order\n    }\n\n    /// \u30CE\u30FC\u30C9 v \u306B\u5BFE\u5FDC\
    \u3059\u308B\u6587\u5B57\u5217\u306E suffix \u304C\u3044\u304F\u3064\u306E\u5358\
    \u8A9E\u3068\u4E00\u81F4\u3059\u308B\u304B\u3092\u8FD4\u3059\u3002  \n    ///\
    \ build \u304C\u547C\u3070\u308C\u3066\u3044\u308B\u5FC5\u8981\u304C\u3042\u308B\
    \u3002\n    pub fn count(&self, v: usize) -> usize {\n        assert!(self.built);\n\
    \        self.count[v]\n    }\n\n    fn add_node(&mut self) -> usize {\n     \
    \   assert!(!self.built);\n\n        let n = self.children.len();\n        self.children.push([!0;\
    \ SIGMA]);\n        self.parent.push(!0);\n        self.suffix_link.push(!0);\n\
    \        self.next.push([!0; SIGMA]);\n        self.count.push(0);\n        n\n\
    \    }\n\n    fn add_child(&mut self, v: usize, c: usize) -> usize {\n       \
    \ if let Some(u) = self.child(v, c) {\n            u\n        } else {\n     \
    \       let u = self.add_node();\n            self.children[v][c] = u;\n     \
    \       self.parent[u] = v;\n            u\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-03 08:24:26+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/trie_yuki1269/src/main.rs
documentation_of: crates/string/trie/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/trie/src/lib.rs
- /library/crates/string/trie/src/lib.rs.html
title: crates/string/trie/src/lib.rs
---
