// 参考: https://eijirou-kyopro.hatenablog.com/entry/2024/02/01/115639

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
};

use timer::Timer;

pub trait State {
    type A: Action;

    fn score(&mut self) -> i32;
    fn hash(&mut self) -> u64;
    fn is_valid(&mut self) -> bool;

    fn enumerate_actions(&mut self) -> Vec<Self::A>;

    fn apply_action(&mut self, action: &Self::A);
    fn revert_action(&mut self, action: &Self::A);
}

pub trait Action: Clone + Default {
    fn consumed_turns(&self) -> usize;
}

pub trait WidthManager {
    fn width(&mut self, turn: usize, elapsed_secs: f64) -> usize;
}

#[derive(Clone)]
struct Candidate<A: Action> {
    action: A,
    parent: u32,
    score: i32,
    hash: u64,
    valid: bool,
}

// 二重連鎖木のノード
#[derive(Clone, Default)]
struct Node<A: Action> {
    action: A,
    parent: u32,
    child: u32, // 長男
    left: u32,  // 兄
    right: u32, // 弟
}

struct Pool<T> {
    nodes: Vec<T>,
    free: Vec<u32>,
}

impl<T> Pool<T> {
    fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            free: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, node: T) -> u32 {
        if let Some(i) = self.free.pop() {
            self.nodes[i as usize] = node;
            i
        } else {
            let i = self.nodes.len() as u32;
            self.nodes.push(node);
            i
        }
    }

    fn remove(&mut self, i: u32) {
        self.free.push(i);
    }

    fn get(&self, i: u32) -> &T {
        &self.nodes[i as usize]
    }

    fn get_mut(&mut self, i: u32) -> &mut T {
        &mut self.nodes[i as usize]
    }
}

pub struct BeamSearch<S: State, W: WidthManager> {
    max_turns: usize,
    width_manager: W,

    v: u32, // 現在のノード
    turn: usize,
    state: S,
    nodes: Pool<Node<S::A>>,
    root: u32,
    best_valid_score: i32,
    best_node: u32,
    dfs_stack: Vec<u32>,
    candidates: Vec<Vec<Candidate<S::A>>>,
}

impl<S: State, W: WidthManager> BeamSearch<S, W> {
    pub fn new(
        initial_state: S,
        max_turns: usize,
        width_manager: W,
        nodes_capacity: usize,
    ) -> Self {
        let mut nodes = Pool::new(nodes_capacity);
        let v = nodes.push(Node {
            action: Default::default(),
            parent: !0,
            child: !0,
            left: !0,
            right: !0,
        });
        Self {
            max_turns,
            width_manager,
            v,
            turn: 0,
            state: initial_state,
            nodes,
            root: v,
            best_valid_score: i32::MIN,
            best_node: !0,
            dfs_stack: Vec::with_capacity(nodes_capacity * 2),
            candidates: vec![vec![]; max_turns + 1],
        }
    }

    pub fn run(&mut self) -> (Vec<S::A>, i32) {
        let timer = Timer::new();
        let mut appeared = NopHashSet::default();

        for turn in 0..=self.max_turns {
            let width = self.width_manager.width(turn, timer.elapsed_secs());

            let mut ord = (0..self.candidates[turn].len() as u32).collect::<Vec<_>>();
            if self.candidates[turn].len() > width * 2 {
                ord.select_nth_unstable_by_key(width * 2, |&i| {
                    Reverse(self.candidates[turn][i as usize].score)
                });
                ord.truncate(width * 2);
            }
            ord.sort_unstable_by_key(|&i| Reverse(self.candidates[turn][i as usize].score));

            appeared.clear();
            let mut cnt = 0;
            for &i in &ord {
                let i = i as usize;

                // ハッシュが被ったら、1個だけ残す。
                // 別の方針としては、消さずにペナルティを与えるとかもあるかも。
                if appeared.insert(self.candidates[turn][i].hash) {
                    self.add_node(self.candidates[turn][i].clone());
                    cnt += 1;
                    if cnt >= width {
                        break;
                    }
                }
            }

            self.candidates[turn].clear();
            self.candidates[turn].shrink_to_fit();

            if turn == self.max_turns {
                break;
            }

            self.dfs(turn);
        }

        let mut res = vec![];
        let mut v = self.best_node;

        assert!(v != !0, "解が見つかりませんでした");

        loop {
            let node = self.nodes.get(v);
            let parent = node.parent;
            if parent == !0 {
                break;
            }
            res.push(node.action.clone());
            v = parent;
        }

        res.reverse();
        (res, self.best_valid_score)
    }

    // 新しいノードを長男として追加する
    fn add_node(&mut self, cand: Candidate<S::A>) -> u32 {
        let parent = cand.parent;
        let sibling = self.nodes.get(parent).child;
        let u = self.nodes.push(Node {
            action: cand.action,
            parent,
            child: !0,
            left: !0,
            right: sibling,
        });
        self.nodes.get_mut(parent).child = u;
        if sibling != !0 {
            self.nodes.get_mut(sibling).left = u;
        }
        if cand.valid && cand.score > self.best_valid_score {
            self.best_node = u;
            self.best_valid_score = cand.score;
        }
        u
    }

