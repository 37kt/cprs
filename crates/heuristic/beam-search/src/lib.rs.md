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
    - https://qiita.com/rhoo/items/2f647e32f6ff2c6ee056
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// rhoo\u3055\u3093\u306E\u8A18\u4E8B\u3092\u53C2\u8003\u306B\u3055\u305B\
    \u3066\u3044\u305F\u3060\u304D\u307E\u3057\u305F\n// https://qiita.com/rhoo/items/2f647e32f6ff2c6ee056\n\
    \nuse std::{\n    cmp::Reverse,\n    collections::{HashMap, HashSet},\n    hash::{BuildHasherDefault,\
    \ Hasher},\n};\n\npub trait State\nwhere\n    Self: Clone,\n{\n    fn turn(&self)\
    \ -> usize;\n    fn score(&self) -> i32;\n    fn hash(&self) -> u64;\n    fn is_valid(&self)\
    \ -> bool;\n}\n\npub trait Action\nwhere\n    Self: Clone + Sized,\n{\n    type\
    \ S: State;\n\n    fn apply(&self, state: &mut Self::S);\n    fn revert(&self,\
    \ state: &mut Self::S);\n    fn comsumed_turns(&self) -> usize {\n        1\n\
    \    }\n    fn enumerate_actions(state: &Self::S, actions: &mut Vec<Self>);\n\
    }\n\npub trait WidthManager {\n    fn beam_width(&self, turn: usize, elapsed:\
    \ f64) -> usize;\n}\n\npub struct FixedWidthManager {\n    width: usize,\n}\n\n\
    impl FixedWidthManager {\n    pub fn new(width: usize) -> Self {\n        Self\
    \ { width }\n    }\n}\n\nimpl WidthManager for FixedWidthManager {\n    fn beam_width(&self,\
    \ _: usize, _: f64) -> usize {\n        self.width\n    }\n}\n\n#[derive(Clone,\
    \ Default)]\nstruct Node<A: Action> {\n    action: Option<A>,\n    parent: u32,\n\
    \    child: u32,\n    prev: u32,\n    next: u32,\n    refs: u32,\n    valid: u32,\n\
    }\n\n#[derive(Clone)]\nstruct Candidate<A: Action> {\n    action: A,\n    parent:\
    \ u32,\n    score: i32,\n    hash: u64,\n    valid: bool,\n}\n\npub struct BeamSearch<S:\
    \ State, A: Action, W: WidthManager> {\n    state: S,\n    nodes: Vec<Node<A>>,\n\
    \    que: Vec<u32>,\n    cur_node: usize,\n    free: Vec<u32>,\n    at: u32,\n\
    \    max_turn: usize,\n    width_manager: W,\n}\n\nimpl<S, A, W> BeamSearch<S,\
    \ A, W>\nwhere\n    S: State,\n    A: Action<S = S>,\n    W: WidthManager,\n{\n\
    \    pub fn new(state: S, turn: usize, width_manager: W) -> Self {\n        const\
    \ MAX_NODES: usize = 1 << 20;\n        let nodes = vec![\n            Node {\n\
    \                action: None,\n                parent: !0,\n                child:\
    \ !0,\n                prev: !0,\n                next: !0,\n                refs:\
    \ 0,\n                valid: 0,\n            };\n            MAX_NODES\n     \
    \   ];\n        let free = (1..MAX_NODES as u32).rev().collect();\n\n        Self\
    \ {\n            state,\n            nodes,\n            que: Vec::with_capacity(MAX_NODES),\n\
    \            cur_node: 0,\n            free,\n            at: 0,\n           \
    \ width_manager,\n            max_turn: turn,\n        }\n    }\n\n    fn add_node(&mut\
    \ self, cand: Candidate<A>) {\n        let next = self.nodes[cand.parent as usize].child;\n\
    \        let new = self.free.pop().unwrap();\n        if next != !0 {\n      \
    \      self.nodes[next as usize].prev = new;\n        }\n        self.nodes[cand.parent\
    \ as usize].child = new;\n\n        self.nodes[new as usize] = Node {\n      \
    \      action: Some(cand.action),\n            parent: cand.parent,\n        \
    \    child: !0,\n            prev: !0,\n            next,\n            refs: 0,\n\
    \            valid: 0,\n        };\n        self.retarget(new);\n    }\n\n   \
    \ fn del_node(&mut self, mut idx: u32) {\n        assert_eq!(self.nodes[idx as\
    \ usize].refs, 0);\n        loop {\n            self.free.push(idx);\n       \
    \     let Node {\n                prev, next, parent, ..\n            } = self.nodes[idx\
    \ as usize];\n            assert_ne!(parent, !0);\n            self.nodes[parent\
    \ as usize].refs -= 1;\n            if prev & next == !0 && self.nodes[parent\
    \ as usize].refs == 0 {\n                idx = parent;\n                continue;\n\
    \            }\n\n            if prev != !0 {\n                self.nodes[prev\
    \ as usize].next = next;\n            } else {\n                self.nodes[parent\
    \ as usize].child = next;\n            }\n            if next != !0 {\n      \
    \          self.nodes[next as usize].prev = prev;\n            }\n\n         \
    \   break;\n        }\n    }\n\n    fn dfs(&mut self, turn: usize, cands: &mut\
    \ Vec<Vec<Candidate<A>>>, single: bool) {\n        if self.nodes[self.cur_node].child\
    \ == !0 {\n            let cnt = self.append_cands(turn, self.cur_node, cands);\n\
    \            if cnt == 0 {\n                self.que.push(self.cur_node as u32);\n\
    \            }\n            self.nodes[self.cur_node].refs += cnt;\n         \
    \   return;\n        }\n\n        let node = self.cur_node;\n        let mut child\
    \ = self.nodes[node].child;\n        let next_single = single & (self.nodes[child\
    \ as usize].next == !0);\n\n        'a: loop {\n            while self.nodes[child\
    \ as usize].valid != self.at {\n                child = self.nodes[child as usize].next;\n\
    \                if child == !0 {\n                    break 'a;\n           \
    \     }\n            }\n\n            self.cur_node = child as usize;\n      \
    \      self.nodes[child as usize]\n                .action\n                .as_ref()\n\
    \                .unwrap()\n                .apply(&mut self.state);\n       \
    \     self.dfs(turn, cands, next_single);\n\n            if !next_single {\n \
    \               self.nodes[child as usize]\n                    .action\n    \
    \                .as_ref()\n                    .unwrap()\n                  \
    \  .revert(&mut self.state);\n            }\n\n            child = self.nodes[child\
    \ as usize].next;\n            if child == !0 {\n                break;\n    \
    \        }\n        }\n\n        if !next_single {\n            self.cur_node\
    \ = node;\n        }\n    }\n\n    fn enum_cands(&mut self, turn: usize, cands:\
    \ &mut Vec<Vec<Candidate<A>>>) {\n        assert_eq!(self.nodes[self.cur_node].valid,\
    \ self.at);\n        self.que.clear();\n        self.dfs(turn, cands, true);\n\
    \    }\n\n    fn retarget(&mut self, mut idx: u32) {\n        while self.nodes[idx\
    \ as usize].valid != self.at {\n            self.nodes[idx as usize].valid = self.at;\n\
    \            if idx as usize == self.cur_node {\n                break;\n    \
    \        }\n            idx = self.nodes[idx as usize].parent;\n        }\n  \
    \  }\n\n    fn update(&mut self, cands: impl Iterator<Item = (Candidate<A>, bool)>)\
    \ {\n        self.at += 1;\n        for i in 0..self.que.len() {\n           \
    \ self.del_node(self.que[i]);\n        }\n\n        for (cand, f) in cands {\n\
    \            let node = &mut self.nodes[cand.parent as usize];\n            if\
    \ f {\n                self.add_node(cand)\n            } else {\n           \
    \     node.refs -= 1;\n                if node.refs == 0 {\n                 \
    \   self.del_node(cand.parent);\n                }\n            }\n        }\n\
    \    }\n\n    fn append_cands(&mut self, turn: usize, idx: usize, cands: &mut\
    \ Vec<Vec<Candidate<A>>>) -> u32 {\n        let node = &self.nodes[idx];\n   \
    \     assert_eq!(node.child, !0);\n\n        let mut res = 0;\n        let mut\
    \ actions = vec![];\n        A::enumerate_actions(&self.state, &mut actions);\n\
    \        for action in actions {\n            let mut next_turn = turn + action.comsumed_turns();\n\
    \            if next_turn > self.max_turn {\n                continue;\n     \
    \       }\n            action.apply(&mut self.state);\n            if self.state.is_valid()\
    \ {\n                next_turn = self.max_turn;\n            }\n            cands[next_turn].push(Candidate\
    \ {\n                action: action.clone(),\n                parent: idx as u32,\n\
    \                score: self.state.score(),\n                hash: self.state.hash(),\n\
    \                valid: self.state.is_valid(),\n            });\n            action.revert(&mut\
    \ self.state);\n            res += 1;\n        }\n        res\n    }\n\n    pub\
    \ fn solve(&mut self) -> (Vec<A>, i32) {\n        let start = std::time::Instant::now();\n\
    \        let mut cands: Vec<Vec<Candidate<A>>> =\n            (0..=self.max_turn).map(|_|\
    \ vec![]).collect::<Vec<_>>();\n        let mut set = NopHashSet::default();\n\
    \        // let mut best = None;\n        for t in 0..self.max_turn {\n      \
    \      let w = self\n                .width_manager\n                .beam_width(t,\
    \ start.elapsed().as_secs_f64());\n            if t != 0 {\n                let\
    \ cands = &mut cands[t];\n                if cands.len() > w * 2 {\n         \
    \           cands.select_nth_unstable_by_key(w * 2, |c| Reverse(c.score));\n \
    \                   cands.truncate(w * 2);\n                }\n\n            \
    \    cands.sort_unstable_by_key(|c| Reverse(c.score));\n                set.clear();\n\
    \                let mut total = 0;\n                // self.update(cands.iter().map(|c|\
    \ {\n                //     let f = total < w && set.insert(c.hash);\n       \
    \         //     total += f as usize;\n                //     if f && best\n \
    \               //         .as_ref()\n                //         .map_or(std::i32::MIN,\
    \ |b: &Candidate<A>| b.score)\n                //         < c.score\n        \
    \        //         && c.valid\n                //     {\n                // \
    \        best = Some(c.clone());\n                //     }\n                //\
    \     (c.clone(), f)\n                // }));\n                self.update(cands.iter().map(|c|\
    \ {\n                    let f = total < w && set.insert(c.hash);\n          \
    \          total += f as usize;\n                    (c.clone(), f)\n        \
    \        }));\n            }\n            // if t < self.max_turn {\n        \
    \    if t == 0 || cands[t].len() != 0 {\n                self.enum_cands(t, &mut\
    \ cands);\n            }\n            // }\n        }\n\n        let best = cands[self.max_turn]\n\
    \            .iter()\n            .filter(|c| c.valid)\n            .max_by_key(|c|\
    \ c.score)\n            .cloned()\n            .unwrap();\n        let mut res\
    \ = vec![];\n        let mut idx = best.parent;\n        loop {\n            let\
    \ Node { action, parent, .. } = &self.nodes[idx as usize];\n            if *parent\
    \ == !0 {\n                break;\n            }\n            res.push(action.as_ref().unwrap().clone());\n\
    \            idx = *parent;\n        }\n        res.reverse();\n        res.push(best.action);\n\
    \        (res, best.score)\n    }\n}\n\n#[derive(Default)]\nstruct NopHasher {\n\
    \    hash: u64,\n}\n\nimpl Hasher for NopHasher {\n    fn write(&mut self, _:\
    \ &[u8]) {\n        unimplemented!()\n    }\n\n    #[inline]\n    fn write_u64(&mut\
    \ self, n: u64) {\n        self.hash = n;\n    }\n\n    #[inline]\n    fn finish(&self)\
    \ -> u64 {\n        self.hash\n    }\n}\n\n#[allow(dead_code)]\ntype NopHashMap<K,\
    \ V> = HashMap<K, V, BuildHasherDefault<NopHasher>>;\ntype NopHashSet<K> = HashSet<K,\
    \ BuildHasherDefault<NopHasher>>;\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/heuristic/beam-search/src/lib.rs
  requiredBy: []
  timestamp: '2024-06-19 10:50:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam-search/src/lib.rs
- /library/crates/heuristic/beam-search/src/lib.rs.html
title: crates/heuristic/beam-search/src/lib.rs
---
