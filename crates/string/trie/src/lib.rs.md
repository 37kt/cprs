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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Clone)]\npub struct Trie<const SIGMA: usize = 26> {\n    children:\
    \ Vec<[usize; SIGMA]>,\n    parent: Vec<usize>,\n\n    built: bool,\n\n    suffix_link:\
    \ Vec<usize>,\n    next: Vec<[usize; SIGMA]>,\n\n    words: Vec<usize>,\n    bfs_order:\
    \ Vec<usize>,\n    count: Vec<usize>,\n}\n\nimpl<const SIGMA: usize> Default for\
    \ Trie<SIGMA> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n\
    impl<const SIGMA: usize> Trie<SIGMA> {\n    pub fn new() -> Self {\n        let\
    \ mut trie = Self {\n            children: vec![],\n            parent: vec![],\n\
    \            built: false,\n            suffix_link: vec![],\n            next:\
    \ vec![],\n            words: vec![],\n            bfs_order: vec![],\n      \
    \      count: vec![],\n        };\n        trie.add_node();\n        trie\n  \
    \  }\n\n    pub fn len(&self) -> usize {\n        self.children.len()\n    }\n\
    \n    pub fn is_empty(&self) -> bool {\n        false\n    }\n\n    pub fn add_word(&mut\
    \ self, s: &[usize]) -> usize {\n        assert!(!self.built);\n\n        let\
    \ mut v = 0;\n        for &c in s {\n            v = self.add_child(v, c);\n \
    \       }\n        self.words.push(v);\n        self.count[v] += 1;\n        v\n\
    \    }\n\n    pub fn build_suffix_links(&mut self) {\n        assert!(!self.built);\n\
    \n        let mut p = 0;\n        let mut q = 0;\n        self.bfs_order.resize(self.len(),\
    \ !0);\n        self.bfs_order[q] = 0;\n        q += 1;\n        self.next[0]\
    \ = [0; SIGMA];\n        while p < q {\n            let v = self.bfs_order[p];\n\
    \            p += 1;\n            if v != 0 {\n                self.next[v] =\
    \ self.next[self.suffix_link[v]];\n            }\n            for c in 0..SIGMA\
    \ {\n                let Some(u) = self.child(v, c) else {\n                 \
    \   continue;\n                };\n                self.suffix_link[u] = self.next[v][c];\n\
    \                self.next[v][c] = u;\n                self.bfs_order[q] = u;\n\
    \                q += 1;\n            }\n        }\n\n        for &v in &self.bfs_order\
    \ {\n            if v != 0 {\n                self.count[v] += self.count[self.suffix_link[v]];\n\
    \            }\n        }\n\n        self.built = true;\n    }\n\n    pub fn child(&self,\
    \ v: usize, c: usize) -> Option<usize> {\n        let u = self.children[v][c];\n\
    \        if u == !0 {\n            None\n        } else {\n            Some(u)\n\
    \        }\n    }\n\n    pub fn parent(&self, v: usize) -> Option<usize> {\n \
    \       let p = self.parent[v];\n        if p == !0 {\n            None\n    \
    \    } else {\n            Some(p)\n        }\n    }\n\n    pub fn suffix_link(&self,\
    \ v: usize) -> Option<usize> {\n        assert!(self.built);\n\n        let s\
    \ = self.suffix_link[v];\n        if s == !0 {\n            None\n        } else\
    \ {\n            Some(s)\n        }\n    }\n\n    pub fn next(&self, v: usize,\
    \ c: usize) -> usize {\n        assert!(self.built);\n        self.next[v][c]\n\
    \    }\n\n    pub fn words(&self) -> &[usize] {\n        &self.words\n    }\n\n\
    \    pub fn bfs_order(&self) -> &[usize] {\n        assert!(self.built);\n   \
    \     &self.bfs_order\n    }\n\n    pub fn count(&self, v: usize) -> usize {\n\
    \        assert!(self.built);\n        self.count[v]\n    }\n\n    pub fn find(&self,\
    \ s: &[usize]) -> Option<usize> {\n        let mut v = 0;\n        for &c in s\
    \ {\n            v = self.child(v, c)?;\n        }\n        Some(v)\n    }\n\n\
    \    pub fn path(&self, s: &[usize]) -> Option<Vec<usize>> {\n        let mut\
    \ path = vec![0];\n        let mut v = 0;\n        for &c in s {\n           \
    \ v = self.child(v, c)?;\n            path.push(v);\n        }\n        Some(path)\n\
    \    }\n\n    fn add_node(&mut self) -> usize {\n        assert!(!self.built);\n\
    \n        let n = self.len();\n        self.children.push([!0; SIGMA]);\n    \
    \    self.parent.push(!0);\n        self.suffix_link.push(!0);\n        self.next.push([!0;\
    \ SIGMA]);\n        self.count.push(0);\n        n\n    }\n\n    fn add_child(&mut\
    \ self, v: usize, c: usize) -> usize {\n        if let Some(u) = self.child(v,\
    \ c) {\n            u\n        } else {\n            let u = self.add_node();\n\
    \            self.children[v][c] = u;\n            self.parent[u] = v;\n     \
    \       u\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-01 23:54:34+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/string/trie/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/trie/src/lib.rs
- /library/crates/string/trie/src/lib.rs.html
title: crates/string/trie/src/lib.rs
---