    // 現在のターン数が target_turn である状態たちの子の候補を列挙する。
    // ついでに、途中で見つけた不要なノードを削除する。
    fn dfs(&mut self, target_turn: usize) {
        assert!(self.dfs_stack.is_empty());

        self.update_root();
        // self.v == self.root

        if self.turn > target_turn {
            return;
        }

        if self.turn == target_turn {
            self.enumerate_candidates();
            return;
        }

        let mut u = self.child(self.v);

        while u != !0 {
            let next_turn = self.turn + self.nodes.get(u).action.consumed_turns();
            if next_turn <= target_turn {
                self.dfs_stack.push(u);
            }
            u = self.right(u);
        }

        let mut disused_nodes = vec![];
        while let Some(u) = self.dfs_stack.pop() {
            if u == !0 {
                self.move_to_parent();
                continue;
            }

            self.dfs_stack.push(!0);
            self.move_to_child(u);
            if self.turn == target_turn {
                assert!(!self.has_child(self.v));
                self.enumerate_candidates();
            } else if self.turn < target_turn {
                let mut u = self.child(self.v);
                if u == !0 {
                    if self.state.score() < self.best_valid_score {
                        disused_nodes.push(self.v);
                    }
                } else {
                    while u != !0 {
                        let next_turn = self.turn + self.nodes.get(u).action.consumed_turns();
                        if next_turn <= target_turn {
                            self.dfs_stack.push(u);
                        }
                        u = self.right(u);
                    }
                }
            } else {
                unreachable!()
            }
        }
        assert!(self.v == self.root);

        for v in disused_nodes {
            self.remove_leaf(v);
        }
    }

    // 現在の状態の子の候補を列挙する
    fn enumerate_candidates(&mut self) {
        let actions = self.state.enumerate_actions();
        for action in actions {
            let next_turn = self.turn + action.consumed_turns();
            if next_turn > self.max_turns {
                continue;
            }

            // ノードを移動しているわけではないけど、score を計算するために一時的に state を変化させて、
            // 終わったらすぐに元に戻している。
            // apply_action 後のスコアとハッシュを apply_action せずに高速に計算できるときは、
            // こうしなくてもいい
            self.state.apply_action(&action);
            self.candidates[next_turn].push(Candidate {
                action: action.clone(),
                parent: self.v,
                score: self.state.score(),
                hash: self.state.hash(),
                valid: self.state.is_valid(),
            });
            self.state.revert_action(&action);
        }
    }

    // 根の子がひとつである間、その子を根にすることを繰り返す
    fn update_root(&mut self) {
        assert!(self.v == self.root);

        while self.has_child(self.v) {
            let child = self.child(self.v);
            if self.has_right(child) {
                break;
            }
            self.move_to_first_child();
        }
        self.root = self.v;
    }

    // 葉 v を削除する。その結果親が葉になる場合は、その親も削除する。これを繰り返す
    fn remove_leaf(&mut self, mut v: u32) {
        assert!(!self.has_child(v));

        while v != self.best_node {
            assert!(!self.has_child(v));

            let parent = self.parent(v);
            let left = self.left(v);
            let right = self.right(v);
            self.nodes.remove(v);
            if left != !0 {
                self.nodes.get_mut(left).right = right;
                if right != !0 {
                    self.nodes.get_mut(right).left = left;
                }
                return;
            } else {
                assert!(parent != !0, "根を削除しようとしています");
                self.nodes.get_mut(parent).child = right;
                if right != !0 {
                    self.nodes.get_mut(right).left = !0;
                    return;
                }
                v = parent;
            }
        }
    }

    #[allow(unused)]
    fn has_parent(&self, v: u32) -> bool {
        self.nodes.get(v).parent != !0
    }

    fn parent(&self, v: u32) -> u32 {
        self.nodes.get(v).parent
    }

    fn has_child(&self, v: u32) -> bool {
        self.nodes.get(v).child != !0
    }

    fn child(&self, v: u32) -> u32 {
        self.nodes.get(v).child
    }

    #[allow(unused)]
    fn has_left(&self, v: u32) -> bool {
        self.nodes.get(v).left != !0
    }

    fn left(&self, v: u32) -> u32 {
        self.nodes.get(v).left
    }

    fn has_right(&self, v: u32) -> bool {
        self.nodes.get(v).right != !0
    }

    fn right(&self, v: u32) -> u32 {
        self.nodes.get(v).right
    }

    // 親に移動する
    fn move_to_parent(&mut self) {
        let node = &self.nodes.get(self.v);
        self.state.revert_action(&node.action);
        self.turn -= node.action.consumed_turns();
        self.v = node.parent;
    }

    // 長男に移動する
    fn move_to_first_child(&mut self) {
        let u = self.child(self.v);
        self.move_to_child(u);
    }

    // 子である u に移動する
    fn move_to_child(&mut self, u: u32) {
        self.v = u;
        let action = &self.nodes.get(self.v).action;
        self.state.apply_action(action);
        self.turn += action.consumed_turns();
    }

    #[allow(unused)]
    // 弟に移動する
    fn move_to_right(&mut self) {
        let u = self.right(self.v);
        self.move_to_sibling(u);
    }

    // 兄弟である u に移動する
    fn move_to_sibling(&mut self, u: u32) {
        self.move_to_parent();
        self.move_to_child(u);
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
