---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://eijirou-kyopro.hatenablog.com/entry/2024/02/01/115639
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// \u53C2\u8003: https://eijirou-kyopro.hatenablog.com/entry/2024/02/01/115639\n\
    \nuse std::{\n    cmp::{Ordering, Reverse},\n    collections::{BinaryHeap, HashMap,\
    \ HashSet},\n    hash::{BuildHasherDefault, Hasher},\n};\n\npub trait State {\n\
    \    type A: Action;\n\n    fn enumerate_actions(&self) -> Vec<Self::A>;\n   \
    \ fn apply_action(&mut self, action: &Self::A);\n    fn revert_action(&mut self,\
    \ action: &Self::A);\n\n    fn evaluate_current_state(&self) -> StateInfo {\n\
    \        unimplemented!()\n    }\n\n    fn evaluate_after_action(&mut self, action:\
    \ &Self::A) -> StateInfo {\n        self.apply_action(action);\n        let res\
    \ = self.evaluate_current_state();\n        self.revert_action(action);\n    \
    \    res\n    }\n}\n\npub trait Action: Clone + Default {\n    fn consumed_turns(&self)\
    \ -> usize;\n}\n\n#[derive(Clone, Copy, PartialEq, Eq)]\npub struct StateInfo\
    \ {\n    pub score: i32,\n    pub hash: u64,\n    pub valid: bool,\n}\n\n#[derive(Clone)]\n\
    struct Candidate<A: Action> {\n    action: A,\n    parent: u32,\n    score: i32,\n\
    \    hash: u64,\n    valid: bool,\n}\n\nimpl<A: Action> PartialEq for Candidate<A>\
    \ {\n    fn eq(&self, other: &Self) -> bool {\n        self.score == other.score\n\
    \    }\n}\n\nimpl<A: Action> Eq for Candidate<A> {}\n\nimpl<A: Action> PartialOrd\
    \ for Candidate<A> {\n    fn partial_cmp(&self, other: &Self) -> Option<Ordering>\
    \ {\n        Some(self.cmp(other))\n    }\n}\n\nimpl<A: Action> Ord for Candidate<A>\
    \ {\n    fn cmp(&self, other: &Self) -> Ordering {\n        other.score.cmp(&self.score)\n\
    \    }\n}\n\n// \u4E8C\u91CD\u9023\u9396\u6728\u306E\u30CE\u30FC\u30C9\n#[derive(Clone,\
    \ Default)]\nstruct Node<A: Action> {\n    action: A,\n    parent: u32,\n    child:\
    \ u32,       // \u9577\u7537\n    left: u32,        // \u5144\n    right: u32,\
    \       // \u5F1F\n    count_cands: u32, // \u5B50\u5019\u88DC\u306E Candidate\
    \ \u306E\u500B\u6570\n}\n\nstruct Pool<T> {\n    nodes: Vec<T>,\n    free: Vec<u32>,\n\
    }\n\nimpl<T> Pool<T> {\n    fn new(capacity: usize) -> Self {\n        Self {\n\
    \            nodes: Vec::with_capacity(capacity),\n            free: Vec::with_capacity(capacity),\n\
    \        }\n    }\n\n    fn push(&mut self, node: T) -> u32 {\n        if let\
    \ Some(i) = self.free.pop() {\n            self.nodes[i as usize] = node;\n  \
    \          i\n        } else {\n            let i = self.nodes.len() as u32;\n\
    \            self.nodes.push(node);\n            i\n        }\n    }\n\n    fn\
    \ remove(&mut self, i: u32) {\n        self.free.push(i);\n    }\n\n    fn get(&self,\
    \ i: u32) -> &T {\n        &self.nodes[i as usize]\n    }\n\n    fn get_mut(&mut\
    \ self, i: u32) -> &mut T {\n        &mut self.nodes[i as usize]\n    }\n}\n\n\
    #[derive(Clone)]\nstruct MinK<A: Action> {\n    size: usize,\n    pq: BinaryHeap<Candidate<A>>,\n\
    }\n\nimpl<A: Action> MinK<A> {\n    fn new(size: usize) -> Self {\n        Self\
    \ {\n            size,\n            pq: BinaryHeap::with_capacity(size),\n   \
    \     }\n    }\n\n    // \u524A\u9664\u3055\u308C\u308B\u5019\u88DC\u306E\u89AA\
    \u3092\u8FD4\u3059\n    fn push(&mut self, x: Candidate<A>) -> u32 {\n       \
    \ if self.pq.len() < self.size {\n            self.pq.push(x);\n            !0\n\
    \        } else if let Some(mut top) = self.pq.peek_mut() {\n            if top.score\
    \ < x.score {\n                let res = top.parent;\n                *top = x;\n\
    \                res\n            } else {\n                x.parent\n       \
    \     }\n        } else {\n            unreachable!()\n        }\n    }\n\n  \
    \  fn drain(&mut self) -> impl Iterator<Item = Candidate<A>> + '_ {\n        self.pq.drain()\n\
    \    }\n}\n\npub struct BeamSearch<S: State> {\n    max_turns: usize,\n    minimize_turn:\
    \ bool,\n\n    v: u32, // \u73FE\u5728\u306E\u30CE\u30FC\u30C9\n    turn: usize,\n\
    \    state: S,\n    nodes: Pool<Node<S::A>>,\n    root: u32,\n    best_valid_score:\
    \ i32,\n    best_node: u32,\n    dfs_stack: Vec<u32>,\n    candidates: Vec<MinK<S::A>>,\n\
    }\n\nimpl<S: State> BeamSearch<S> {\n    pub fn new(\n        initial_state: S,\n\
    \        max_turns: usize,\n        width: usize,\n        nodes_capacity: usize,\n\
    \        minimize_turn: bool,\n    ) -> Self {\n        let mut nodes = Pool::new(nodes_capacity);\n\
    \        let v = nodes.push(Node {\n            action: Default::default(),\n\
    \            parent: !0,\n            child: !0,\n            left: !0,\n    \
    \        right: !0,\n            count_cands: 0,\n        });\n        Self {\n\
    \            max_turns,\n            minimize_turn,\n            v,\n        \
    \    turn: 0,\n            state: initial_state,\n            nodes,\n       \
    \     root: v,\n            best_valid_score: i32::MIN,\n            best_node:\
    \ !0,\n            dfs_stack: Vec::with_capacity(nodes_capacity * 2),\n      \
    \      candidates: vec![MinK::new(width); max_turns + 1],\n        }\n    }\n\n\
    \    pub fn run(&mut self) -> Result<(Vec<S::A>, i32), &'static str> {\n     \
    \   let mut appeared = NopHashSet::default();\n\n        for turn in 0..=self.max_turns\
    \ {\n            let mut cands = self.candidates[turn].drain().collect::<Vec<_>>();\n\
    \            cands.sort_unstable_by_key(|c| Reverse(c.score));\n\n           \
    \ appeared.clear();\n            for c in cands {\n                let p = c.parent;\n\
    \                if appeared.insert(c.hash) {\n                    self.add_node(c);\n\
    \                }\n                self.nodes.get_mut(p).count_cands -= 1;\n\
    \                if self.removable(p) {\n                    self.remove_leaf(p);\n\
    \                }\n            }\n\n            if self.minimize_turn && self.best_node\
    \ != !0 {\n                break;\n            }\n            if turn == self.max_turns\
    \ {\n                break;\n            }\n\n            self.dfs(turn);\n  \
    \      }\n\n        let mut res = vec![];\n        let mut v = self.best_node;\n\
    \n        if v == !0 {\n            return Err(\"\u89E3\u304C\u898B\u3064\u304B\
    \u308A\u307E\u305B\u3093\u3067\u3057\u305F\");\n        }\n\n        loop {\n\
    \            let node = self.nodes.get(v);\n            let parent = node.parent;\n\
    \            if parent == !0 {\n                break;\n            }\n      \
    \      res.push(node.action.clone());\n            v = parent;\n        }\n\n\
    \        res.reverse();\n        Ok((res, self.best_valid_score))\n    }\n\n \
    \   // \u65B0\u3057\u3044\u30CE\u30FC\u30C9\u3092\u9577\u7537\u3068\u3057\u3066\
    \u8FFD\u52A0\u3059\u308B\n    fn add_node(&mut self, cand: Candidate<S::A>) ->\
    \ u32 {\n        let parent = cand.parent;\n        let sibling = self.nodes.get(parent).child;\n\
    \        let u = self.nodes.push(Node {\n            action: cand.action,\n  \
    \          parent,\n            child: !0,\n            left: !0,\n          \
    \  right: sibling,\n            count_cands: 0,\n        });\n        self.nodes.get_mut(parent).child\
    \ = u;\n        if sibling != !0 {\n            self.nodes.get_mut(sibling).left\
    \ = u;\n        }\n        if cand.valid && cand.score > self.best_valid_score\
    \ {\n            self.best_node = u;\n            self.best_valid_score = cand.score;\n\
    \        }\n        u\n    }\n\n    // \u73FE\u5728\u306E\u30BF\u30FC\u30F3\u6570\
    \u304C target_turn \u3067\u3042\u308B\u72B6\u614B\u305F\u3061\u306E\u5B50\u306E\
    \u5019\u88DC\u3092\u5217\u6319\u3059\u308B\u3002\n    // \u3064\u3044\u3067\u306B\
    \u3001\u9014\u4E2D\u3067\u898B\u3064\u3051\u305F\u4E0D\u8981\u306A\u30CE\u30FC\
    \u30C9\u3092\u524A\u9664\u3059\u308B\u3002\n    fn dfs(&mut self, target_turn:\
    \ usize) {\n        assert!(self.dfs_stack.is_empty());\n\n        self.update_root();\n\
    \        // self.v == self.root\n\n        if self.turn > target_turn {\n    \
    \        return;\n        }\n\n        if self.turn == target_turn {\n       \
    \     self.enumerate_candidates();\n            return;\n        }\n\n       \
    \ let mut u = self.child(self.v);\n\n        while u != !0 {\n            let\
    \ next_turn = self.turn + self.nodes.get(u).action.consumed_turns();\n       \
    \     if next_turn <= target_turn {\n                self.dfs_stack.push(u);\n\
    \            }\n            u = self.right(u);\n        }\n\n        let mut disused_nodes\
    \ = vec![];\n        while let Some(u) = self.dfs_stack.pop() {\n            if\
    \ u == !0 {\n                self.move_to_parent();\n                continue;\n\
    \            }\n\n            self.dfs_stack.push(!0);\n            self.move_to_child(u);\n\
    \            if self.turn == target_turn {\n                assert!(\n       \
    \             !self.has_child(self.v),\n                    \"v={}, child={}\"\
    ,\n                    self.v,\n                    self.child(self.v)\n     \
    \           );\n                self.enumerate_candidates();\n               \
    \ if self.removable(self.v) {\n                    disused_nodes.push(self.v);\n\
    \                }\n            } else if self.turn < target_turn {\n        \
    \        let mut u = self.child(self.v);\n                if u == !0 {\n     \
    \               if self.removable(self.v) {\n                        disused_nodes.push(self.v);\n\
    \                    }\n                } else {\n                    while u\
    \ != !0 {\n                        let next_turn = self.turn + self.nodes.get(u).action.consumed_turns();\n\
    \                        if next_turn <= target_turn {\n                     \
    \       self.dfs_stack.push(u);\n                        }\n                 \
    \       u = self.right(u);\n                    }\n                }\n       \
    \     } else {\n                unreachable!()\n            }\n        }\n   \
    \     assert!(self.v == self.root);\n\n        for v in disused_nodes {\n    \
    \        self.remove_leaf(v);\n        }\n    }\n\n    // \u73FE\u5728\u306E\u72B6\
    \u614B\u306E\u5B50\u306E\u5019\u88DC\u3092\u5217\u6319\u3059\u308B\n    fn enumerate_candidates(&mut\
    \ self) {\n        for action in self.state.enumerate_actions() {\n          \
    \  let next_turn = self.turn + action.consumed_turns();\n            if next_turn\
    \ > self.max_turns {\n                continue;\n            }\n\n           \
    \ let after_action = self.state.evaluate_after_action(&action);\n            self.nodes.get_mut(self.v).count_cands\
    \ += 1;\n            let p = self.candidates[next_turn].push(Candidate {\n   \
    \             action,\n                parent: self.v,\n                score:\
    \ after_action.score,\n                hash: after_action.hash,\n            \
    \    valid: after_action.valid,\n            });\n            if p != !0 {\n \
    \               self.nodes.get_mut(p).count_cands -= 1;\n            }\n     \
    \   }\n    }\n\n    // \u6839\u306E\u5B50\u304C\u3072\u3068\u3064\u3067\u3042\u308B\
    \u9593\u3001\u305D\u306E\u5B50\u3092\u6839\u306B\u3059\u308B\u3053\u3068\u3092\
    \u7E70\u308A\u8FD4\u3059\n    fn update_root(&mut self) {\n        assert!(self.v\
    \ == self.root);\n\n        while self.has_child(self.v) {\n            let child\
    \ = self.child(self.v);\n            if self.has_right(child) {\n            \
    \    break;\n            }\n            self.move_to_first_child();\n        }\n\
    \        self.root = self.v;\n    }\n\n    fn removable(&self, v: u32) -> bool\
    \ {\n        v != !0 && !self.has_child(v) && v != self.best_node && self.nodes.get(v).count_cands\
    \ == 0\n    }\n\n    // \u8449 v \u3092\u524A\u9664\u3059\u308B\u3002\u305D\u306E\
    \u7D50\u679C\u89AA\u304C\u8449\u306B\u306A\u308B\u5834\u5408\u306F\u3001\u305D\
    \u306E\u89AA\u3082\u524A\u9664\u3059\u308B\u3002\u3053\u308C\u3092\u7E70\u308A\
    \u8FD4\u3059\n    fn remove_leaf(&mut self, mut v: u32) {\n        assert!(!self.has_child(v));\n\
    \n        while self.removable(v) {\n            assert!(!self.has_child(v));\n\
    \n            let parent = self.parent(v);\n            let left = self.left(v);\n\
    \            let right = self.right(v);\n            self.nodes.remove(v);\n \
    \           if left != !0 {\n                self.nodes.get_mut(left).right =\
    \ right;\n                if right != !0 {\n                    self.nodes.get_mut(right).left\
    \ = left;\n                }\n                return;\n            } else {\n\
    \                assert!(parent != !0, \"\u6839\u3092\u524A\u9664\u3057\u3088\u3046\
    \u3068\u3057\u3066\u3044\u307E\u3059\");\n                self.nodes.get_mut(parent).child\
    \ = right;\n                if right != !0 {\n                    self.nodes.get_mut(right).left\
    \ = !0;\n                    return;\n                }\n                v = parent;\n\
    \            }\n        }\n    }\n\n    #[allow(unused)]\n    fn has_parent(&self,\
    \ v: u32) -> bool {\n        self.nodes.get(v).parent != !0\n    }\n\n    fn parent(&self,\
    \ v: u32) -> u32 {\n        self.nodes.get(v).parent\n    }\n\n    fn has_child(&self,\
    \ v: u32) -> bool {\n        self.nodes.get(v).child != !0\n    }\n\n    fn child(&self,\
    \ v: u32) -> u32 {\n        self.nodes.get(v).child\n    }\n\n    #[allow(unused)]\n\
    \    fn has_left(&self, v: u32) -> bool {\n        self.nodes.get(v).left != !0\n\
    \    }\n\n    fn left(&self, v: u32) -> u32 {\n        self.nodes.get(v).left\n\
    \    }\n\n    fn has_right(&self, v: u32) -> bool {\n        self.nodes.get(v).right\
    \ != !0\n    }\n\n    fn right(&self, v: u32) -> u32 {\n        self.nodes.get(v).right\n\
    \    }\n\n    // \u89AA\u306B\u79FB\u52D5\u3059\u308B\n    fn move_to_parent(&mut\
    \ self) {\n        let node = &self.nodes.get(self.v);\n        self.state.revert_action(&node.action);\n\
    \        self.turn -= node.action.consumed_turns();\n        self.v = node.parent;\n\
    \    }\n\n    // \u9577\u7537\u306B\u79FB\u52D5\u3059\u308B\n    fn move_to_first_child(&mut\
    \ self) {\n        let u = self.child(self.v);\n        self.move_to_child(u);\n\
    \    }\n\n    // \u5B50\u3067\u3042\u308B u \u306B\u79FB\u52D5\u3059\u308B\n \
    \   fn move_to_child(&mut self, u: u32) {\n        self.v = u;\n        let action\
    \ = &self.nodes.get(self.v).action;\n        self.state.apply_action(action);\n\
    \        self.turn += action.consumed_turns();\n    }\n\n    #[allow(unused)]\n\
    \    // \u5F1F\u306B\u79FB\u52D5\u3059\u308B\n    fn move_to_right(&mut self)\
    \ {\n        let u = self.right(self.v);\n        self.move_to_sibling(u);\n \
    \   }\n\n    // \u5144\u5F1F\u3067\u3042\u308B u \u306B\u79FB\u52D5\u3059\u308B\
    \n    fn move_to_sibling(&mut self, u: u32) {\n        self.move_to_parent();\n\
    \        self.move_to_child(u);\n    }\n}\n\n#[derive(Default)]\nstruct NopHasher\
    \ {\n    hash: u64,\n}\n\nimpl Hasher for NopHasher {\n    fn write(&mut self,\
    \ _: &[u8]) {\n        unimplemented!()\n    }\n\n    #[inline]\n    fn write_u64(&mut\
    \ self, n: u64) {\n        self.hash = n;\n    }\n\n    #[inline]\n    fn finish(&self)\
    \ -> u64 {\n        self.hash\n    }\n}\n\n#[allow(dead_code)]\ntype NopHashMap<K,\
    \ V> = HashMap<K, V, BuildHasherDefault<NopHasher>>;\ntype NopHashSet<K> = HashSet<K,\
    \ BuildHasherDefault<NopHasher>>;\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/heuristic/beam-search/src/lib.rs
  requiredBy: []
  timestamp: '2025-02-05 04:08:52+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam-search/src/lib.rs
- /library/crates/heuristic/beam-search/src/lib.rs.html
title: crates/heuristic/beam-search/src/lib.rs
---
