// rhooさんの記事を参考にさせていただきました
// https://qiita.com/rhoo/items/2f647e32f6ff2c6ee056

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
};

pub trait State
where
    Self: Clone,
{
    fn turn(&self) -> usize;
    fn score(&self) -> i32;
    fn hash(&self) -> u64;
    fn is_valid(&self) -> bool;
}

pub trait Action
where
    Self: Clone + Sized,
{
    type S: State;

    fn apply(&self, state: &mut Self::S);
    fn revert(&self, state: &mut Self::S);
    fn comsumed_turns(&self) -> usize {
        1
    }
    fn enumerate_actions(state: &Self::S, actions: &mut Vec<Self>);
}

pub trait WidthManager {
    fn beam_width(&self, turn: usize, elapsed: f64) -> usize;
}

pub struct FixedWidthManager {
    width: usize,
}

impl FixedWidthManager {
    pub fn new(width: usize) -> Self {
        Self { width }
    }
}

impl WidthManager for FixedWidthManager {
    fn beam_width(&self, _: usize, _: f64) -> usize {
        self.width
    }
}

#[derive(Clone, Default)]
struct Node<A: Action> {
    action: Option<A>,
    parent: u32,
    child: u32,
    prev: u32,
    next: u32,
    refs: u32,
    valid: u32,
}

#[derive(Clone)]
struct Candidate<A: Action> {
    action: A,
    parent: u32,
    score: i32,
    hash: u64,
    valid: bool,
}

pub struct BeamSearch<S: State, A: Action, W: WidthManager> {
    state: S,
    nodes: Vec<Node<A>>,
    que: Vec<u32>,
    cur_node: usize,
    free: Vec<u32>,
    at: u32,
    max_turn: usize,
    width_manager: W,
}

