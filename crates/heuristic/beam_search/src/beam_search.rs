use std::{cmp::Ordering, mem::MaybeUninit};

use crate::{
    candidate::Candidate,
    config::{Config, HashDeduplicationScope},
    heap::Heap,
    index::Index,
    node::Node,
    nop_hash::NopHashSet,
    pool::Pool,
    state::BeamState,
};

pub struct BeamSearch<State: BeamState> {
    config: Config,

    v: Index, // 現在のノード
    turn: usize,
    state: State,
    pool: Pool<Node<State::Action>>,
    root: Index,

    best_valid_score: i64,
    best_node: Index,

    dfs_stack: Vec<Index>,
    candidates: Vec<Heap<State::Action>>,
}

impl<State: BeamState> BeamSearch<State> {
    pub fn new(config: Config, initial_state: State) -> Self {
        let mut pool = Pool::new(config.nodes_capacity);
        let v = pool.push(Node {
            #[allow(clippy::uninit_assumed_init)]
            action: unsafe { MaybeUninit::uninit().assume_init() },
            parent: None.into(),
            child: None.into(),
            left: None.into(),
            right: None.into(),
            count_cands: 0,
        });

        Self {
            config,
            v,
            turn: 0,
            state: initial_state,
            pool,
            root: v,
            best_valid_score: i64::MIN,
            best_node: None.into(),
            dfs_stack: Vec::with_capacity(config.nodes_capacity * 2),
            candidates: (0..=config.max_turn)
                .map(|_| Heap::new(config.beam_width))
                .collect(),
        }
    }

    pub fn run(&mut self) -> Option<(Vec<State::Action>, i64)> {
        let mut appeared = NopHashSet::default();

        for turn in 0..=self.config.max_turn {
            let mut cands = self.candidates[turn].drain().collect::<Vec<_>>();
            cands.sort_unstable_by_key(|c| -c.score);

            if matches!(
                self.config.hash_deduplication_scope,
                HashDeduplicationScope::PerTurn
            ) {
                appeared.clear();
            }

            for c in cands {
                let p = c.parent;
                if appeared.insert(c.hash) {
                    self.add_node(c);
                }
                self.pool.get_mut(p).count_cands -= 1;
                // if self.removable(p) {
                //     self.remove_leaf(p);
                // }
            }

            if (self.config.minimize_turn && self.best_node.is_some())
                || turn == self.config.max_turn
            {
                break;
            }

            self.dfs(turn);
        }

        let mut v = self.best_node;
        if v.is_none() {
            return None;
        }

        let mut actions = vec![];
        loop {
            let node = self.pool.get(v);
            let parent = node.parent;
            if parent.is_none() {
                break;
            }
            actions.push(node.action.clone());
            v = parent;
        }

        actions.reverse();
        Some((actions, self.best_valid_score))
    }

    // 新しいノードを長男として追加する
    fn add_node(&mut self, candidate: Candidate<State::Action>) -> Index {
        let parent = candidate.parent;
        let sibling = self.pool.get(parent).child;
        let u = self.pool.push(Node {
            action: candidate.action,
            parent,
            child: None.into(),
            left: None.into(),
            right: sibling,
            count_cands: 0,
        });

        self.pool.get_mut(parent).child = u;
        if sibling.is_some() {
            self.pool.get_mut(sibling).left = u;
        }
        if candidate.valid && self.best_valid_score < candidate.score {
            self.best_node = u;
            self.best_valid_score = candidate.score;
        }

        u
    }

