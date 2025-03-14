// TODO: マージ

use std::ops::{Bound, RangeBounds};

const W: u8 = usize::BITS as u8;

#[derive(Clone)]
pub struct BinaryTrie {
    nodes: Vec<Node>,
}

impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new(0)],
        }
    }

    /// x を n 個追加する  
    pub fn insert(&mut self, mut x: usize, n: usize) {
        if n == 0 {
            return;
        }

        let mut v = 0;
        let mut d = W;
        while {
            self.nodes[v].cnt += n;
            d != 0
        } {
            x &= !0 >> W - d;
            let c = x >> (d - 1) & 1;
            if let Some(link) = self.nodes[v].link[c] {
                let common_prefix = (W - (x ^ link.seq).leading_zeros() as u8).max(link.d);
                if common_prefix == link.d {
                    v = link.ch as _;
                    d = common_prefix;
                } else {
                    let u = link.ch as usize;
                    let w = self.new_node(self.nodes[u].cnt);
                    let nx = x & !((1 << common_prefix) - 1);
                    self.nodes[v].link[c] = Some(Link::new(w, nx, common_prefix));
                    let x2 = link.seq & (!0 >> W - common_prefix);
                    let c2 = link.seq >> (common_prefix - 1) & 1;
                    self.nodes[w].cnt = self.nodes[u].cnt;
                    self.nodes[w].link[c2] = Some(Link::new(u, x2, link.d));
                    v = w;
                    d = common_prefix;
                }
            } else {
                let u = self.new_node(0);
                self.nodes[v].link[c] = Some(Link::new(u, x, 0));
                v = u;
                d = 0;
            }
        }
    }

    /// x を n 個削除する  
    /// n 個以上存在しない場合の動作は未定義
    pub fn remove(&mut self, mut x: usize, n: usize) {
        if n == 0 {
            return;
        }

        let mut v = 0;
        let mut d = W;
        while {
            self.nodes[v].cnt -= n;
            d != 0
        } {
            x &= !0 >> W - d;
            let c = x >> (d - 1) & 1;
            let link = self.nodes[v].link[c].unwrap();
            v = link.ch as _;
            d = link.d;
        }
    }

    /// x の個数を返す  
    pub fn count(&self, mut x: usize) -> usize {
        let mut v = 0;
        let mut d = W;
        while d > 0 {
            x &= !0 >> W - d;
            let c = x >> (d - 1) & 1;
            if let Some(link) = self.nodes[v].link[c] {
                let common_prefix = (W - (x ^ link.seq).leading_zeros() as u8).max(link.d);
                if common_prefix == link.d {
                    v = link.ch as _;
                    d = link.d;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        self.nodes[v].cnt
    }

    /// 多重集合に含まれる要素数を返す
    pub fn count_all(&self) -> usize {
        self.nodes[0].cnt
    }

    /// min_{x ∈ S} (x ^ xor) を返す
    pub fn min(&self, xor: usize) -> Option<usize> {
        if self.nodes[0].cnt == 0 {
            return None;
        }
        let mut v = 0;
        let mut d = W;
        let mut res = xor;
        while d > 0 {
            let c = xor >> (d - 1) & 1;
            let link = self.nodes[v].link[c]
                .filter(|link| self.nodes[link.ch as usize].cnt > 0)
                .or(self.nodes[v].link[c ^ 1])
                .unwrap();
            res ^= link.seq;
            v = link.ch as _;
            d = link.d;
        }
        Some(res)
    }

    /// max_{x ∈ S} (x ^ xor) を返す
    pub fn max(&self, xor: usize) -> Option<usize> {
        self.min(xor ^ !0).map(|x| x ^ !0)
    }

    /// {y ∈ S | y ^ xor ∈ range} の個数を返す
    pub fn count_range(&self, range: impl RangeBounds<usize>, xor: usize) -> usize {
        let l = match range.start_bound() {
            Bound::Unbounded => self.count_all(),
            Bound::Included(&start) => self.count_geq(start, xor),
            Bound::Excluded(&start) => self.count_geq(start + 1, xor),
        };
        let r = match range.end_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&end) => end.checked_add(1).map_or(0, |end| self.count_geq(end, xor)),
            Bound::Excluded(&end) => self.count_geq(end, xor),
        };
        l - r
    }

    /// x ^ xor で k 番目に小さい値を返す
    pub fn kth_smallest(&self, mut k: usize, xor: usize) -> Option<usize> {
        if k >= self.count_all() {
            return None;
        }
        let mut v = 0;
        let mut d = W;
        let mut res = xor;
        while d > 0 {
            let c = xor >> (d - 1) & 1;
            let link = self.nodes[v].link[c]
                .filter(|link| {
                    let cnt = self.nodes[link.ch as usize].cnt;
                    if k >= cnt {
                        k -= cnt;
                        false
                    } else {
                        true
                    }
                })
                .or(self.nodes[v].link[c ^ 1])
                .unwrap();
            res ^= link.seq;
            v = link.ch as _;
            d = link.d;
        }
        Some(res)
    }

    /// x ^ xor で k 番目に大きい値を返す
    pub fn kth_largest(&self, k: usize, xor: usize) -> Option<usize> {
        if k >= self.count_all() {
            return None;
        }
        self.kth_smallest(self.count_all() - k - 1, xor)
    }

    fn new_node(&mut self, cnt: usize) -> usize {
        self.nodes.push(Node::new(cnt));
        self.nodes.len() - 1
    }

    /// {y ∈ S | y ^ xor ≥ x} の個数を返す
    fn count_geq(&self, mut x: usize, mut xor: usize) -> usize {
        let mut v = 0;
        let mut d = W;
        let mut res = 0;
        while d > 0 {
            x &= !0 >> W - d;
            xor &= !0 >> W - d;
            let c = (x ^ xor) >> (d - 1) & 1;
            if x >> (d - 1) & 1 == 0 {
                if let Some(link) = self.nodes[v].link[c ^ 1] {
                    res += self.nodes[link.ch as usize].cnt;
                }
            }
            if let Some(link) = self.nodes[v].link[c] {
                let mask = !((1 << link.d) - 1);
                if (x ^ xor) & mask == link.seq {
                    v = link.ch as usize;
                    d = link.d;
                } else {
                    if x < link.seq ^ xor {
                        res += self.nodes[link.ch as usize].cnt;
                    }
                    return res;
                }
            } else {
                return res;
            }
        }
        res += self.nodes[v].cnt;
        res
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    cnt: usize,
    link: [Option<Link>; 2],
}

impl Node {
    fn new(cnt: usize) -> Self {
        Self {
            cnt,
            link: [None; 2],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Link {
    ch: u32,
    seq: usize,
    d: u8,
    _f: bool, // これをつけると niche optimization が効いてメモリが小さくなる
}

impl Link {
    fn new(ch: usize, seq: usize, d: u8) -> Self {
        Self {
            ch: ch as u32,
            seq,
            d,
            _f: false,
        }
    }
}

#[test]
fn test() {
    let st = std::collections::BTreeSet::from([1, 2, 10, 4, 5]);
    let mut bt = BinaryTrie::new();
    for &x in &st {
        bt.insert(x, 1);
    }
    for s in 0..=10 {
        eprintln!("{}", bt.count_geq(s, 0));
    }
    for s in 0..=10 {
        for t in s..=10 {
            let c = st.range(s..=t).count();
            let d = bt.count_range(s..=t, 0);
            assert_eq!(c, d, "{} {}", s, t);
        }
    }
}