impl<S, A, W> BeamSearch<S, A, W>
where
    S: State,
    A: Action<S = S>,
    W: WidthManager,
{
    pub fn new(state: S, turn: usize, width_manager: W) -> Self {
        const MAX_NODES: usize = 1 << 20;
        let nodes = vec![
            Node {
                action: None,
                parent: !0,
                child: !0,
                prev: !0,
                next: !0,
                refs: 0,
                valid: 0,
            };
            MAX_NODES
        ];
        let free = (1..MAX_NODES as u32).rev().collect();

        Self {
            state,
            nodes,
            que: Vec::with_capacity(MAX_NODES),
            cur_node: 0,
            free,
            at: 0,
            width_manager,
            max_turn: turn,
        }
    }

    fn add_node(&mut self, cand: Candidate<A>) {
        let next = self.nodes[cand.parent as usize].child;
        let new = self.free.pop().unwrap();
        if next != !0 {
            self.nodes[next as usize].prev = new;
        }
        self.nodes[cand.parent as usize].child = new;

        self.nodes[new as usize] = Node {
            action: Some(cand.action),
            parent: cand.parent,
            child: !0,
            prev: !0,
            next: next,
            refs: 0,
            valid: 0,
        };
        self.retarget(new);
    }

    fn del_node(&mut self, mut idx: u32) {
        assert_eq!(self.nodes[idx as usize].refs, 0);
        loop {
            self.free.push(idx);
            let Node {
                prev, next, parent, ..
            } = self.nodes[idx as usize];
            assert_ne!(parent, !0);
            if prev & next == !0 && self.nodes[parent as usize].refs == 0 {
                idx = parent;
                continue;
            }

            if prev != !0 {
                self.nodes[prev as usize].next = next;
            } else {
                self.nodes[parent as usize].child = next;
            }
            if next != !0 {
                self.nodes[next as usize].prev = prev;
            }

            break;
        }
    }

    fn dfs(&mut self, turn: usize, cands: &mut Vec<Vec<Candidate<A>>>, single: bool) {
        if self.nodes[self.cur_node].child == !0 {
            let cnt = self.append_cands(turn, self.cur_node, cands);
            if cnt == 0 {
                self.que.push(self.cur_node as u32);
            }
            self.nodes[self.cur_node].refs += cnt;
            return;
        }

        let node = self.cur_node;
        let mut child = self.nodes[node].child;
        let next_single = single & (self.nodes[child as usize].next == !0);

        'a: loop {
            loop {
                if child == !0 {
                    break 'a;
                } else if self.nodes[child as usize].valid == self.at {
                    break;
                }
                child = self.nodes[child as usize].next;
            }

            self.cur_node = child as usize;
            self.nodes[child as usize]
                .action
                .as_ref()
                .unwrap()
                .apply(&mut self.state);
            self.dfs(turn, cands, next_single);

            if !next_single {
                self.nodes[child as usize]
                    .action
                    .as_ref()
                    .unwrap()
                    .revert(&mut self.state);
            }

            child = self.nodes[child as usize].next;
        }

        if !next_single {
            self.cur_node = node;
        }
    }

    fn enum_cands(&mut self, turn: usize, cands: &mut Vec<Vec<Candidate<A>>>) {
        assert_eq!(self.nodes[self.cur_node].valid, self.at);
        self.que.clear();
        self.dfs(turn, cands, true);
    }

    fn retarget(&mut self, mut idx: u32) {
        while self.nodes[idx as usize].valid != self.at {
            self.nodes[idx as usize].valid = self.at;
            if idx as usize == self.cur_node {
                break;
            }
            idx = self.nodes[idx as usize].parent;
        }
    }

    fn update(&mut self, cands: impl Iterator<Item = (Candidate<A>, bool)>) {
        self.at += 1;
        for i in 0..self.que.len() {
            self.del_node(self.que[i]);
        }

        for (cand, f) in cands {
            let node = &mut self.nodes[cand.parent as usize];
            node.refs -= 1;
            if f {
                self.add_node(cand)
            } else if node.refs == 0 && node.child == !0 {
                self.del_node(cand.parent);
            }
        }
    }

    fn append_cands(&mut self, turn: usize, idx: usize, cands: &mut Vec<Vec<Candidate<A>>>) -> u32 {
        let node = &self.nodes[idx];
        assert_eq!(node.child, !0);

        let mut res = 0;
        let mut actions = vec![];
        A::enumerate_actions(&self.state, &mut actions);
        for action in actions {
            let next_turn = turn + action.comsumed_turns();
            if next_turn > self.max_turn {
                continue;
            }
            action.apply(&mut self.state);
            cands[next_turn].push(Candidate {
                action: action.clone(),
                parent: idx as u32,
                score: self.state.score(),
                hash: self.state.hash(),
                valid: self.state.is_valid(),
            });
            action.revert(&mut self.state);
            res += 1;
        }
        res
    }

    pub fn solve(&mut self) -> (Vec<A>, i32) {
        let start = std::time::Instant::now();
        let mut cands: Vec<Vec<Candidate<A>>> =
            (0..=self.max_turn).map(|_| vec![]).collect::<Vec<_>>();
        let mut set = NopHashSet::default();
        let mut best = None;
        for t in 0..=self.max_turn {
            let w = self
                .width_manager
                .beam_width(t, start.elapsed().as_secs_f64());
            if t != 0 {
                let cands = &mut cands[t];
                if cands.len() > w * 2 {
                    cands.select_nth_unstable_by_key(w * 2, |c| Reverse(c.score));
                    cands.truncate(w * 2);
                }

                cands.sort_unstable_by_key(|c| Reverse(c.score));
                set.clear();
                let mut total = 0;
                self.update(cands.iter().map(|c| {
                    let f = total < w && set.insert(c.hash);
                    total += f as usize;
                    if f && best
                        .as_ref()
                        .map_or(std::i32::MIN, |b: &Candidate<A>| b.score)
                        < c.score
                        && c.valid
                    {
                        best = Some(c.clone());
                    }
                    (c.clone(), f)
                }));
            }
            if t < self.max_turn {
                self.enum_cands(t, &mut cands);
            }
        }

        let best = best.unwrap();
        let mut res = vec![];
        let mut idx = best.parent;
        loop {
            let Node { action, parent, .. } = &self.nodes[idx as usize];
            if *parent == !0 {
                break;
            }
            res.push(action.as_ref().unwrap().clone());
            idx = *parent;
        }
        res.reverse();
        res.push(best.action);
        (res, best.score)
    }
}

#[derive(Default)]
struct NopHasher {
    hash: u64,
}

impl Hasher for NopHasher {
    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }

    #[inline]
    fn write_u64(&mut self, n: u64) {
        self.hash = n;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

#[allow(dead_code)]
type NopHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NopHasher>>;
type NopHashSet<K> = HashSet<K, BuildHasherDefault<NopHasher>>;
