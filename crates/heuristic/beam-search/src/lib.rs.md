---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/timer/src/lib.rs
    title: crates/heuristic/timer/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://eijirou-kyopro.hatenablog.com/entry/2024/02/01/115639
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// \u53C2\u8003: https://eijirou-kyopro.hatenablog.com/entry/2024/02/01/115639\n\
    \nuse std::{\n    cmp::Reverse,\n    collections::{HashMap, HashSet},\n    hash::{BuildHasherDefault,\
    \ Hasher},\n};\n\nuse timer::Timer;\n\npub trait State {\n    type A: Action;\n\
    \n    fn score(&mut self) -> i32;\n    fn hash(&mut self) -> u64;\n    fn is_valid(&mut\
    \ self) -> bool;\n\n    fn enumerate_actions(&mut self) -> Vec<Self::A>;\n\n \
    \   fn apply_action(&mut self, action: &Self::A);\n    fn revert_action(&mut self,\
    \ action: &Self::A);\n}\n\npub trait Action: Clone + Default {\n    fn consumed_turns(&self)\
    \ -> usize;\n}\n\npub trait WidthManager {\n    fn width(&mut self, turn: usize,\
    \ elapsed_secs: f64) -> usize;\n}\n\n#[derive(Clone)]\nstruct Candidate<A: Action>\
    \ {\n    action: A,\n    parent: u32,\n    score: i32,\n    hash: u64,\n    valid:\
    \ bool,\n}\n\n// \u4E8C\u91CD\u9023\u9396\u6728\u306E\u30CE\u30FC\u30C9\n#[derive(Clone,\
    \ Default)]\nstruct Node<A: Action> {\n    action: A,\n    parent: u32,\n    child:\
    \ u32, // \u9577\u7537\n    left: u32,  // \u5144\n    right: u32, // \u5F1F\n\
    }\n\nstruct Pool<T> {\n    nodes: Vec<T>,\n    free: Vec<u32>,\n}\n\nimpl<T> Pool<T>\
    \ {\n    fn new(capacity: usize) -> Self {\n        Self {\n            nodes:\
    \ Vec::with_capacity(capacity),\n            free: Vec::with_capacity(capacity),\n\
    \        }\n    }\n\n    fn push(&mut self, node: T) -> u32 {\n        if let\
    \ Some(i) = self.free.pop() {\n            self.nodes[i as usize] = node;\n  \
    \          i\n        } else {\n            let i = self.nodes.len() as u32;\n\
    \            self.nodes.push(node);\n            i\n        }\n    }\n\n    fn\
    \ remove(&mut self, i: u32) {\n        self.free.push(i);\n    }\n\n    fn get(&self,\
    \ i: u32) -> &T {\n        &self.nodes[i as usize]\n    }\n\n    fn get_mut(&mut\
    \ self, i: u32) -> &mut T {\n        &mut self.nodes[i as usize]\n    }\n}\n\n\
    pub struct BeamSearch<S: State, W: WidthManager> {\n    max_turns: usize,\n  \
    \  width_manager: W,\n\n    v: u32, // \u73FE\u5728\u306E\u30CE\u30FC\u30C9\n\
    \    turn: usize,\n    state: S,\n    nodes: Pool<Node<S::A>>,\n    root: u32,\n\
    \    best_valid_score: i32,\n    best_node: u32,\n    dfs_stack: Vec<u32>,\n \
    \   candidates: Vec<Vec<Candidate<S::A>>>,\n}\n\nimpl<S: State, W: WidthManager>\
    \ BeamSearch<S, W> {\n    pub fn new(\n        initial_state: S,\n        max_turns:\
    \ usize,\n        width_manager: W,\n        nodes_capacity: usize,\n    ) ->\
    \ Self {\n        let mut nodes = Pool::new(nodes_capacity);\n        let v =\
    \ nodes.push(Node {\n            action: Default::default(),\n            parent:\
    \ !0,\n            child: !0,\n            left: !0,\n            right: !0,\n\
    \        });\n        Self {\n            max_turns,\n            width_manager,\n\
    \            v,\n            turn: 0,\n            state: initial_state,\n   \
    \         nodes,\n            root: v,\n            best_valid_score: i32::MIN,\n\
    \            best_node: !0,\n            dfs_stack: Vec::with_capacity(nodes_capacity\
    \ * 2),\n            candidates: vec![vec![]; max_turns + 1],\n        }\n   \
    \ }\n\n    pub fn run(&mut self) -> Vec<S::A> {\n        let timer = Timer::new();\n\
    \        let mut appeared = NopHashSet::default();\n\n        for turn in 0..=self.max_turns\
    \ {\n            let width = self.width_manager.width(turn, timer.elapsed_secs());\n\
    \n            let mut ord = (0..self.candidates[turn].len() as u32).collect::<Vec<_>>();\n\
    \            if self.candidates[turn].len() > width * 2 {\n                ord.select_nth_unstable_by_key(width\
    \ * 2, |&i| {\n                    Reverse(self.candidates[turn][i as usize].score)\n\
    \                });\n                ord.truncate(width * 2);\n            }\n\
    \            ord.sort_unstable_by_key(|&i| Reverse(self.candidates[turn][i as\
    \ usize].score));\n\n            appeared.clear();\n            let mut cnt =\
    \ 0;\n            for &i in &ord {\n                let i = i as usize;\n\n  \
    \              // \u30CF\u30C3\u30B7\u30E5\u304C\u88AB\u3063\u305F\u3089\u3001\
    1\u500B\u3060\u3051\u6B8B\u3059\u3002\n                // \u5225\u306E\u65B9\u91DD\
    \u3068\u3057\u3066\u306F\u3001\u6D88\u3055\u305A\u306B\u30DA\u30CA\u30EB\u30C6\
    \u30A3\u3092\u4E0E\u3048\u308B\u3068\u304B\u3082\u3042\u308B\u304B\u3082\u3002\
    \n                if appeared.insert(self.candidates[turn][i].hash) {\n      \
    \              self.add_node(self.candidates[turn][i].clone());\n            \
    \        cnt += 1;\n                    if cnt >= width {\n                  \
    \      break;\n                    }\n                }\n            }\n\n   \
    \         self.candidates[turn].clear();\n            self.candidates[turn].shrink_to_fit();\n\
    \n            if turn == self.max_turns {\n                break;\n          \
    \  }\n\n            self.dfs(turn);\n        }\n\n        let mut res = vec![];\n\
    \        let mut v = self.best_node;\n\n        assert!(v != !0, \"\u89E3\u304C\
    \u898B\u3064\u304B\u308A\u307E\u305B\u3093\u3067\u3057\u305F\");\n\n        loop\
    \ {\n            let node = self.nodes.get(v);\n            let parent = node.parent;\n\
    \            if parent == !0 {\n                break;\n            }\n      \
    \      res.push(node.action.clone());\n            v = parent;\n        }\n\n\
    \        res.reverse();\n        res\n    }\n\n    // \u65B0\u3057\u3044\u30CE\
    \u30FC\u30C9\u3092\u9577\u7537\u3068\u3057\u3066\u8FFD\u52A0\u3059\u308B\n   \
    \ fn add_node(&mut self, cand: Candidate<S::A>) -> u32 {\n        let parent =\
    \ cand.parent;\n        let sibling = self.nodes.get(parent).child;\n        let\
    \ u = self.nodes.push(Node {\n            action: cand.action,\n            parent,\n\
    \            child: !0,\n            left: !0,\n            right: sibling,\n\
    \        });\n        self.nodes.get_mut(parent).child = u;\n        if sibling\
    \ != !0 {\n            self.nodes.get_mut(sibling).left = u;\n        }\n    \
    \    if cand.valid && cand.score > self.best_valid_score {\n            self.best_node\
    \ = u;\n            self.best_valid_score = cand.score;\n        }\n        u\n\
    \    }\n\n    // \u73FE\u5728\u306E\u30BF\u30FC\u30F3\u6570\u304C target_turn\
    \ \u3067\u3042\u308B\u72B6\u614B\u305F\u3061\u306E\u5B50\u306E\u5019\u88DC\u3092\
    \u5217\u6319\u3059\u308B\u3002\n    // \u3064\u3044\u3067\u306B\u3001\u9014\u4E2D\
    \u3067\u898B\u3064\u3051\u305F\u4E0D\u8981\u306A\u30CE\u30FC\u30C9\u3092\u524A\
    \u9664\u3059\u308B\u3002\n    fn dfs(&mut self, target_turn: usize) {\n      \
    \  assert!(self.dfs_stack.is_empty());\n\n        self.update_root();\n      \
    \  // self.v == self.root\n\n        if self.turn > target_turn {\n          \
    \  return;\n        }\n\n        if self.turn == target_turn {\n            self.enumerate_candidates();\n\
    \            return;\n        }\n\n        let mut u = self.child(self.v);\n\n\
    \        while u != !0 {\n            let next_turn = self.turn + self.nodes.get(u).action.consumed_turns();\n\
    \            if next_turn <= target_turn {\n                self.dfs_stack.push(u);\n\
    \            }\n            u = self.right(u);\n        }\n\n        let mut disused_nodes\
    \ = vec![];\n        while let Some(u) = self.dfs_stack.pop() {\n            if\
    \ u == !0 {\n                self.move_to_parent();\n                continue;\n\
    \            }\n\n            self.dfs_stack.push(!0);\n            self.move_to_child(u);\n\
    \            if self.turn == target_turn {\n                assert!(!self.has_child(self.v));\n\
    \                self.enumerate_candidates();\n            } else if self.turn\
    \ < target_turn {\n                let mut u = self.child(self.v);\n         \
    \       if u == !0 {\n                    if self.state.score() < self.best_valid_score\
    \ {\n                        disused_nodes.push(self.v);\n                   \
    \ }\n                } else {\n                    while u != !0 {\n         \
    \               let next_turn = self.turn + self.nodes.get(u).action.consumed_turns();\n\
    \                        if next_turn <= target_turn {\n                     \
    \       self.dfs_stack.push(u);\n                        }\n                 \
    \       u = self.right(u);\n                    }\n                }\n       \
    \     } else {\n                unreachable!()\n            }\n        }\n   \
    \     assert!(self.v == self.root);\n\n        for v in disused_nodes {\n    \
    \        self.remove_leaf(v);\n        }\n    }\n\n    // \u73FE\u5728\u306E\u72B6\
    \u614B\u306E\u5B50\u306E\u5019\u88DC\u3092\u5217\u6319\u3059\u308B\n    fn enumerate_candidates(&mut\
    \ self) {\n        let actions = self.state.enumerate_actions();\n        for\
    \ action in actions {\n            let next_turn = self.turn + action.consumed_turns();\n\
    \            if next_turn > self.max_turns {\n                continue;\n    \
    \        }\n\n            // \u30CE\u30FC\u30C9\u3092\u79FB\u52D5\u3057\u3066\u3044\
    \u308B\u308F\u3051\u3067\u306F\u306A\u3044\u3051\u3069\u3001score \u3092\u8A08\
    \u7B97\u3059\u308B\u305F\u3081\u306B\u4E00\u6642\u7684\u306B state \u3092\u5909\
    \u5316\u3055\u305B\u3066\u3001\n            // \u7D42\u308F\u3063\u305F\u3089\u3059\
    \u3050\u306B\u5143\u306B\u623B\u3057\u3066\u3044\u308B\u3002\n            // apply_action\
    \ \u5F8C\u306E\u30B9\u30B3\u30A2\u3068\u30CF\u30C3\u30B7\u30E5\u3092 apply_action\
    \ \u305B\u305A\u306B\u9AD8\u901F\u306B\u8A08\u7B97\u3067\u304D\u308B\u3068\u304D\
    \u306F\u3001\n            // \u3053\u3046\u3057\u306A\u304F\u3066\u3082\u3044\u3044\
    \n            self.state.apply_action(&action);\n            self.candidates[next_turn].push(Candidate\
    \ {\n                action: action.clone(),\n                parent: self.v,\n\
    \                score: self.state.score(),\n                hash: self.state.hash(),\n\
    \                valid: self.state.is_valid(),\n            });\n            self.state.revert_action(&action);\n\
    \        }\n    }\n\n    // \u6839\u306E\u5B50\u304C\u3072\u3068\u3064\u3067\u3042\
    \u308B\u9593\u3001\u305D\u306E\u5B50\u3092\u6839\u306B\u3059\u308B\u3053\u3068\
    \u3092\u7E70\u308A\u8FD4\u3059\n    fn update_root(&mut self) {\n        assert!(self.v\
    \ == self.root);\n\n        while self.has_child(self.v) {\n            let child\
    \ = self.child(self.v);\n            if self.has_right(child) {\n            \
    \    break;\n            }\n            self.move_to_first_child();\n        }\n\
    \        self.root = self.v;\n    }\n\n    // \u8449 v \u3092\u524A\u9664\u3059\
    \u308B\u3002\u305D\u306E\u7D50\u679C\u89AA\u304C\u8449\u306B\u306A\u308B\u5834\
    \u5408\u306F\u3001\u305D\u306E\u89AA\u3082\u524A\u9664\u3059\u308B\u3002\u3053\
    \u308C\u3092\u7E70\u308A\u8FD4\u3059\n    fn remove_leaf(&mut self, mut v: u32)\
    \ {\n        assert!(!self.has_child(v));\n\n        while v != self.best_node\
    \ {\n            assert!(!self.has_child(v));\n\n            let parent = self.parent(v);\n\
    \            let left = self.left(v);\n            let right = self.right(v);\n\
    \            self.nodes.remove(v);\n            if left != !0 {\n            \
    \    self.nodes.get_mut(left).right = right;\n                if right != !0 {\n\
    \                    self.nodes.get_mut(right).left = left;\n                }\n\
    \                return;\n            } else {\n                assert!(parent\
    \ != !0, \"\u6839\u3092\u524A\u9664\u3057\u3088\u3046\u3068\u3057\u3066\u3044\u307E\
    \u3059\");\n                self.nodes.get_mut(parent).child = right;\n      \
    \          if right != !0 {\n                    self.nodes.get_mut(right).left\
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
  dependsOn:
  - crates/heuristic/timer/src/lib.rs
  isVerificationFile: false
  path: crates/heuristic/beam-search/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-25 11:36:32+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam-search/src/lib.rs
- /library/crates/heuristic/beam-search/src/lib.rs.html
title: crates/heuristic/beam-search/src/lib.rs
---
