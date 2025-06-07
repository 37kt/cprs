---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/candidate.rs
    title: crates/heuristic/beam_search/src/candidate.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/config.rs
    title: crates/heuristic/beam_search/src/config.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/evaluation.rs
    title: crates/heuristic/beam_search/src/evaluation.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/heap.rs
    title: crates/heuristic/beam_search/src/heap.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/index.rs
    title: crates/heuristic/beam_search/src/index.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/lib.rs
    title: crates/heuristic/beam_search/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/node.rs
    title: crates/heuristic/beam_search/src/node.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/nop_hash.rs
    title: crates/heuristic/beam_search/src/nop_hash.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/pool.rs
    title: crates/heuristic/beam_search/src/pool.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/state.rs
    title: crates/heuristic/beam_search/src/state.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/candidate.rs
    title: crates/heuristic/beam_search/src/candidate.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/config.rs
    title: crates/heuristic/beam_search/src/config.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/evaluation.rs
    title: crates/heuristic/beam_search/src/evaluation.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/heap.rs
    title: crates/heuristic/beam_search/src/heap.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/index.rs
    title: crates/heuristic/beam_search/src/index.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/lib.rs
    title: crates/heuristic/beam_search/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/node.rs
    title: crates/heuristic/beam_search/src/node.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/nop_hash.rs
    title: crates/heuristic/beam_search/src/nop_hash.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/pool.rs
    title: crates/heuristic/beam_search/src/pool.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/state.rs
    title: crates/heuristic/beam_search/src/state.rs
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
  code: "use std::cmp::Ordering;\n\nuse crate::{\n    candidate::Candidate,\n    config::{Config,\
    \ HashDeduplicationScope},\n    heap::Heap,\n    index::Index,\n    node::Node,\n\
    \    nop_hash::NopHashSet,\n    pool::Pool,\n    state::BeamState,\n};\n\npub\
    \ struct BeamSearch<State: BeamState> {\n    config: Config,\n\n    v: Index,\
    \ // \u73FE\u5728\u306E\u30CE\u30FC\u30C9\n    turn: usize,\n    state: State,\n\
    \    pool: Pool<Node<State::Action>>,\n    root: Index,\n\n    best_valid_score:\
    \ i64,\n    best_node: Index,\n\n    dfs_stack: Vec<Index>,\n    candidates: Vec<Heap<State::Action>>,\n\
    }\n\nimpl<State: BeamState> BeamSearch<State> {\n    pub fn new(config: Config,\
    \ initial_state: State) -> Self {\n        let mut pool = Pool::new(config.nodes_capacity);\n\
    \        let v = pool.push(Node {\n            action: State::Action::default(),\n\
    \            parent: None.into(),\n            child: None.into(),\n         \
    \   left: None.into(),\n            right: None.into(),\n            count_cands:\
    \ 0,\n        });\n\n        Self {\n            config,\n            v,\n   \
    \         turn: 0,\n            state: initial_state,\n            pool,\n   \
    \         root: v,\n            best_valid_score: i64::MIN,\n            best_node:\
    \ None.into(),\n            dfs_stack: Vec::with_capacity(config.nodes_capacity\
    \ * 2),\n            candidates: (0..=config.max_turn)\n                .map(|_|\
    \ Heap::new(config.beam_width))\n                .collect(),\n        }\n    }\n\
    \n    pub fn run(&mut self) -> Option<(Vec<State::Action>, i64)> {\n        let\
    \ mut appeared = NopHashSet::default();\n\n        for turn in 0..=self.config.max_turn\
    \ {\n            let mut cands = self.candidates[turn].drain().collect::<Vec<_>>();\n\
    \            cands.sort_unstable_by_key(|c| -c.score);\n\n            if matches!(\n\
    \                self.config.hash_deduplication_scope,\n                HashDeduplicationScope::PerTurn\n\
    \            ) {\n                appeared.clear();\n            }\n\n       \
    \     for c in cands {\n                let p = c.parent;\n                if\
    \ appeared.insert(c.hash) {\n                    self.add_node(c);\n         \
    \       }\n                self.pool.get_mut(p).count_cands -= 1;\n          \
    \      // if self.removable(p) {\n                //     self.remove_leaf(p);\n\
    \                // }\n            }\n\n            if (self.config.minimize_turn\
    \ && self.best_node.is_some())\n                || turn == self.config.max_turn\n\
    \            {\n                break;\n            }\n\n            self.dfs(turn);\n\
    \        }\n\n        let mut v = self.best_node;\n        if v.is_none() {\n\
    \            return None;\n        }\n\n        let mut actions = vec![];\n  \
    \      loop {\n            let node = self.pool.get(v);\n            let parent\
    \ = node.parent;\n            if parent.is_none() {\n                break;\n\
    \            }\n            actions.push(node.action.clone());\n            v\
    \ = parent;\n        }\n\n        actions.reverse();\n        Some((actions, self.best_valid_score))\n\
    \    }\n\n    // \u65B0\u3057\u3044\u30CE\u30FC\u30C9\u3092\u9577\u7537\u3068\u3057\
    \u3066\u8FFD\u52A0\u3059\u308B\n    fn add_node(&mut self, candidate: Candidate<State::Action>)\
    \ -> Index {\n        let parent = candidate.parent;\n        let sibling = self.pool.get(parent).child;\n\
    \        let u = self.pool.push(Node {\n            action: candidate.action,\n\
    \            parent,\n            child: None.into(),\n            left: None.into(),\n\
    \            right: sibling,\n            count_cands: 0,\n        });\n\n   \
    \     self.pool.get_mut(parent).child = u;\n        if sibling.is_some() {\n \
    \           self.pool.get_mut(sibling).left = u;\n        }\n        if candidate.valid\
    \ && self.best_valid_score < candidate.score {\n            self.best_node = u;\n\
    \            self.best_valid_score = candidate.score;\n        }\n\n        u\n\
    \    }\n\n    // \u73FE\u5728\u306E\u30BF\u30FC\u30F3\u6570\u304C target_turn\
    \ \u3067\u3042\u308B\u72B6\u614B\u305F\u3061\u306E\u5B50\u306E\u5019\u88DC\u3092\
    \u5217\u6319\u3059\u308B\u3002\n    // \u3064\u3044\u3067\u306B\u3001\u9014\u4E2D\
    \u3067\u898B\u3064\u3051\u305F\u4E0D\u8981\u306A\u30CE\u30FC\u30C9\u3092\u524A\
    \u9664\u3059\u308B\u3002\n    fn dfs(&mut self, target_turn: usize) {\n      \
    \  assert!(self.dfs_stack.is_empty());\n\n        self.update_root();\n      \
    \  assert_eq!(self.v, self.root);\n\n        match self.turn.cmp(&target_turn)\
    \ {\n            Ordering::Greater => return,\n            Ordering::Equal =>\
    \ return self.enumerate_candidates(),\n            _ => {}\n        }\n\n    \
    \    let mut u = self.child(self.v);\n        while u.is_some() {\n          \
    \  let action = &self.pool.get(u).action;\n            let next_turn = self.turn\
    \ + self.state.action_turns(action);\n            if next_turn <= target_turn\
    \ {\n                self.dfs_stack.push(u);\n            }\n            u = self.right(u);\n\
    \        }\n\n        let mut disused_nodes = vec![];\n        while let Some(u)\
    \ = self.dfs_stack.pop() {\n            if u.is_none() {\n                self.move_to_parent();\n\
    \                continue;\n            }\n\n            self.dfs_stack.push(None.into());\n\
    \            self.move_to_child(u);\n            match self.turn.cmp(&target_turn)\
    \ {\n                Ordering::Equal => {\n                    assert!(self.child(self.v).is_none());\n\
    \                    self.enumerate_candidates();\n                    if self.removable(self.v)\
    \ {\n                        disused_nodes.push(self.v);\n                   \
    \ }\n                }\n                Ordering::Less => {\n                \
    \    let mut u = self.child(self.v);\n                    if u.is_none() {\n \
    \                       if self.removable(self.v) {\n                        \
    \    disused_nodes.push(self.v);\n                        }\n                \
    \    } else {\n                        while u.is_some() {\n                 \
    \           let action = &self.pool.get(u).action;\n                         \
    \   let next_turn = self.turn + self.state.action_turns(action);\n           \
    \                 if next_turn <= target_turn {\n                            \
    \    self.dfs_stack.push(u);\n                            }\n                \
    \            u = self.right(u);\n                        }\n                 \
    \   }\n                }\n                _ => unreachable!(),\n            }\n\
    \        }\n        assert_eq!(self.v, self.root);\n\n        for v in disused_nodes\
    \ {\n            self.remove_leaf(v);\n        }\n    }\n\n    fn enumerate_candidates(&mut\
    \ self) {\n        for action in self.state.enumerate_actions() {\n          \
    \  let next_turn = self.turn + self.state.action_turns(&action);\n           \
    \ if next_turn > self.config.max_turn {\n                continue;\n         \
    \   }\n\n            let eval = self.state.evaluate_after_action(&action);\n \
    \           self.pool.get_mut(self.v).count_cands += 1;\n            let p = self.candidates[next_turn].push(Candidate\
    \ {\n                action,\n                parent: self.v,\n              \
    \  score: eval.score,\n                hash: eval.hash,\n                valid:\
    \ eval.valid,\n            });\n\n            if p.is_some() {\n             \
    \   self.pool.get_mut(p).count_cands -= 1;\n            }\n        }\n    }\n\n\
    \    fn update_root(&mut self) {\n        assert_eq!(self.v, self.root);\n\n \
    \       while self.child(self.v).is_some() {\n            let c = self.child(self.v);\n\
    \            if self.right(c).is_some() {\n                break;\n          \
    \  }\n            self.move_to_first_child();\n        }\n        self.root =\
    \ self.v;\n    }\n\n    fn removable(&self, v: Index) -> bool {\n        v.is_some()\n\
    \            && self.child(v).is_none()\n            && v != self.best_node\n\
    \            && self.pool.get(v).count_cands == 0\n    }\n\n    fn remove_leaf(&mut\
    \ self, mut v: Index) {\n        while self.removable(v) {\n            assert!(self.child(v).is_none());\n\
    \n            let p = self.parent(v);\n            let l = self.left(v);\n   \
    \         let r = self.right(v);\n            self.pool.remove(v);\n         \
    \   if l.is_some() {\n                self.pool.get_mut(l).right = r;\n      \
    \          if r.is_some() {\n                    self.pool.get_mut(r).left = l;\n\
    \                }\n                return;\n            } else {\n          \
    \      assert!(p.is_some(), \"\u6839\u3092\u524A\u9664\u3057\u3088\u3046\u3068\
    \u3057\u3066\u3044\u307E\u3059\");\n                self.pool.get_mut(p).child\
    \ = r;\n                if r.is_some() {\n                    self.pool.get_mut(r).left\
    \ = None.into();\n                }\n                v = p;\n            }\n \
    \       }\n    }\n\n    fn parent(&self, v: Index) -> Index {\n        self.pool.get(v).parent\n\
    \    }\n\n    fn child(&self, v: Index) -> Index {\n        self.pool.get(v).child\n\
    \    }\n\n    fn left(&self, v: Index) -> Index {\n        self.pool.get(v).left\n\
    \    }\n\n    fn right(&self, v: Index) -> Index {\n        self.pool.get(v).right\n\
    \    }\n\n    // \u89AA\u306B\u79FB\u52D5\u3059\u308B\n    fn move_to_parent(&mut\
    \ self) {\n        let node = &self.pool.get(self.v);\n        self.state.revert_action(&node.action);\n\
    \        self.turn -= self.state.action_turns(&node.action);\n        self.v =\
    \ node.parent;\n    }\n\n    // \u9577\u7537\u306B\u79FB\u52D5\u3059\u308B\n \
    \   fn move_to_first_child(&mut self) {\n        let u = self.child(self.v);\n\
    \        self.move_to_child(u);\n    }\n\n    // \u5B50\u3067\u3042\u308B u \u306B\
    \u79FB\u52D5\u3059\u308B\n    fn move_to_child(&mut self, u: Index) {\n      \
    \  self.v = u;\n        let action = &self.pool.get(self.v).action;\n        self.turn\
    \ += self.state.action_turns(action);\n        self.state.apply_action(action);\n\
    \    }\n}\n"
  dependsOn:
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/state.rs
  isVerificationFile: false
  path: crates/heuristic/beam_search/src/beam_search.rs
  requiredBy:
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/state.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/pool.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam_search/src/beam_search.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam_search/src/beam_search.rs
- /library/crates/heuristic/beam_search/src/beam_search.rs.html
title: crates/heuristic/beam_search/src/beam_search.rs
---
