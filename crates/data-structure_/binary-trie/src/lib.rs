use std::ops::{Bound, RangeBounds};

/// Binary Trie  
/// 非負整数の多重集合を管理
#[derive(Clone)]
pub struct BinaryTrie<const B: usize> {
    xor: usize,
    nodes: Vec<Node>,
}

#[derive(Clone, Copy)]
struct Node {
    children: [usize; 2],
    count: usize,
}

impl Node {
    fn new() -> Self {
        Self {
            children: [!0; 2],
            count: 0,
        }
    }
}

impl<const B: usize> BinaryTrie<B> {
    /// 空の集合を作成
    pub fn new() -> Self {
        assert!(B < 64, "B は 64 未満である必要があります");
        Self {
            xor: 0,
            nodes: vec![Node::new()],
        }
    }

    /// 集合の要素数を返す
    pub fn len(&self) -> usize {
        self.nodes[0].count
    }

    /// 集合が空かどうかを返す
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 集合に要素 x を n 個追加
    pub fn insert(&mut self, x: usize, n: usize) {
        assert!(x < 1 << B, "x は 2^B 未満である必要があります");
        if n == 0 {
            return;
        }
        self.insert_(0, x, n, B);
    }

    /// 集合から要素 x を n 個削除
    pub fn remove(&mut self, x: usize, n: usize) {
        assert!(x < 1 << B, "x は 2^B 未満である必要があります");
        if n == 0 {
            return;
        }
        self.remove_(0, x, n, B);
    }

    /// 集合のすべての要素に対して xor を作用させる
    pub fn operate_xor(&mut self, x: usize) {
        self.xor ^= x;
    }

    /// 集合の最小値を返す
    pub fn min(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        Some(self.min_(0, B) ^ self.xor)
    }

    /// 集合の最大値を返す
    pub fn max(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        Some(self.max_(0, B) ^ self.xor)
    }

    /// 集合に要素 x が何個あるかを返す
    pub fn count(&self, x: usize) -> usize {
        assert!(x < 1 << B, "x は 2^B 未満である必要があります");
        let mut v = 0;
        for b in (0..B).rev() {
            let c = ((x ^ self.xor) >> b) & 1;
            let Some(u) = self.child(v, c) else {
                return 0;
            };
            v = u;
        }
        self.nodes[v].count
    }

    /// 集合の `range` に含まれる要素の個数を返す
    pub fn frequency(&self, range: impl RangeBounds<usize>) -> usize {
        let (l, r) = range_to_pair(range, 1 << B);
        self.lower_bound(r) - self.lower_bound(l)
    }

    /// 集合の `range` に含まれる k 番目に小さい要素を返す
    pub fn kth_smallest(&self, range: impl RangeBounds<usize>, k: usize) -> Option<usize> {
        let (l, r) = range_to_pair(range, 1 << B);
        let k = k + self.lower_bound(l);
        if k >= self.len() {
            return None;
        }
        let res = self.kth_smallest_(0, k, B) ^ self.xor;
        if res >= r {
            return None;
        }
        Some(res)
    }

    /// 集合の `range` に含まれる k 番目に大きい要素を返す
    pub fn kth_largest(&self, range: impl RangeBounds<usize>, k: usize) -> Option<usize> {
        let (l, r) = range_to_pair(range, 1 << B);
        let m = self.frequency(l..r);
        if k >= m {
            return None;
        }
        self.kth_smallest(l..r, m - k - 1)
    }

    /// 集合の `x` 未満の要素の個数を返す
    pub fn lower_bound(&self, x: usize) -> usize {
        if x >= 1 << B {
            return self.len();
        }
        self.lower_bound_(0, x, B)
    }

    /// 集合の `x` 以下の要素の個数を返す
    pub fn upper_bound(&self, x: usize) -> usize {
        self.lower_bound(x + 1)
    }

    fn insert_(&mut self, v: usize, x: usize, n: usize, b: usize) {
        self.nodes[v].count += n;
        if b == 0 {
            return;
        }
        let c = ((x ^ self.xor) >> b - 1) & 1;
        let u = self.add_child(v, c);
        self.insert_(u, x, n, b - 1);
        self.nodes[v].children[c] = u;
    }

    fn remove_(&mut self, v: usize, x: usize, n: usize, b: usize) {
        assert!(
            self.nodes[v].count >= n,
            "削除する要素数が現在の要素数を超えています"
        );

        self.nodes[v].count -= n;
        if b == 0 {
            return;
        }
        let c = ((x ^ self.xor) >> b - 1) & 1;
        let u = self
            .child(v, c)
            .expect("削除する要素数が現在の要素数を超えています");
        self.remove_(u, x, n, b - 1);
    }

    fn min_(&self, v: usize, b: usize) -> usize {
        if b == 0 {
            return 0;
        }
        let mut c = self.xor >> (b - 1) & 1;
        if self.child(v, c).map_or(0, |u| self.nodes[u].count) == 0 {
            c ^= 1;
        }
        self.min_(self.child(v, c).unwrap(), b - 1) | (c << b - 1)
    }

    fn max_(&self, v: usize, b: usize) -> usize {
        if b == 0 {
            return 0;
        }
        let mut c = (!self.xor) >> (b - 1) & 1;
        if self.child(v, c).map_or(0, |u| self.nodes[u].count) == 0 {
            c ^= 1;
        }
        self.max_(self.child(v, c).unwrap(), b - 1) | (c << b - 1)
    }

    fn kth_smallest_(&self, v: usize, k: usize, b: usize) -> usize {
        if b == 0 {
            return 0;
        }
        let c = (self.xor >> b - 1) & 1;
        let n0 = self.child(v, c).map_or(0, |u| self.nodes[u].count);
        if k < n0 {
            let u = self.child(v, c).unwrap();
            self.kth_smallest_(u, k, b - 1)
        } else {
            let u = self.child(v, c ^ 1).unwrap();
            self.kth_smallest_(u, k - n0, b - 1) | ((c ^ 1) << b - 1)
        }
    }

    fn lower_bound_(&self, v: usize, x: usize, b: usize) -> usize {
        if b == 0 {
            return 0;
        }
        let mut res = 0;
        if x >> (b - 1) & 1 == 1 {
            if let Some(u) = self.child(v, self.xor >> (b - 1) & 1) {
                res += self.nodes[u].count;
            }
        }
        let c = (self.xor ^ x) >> (b - 1) & 1;
        if let Some(u) = self.child(v, c) {
            res += self.lower_bound_(u, x, b - 1);
        }
        res
    }

    fn child(&self, v: usize, c: usize) -> Option<usize> {
        let u = self.nodes[v].children[c];
        if u == !0 {
            None
        } else {
            Some(u)
        }
    }

    fn add_node(&mut self) -> usize {
        let n = self.nodes.len();
        self.nodes.push(Node::new());
        n
    }

    fn add_child(&mut self, v: usize, c: usize) -> usize {
        self.child(v, c).unwrap_or_else(|| self.add_node())
    }
}

fn range_to_pair<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    let l = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Excluded(&l) => (l + 1).min(n),
        Bound::Included(&l) => l.min(n),
    };
    let r = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Excluded(&r) => r.min(n),
        Bound::Included(&r) => (r + 1).min(n),
    };
    assert!(l <= r);
    (l, r)
}