    // 現在のターン数が target_turn である状態たちの子の候補を列挙する。
    // ついでに、途中で見つけた不要なノードを削除する。
    fn dfs(&mut self, target_turn: usize) {
        assert!(self.dfs_stack.is_empty());

        self.update_root();
        assert_eq!(self.v, self.root);

        match self.turn.cmp(&target_turn) {
            Ordering::Greater => return,
            Ordering::Equal => return self.enumerate_candidates(),
            _ => {}
        }

        let mut u = self.child(self.v);
        while u.is_some() {
            let action = &self.pool.get(u).action;
            let next_turn = self.turn + self.state.action_turns(action);
            if next_turn <= target_turn {
                self.dfs_stack.push(u);
            }
            u = self.right(u);
        }

        let mut disused_nodes = vec![];
        while let Some(u) = self.dfs_stack.pop() {
            if u.is_none() {
                self.move_to_parent();
                continue;
            }

            self.dfs_stack.push(None.into());
            self.move_to_child(u);
            match self.turn.cmp(&target_turn) {
                Ordering::Equal => {
                    assert!(self.child(self.v).is_none());
                    self.enumerate_candidates();
                    if self.removable(self.v) {
                        disused_nodes.push(self.v);
                    }
                }
                Ordering::Less => {
                    let mut u = self.child(self.v);
                    if u.is_none() {
                        if self.removable(self.v) {
                            disused_nodes.push(self.v);
                        }
                    } else {
                        while u.is_some() {
                            let action = &self.pool.get(u).action;
                            let next_turn = self.turn + self.state.action_turns(action);
                            if next_turn <= target_turn {
                                self.dfs_stack.push(u);
                            }
                            u = self.right(u);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(self.v, self.root);

        for v in disused_nodes {
            self.remove_leaf(v);
        }
    }

    fn enumerate_candidates(&mut self) {
        for action in self.state.enumerate_actions() {
            let next_turn = self.turn + self.state.action_turns(&action);
            if next_turn > self.config.max_turn {
                continue;
            }

            let eval = self.state.evaluate_after_action(&action);
            self.pool.get_mut(self.v).count_cands += 1;
            let p = self.candidates[next_turn].push(Candidate {
                action,
                parent: self.v,
                score: eval.score,
                hash: eval.hash,
                valid: eval.valid,
            });

            if p.is_some() {
                self.pool.get_mut(p).count_cands -= 1;
            }
        }
    }

    fn update_root(&mut self) {
        assert_eq!(self.v, self.root);

        while self.child(self.v).is_some() {
            let c = self.child(self.v);
            if self.right(c).is_some() {
                break;
            }
            self.move_to_first_child();
        }
        self.root = self.v;
    }

    fn removable(&self, v: Index) -> bool {
        v.is_some()
            && self.child(v).is_none()
            && v != self.best_node
            && self.pool.get(v).count_cands == 0
    }

    fn remove_leaf(&mut self, mut v: Index) {
        while self.removable(v) {
            assert!(self.child(v).is_none());

            let p = self.parent(v);
            let l = self.left(v);
            let r = self.right(v);
            self.pool.remove(v);
            if l.is_some() {
                self.pool.get_mut(l).right = r;
                if r.is_some() {
                    self.pool.get_mut(r).left = l;
                }
                return;
            } else {
                assert!(p.is_some(), "根を削除しようとしています");
                self.pool.get_mut(p).child = r;
                if r.is_some() {
                    self.pool.get_mut(r).left = None.into();
                }
                v = p;
            }
        }
    }

    fn parent(&self, v: Index) -> Index {
        self.pool.get(v).parent
    }

    fn child(&self, v: Index) -> Index {
        self.pool.get(v).child
    }

    fn left(&self, v: Index) -> Index {
        self.pool.get(v).left
    }

    fn right(&self, v: Index) -> Index {
        self.pool.get(v).right
    }

    // 親に移動する
    fn move_to_parent(&mut self) {
        let node = &self.pool.get(self.v);
        self.state.revert_action(&node.action);
        self.turn -= self.state.action_turns(&node.action);
        self.v = node.parent;
    }

    // 長男に移動する
    fn move_to_first_child(&mut self) {
        let u = self.child(self.v);
        self.move_to_child(u);
    }

    // 子である u に移動する
    fn move_to_child(&mut self, u: Index) {
        self.v = u;
        let action = &self.pool.get(self.v).action;
        self.turn += self.state.action_turns(action);
        self.state.apply_action(action);
    }
}
