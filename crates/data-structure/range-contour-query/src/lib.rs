use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;

/// 木の等高線クエリ  
/// seq\[v\] は、重心分解の過程を表す木の部分木 v に含まれる頂点を、元の木における BFS 順に並べたものである。  
/// 各 seq\[v\] について Segment Tree を構築することで、 1 点更新区間取得クエリもしくは区間更新 1 点取得クエリを処理できる。
#[derive(Clone)]
pub struct RangeContourQuery {
    n: usize,
    sz: Vec<usize>,
    ctr: Vec<usize>,
    pub seq: Vec<Vec<usize>>,
    sep: Vec<Vec<usize>>,
    head: Vec<usize>,
    tail: Vec<usize>,
    link: Vec<usize>,
    cur: usize,
    par: Vec<usize>,
    ch: Vec<[usize; 2]>,
    cv: Vec<usize>,
    pos: Vec<Vec<(usize, usize)>>,
    hld: HeavyLightDecomposition,
}

impl RangeContourQuery {
    /// 木の等高線クエリを構築する。
    pub fn new(g: &UndirectedGraph<(), ()>) -> Self {
        let mut h = vec![vec![]; g.len()];
        for v in 0..g.len() {
            for &(u, _) in &g[v] {
                h[v].push(u);
            }
        }

        let n = g.len();
        let mut rcq = Self {
            n,
            sz: vec![0; n * 3],
            ctr: vec![!0; n * 3],
            seq: vec![vec![]; n * 3],
            sep: vec![vec![]; n * 3],
            head: vec![!0; n * 3],
            tail: vec![!0; n * 3],
            link: vec![!0; n * 3],
            cur: n,
            par: vec![!0; n * 3],
            ch: vec![[!0; 2]; n * 3],
            cv: vec![!0; n * 3],
            pos: vec![vec![]; n],
            hld: HeavyLightDecomposition::new(&g),
        };
        for i in 0..n {
            rcq.head[i] = i;
            rcq.tail[i] = i;
            rcq.cv[i] = i;
        }
        rcq.rec(0, n, &mut h);
        rcq.sz.truncate(rcq.cur);
        rcq.ctr.truncate(rcq.cur);
        rcq.seq.truncate(rcq.cur);
        rcq.sep.truncate(rcq.cur);
        rcq.head.truncate(rcq.cur);
        rcq.tail.truncate(rcq.cur);
        rcq.link.truncate(rcq.cur);
        rcq.par.truncate(rcq.cur);
        rcq.ch.truncate(rcq.cur);
        rcq.cv.truncate(rcq.cur);
        for i in 0..rcq.cur {
            for j in 0..rcq.seq[i].len() {
                rcq.pos[rcq.seq[i][j]].push((i, j));
            }
        }
        rcq
    }

    fn search_centroid(&mut self, v: usize, p: usize, n: usize, g: &[Vec<usize>], c: &mut usize) {
        self.sz[v] = 1;
        for &u in &g[v] {
            if u == p {
                continue;
            }
            self.search_centroid(u, v, n, g, c);
            if u == *c {
                self.sz[v] = n - self.sz[*c];
                break;
            }
            self.sz[v] += self.sz[u];
        }
        if *c == !0 && self.sz[v] * 2 > n {
            *c = v;
        }
    }

    fn build_seq(&mut self, h: usize, w: usize, c: usize, g: &[Vec<usize>]) {
        if c < self.n {
            self.seq[w].push(c);
            self.sep[w] = vec![0, 1];
        }
        let mut q = VecDeque::new();
        let mut v = h;
        while v != !0 {
            q.push_back((v, !0, 1));
            v = self.link[v];
        }
        while let Some((v, p, d)) = q.pop_front() {
            self.seq[w].push(v);
            if self.sep[w].len() <= d + 1 {
                self.sep[w].resize(d + 2, 0);
            }
            self.sep[w][d + 1] += 1;
            for &u in &g[v] {
                if u == p {
                    continue;
                }
                q.push_back((u, v, d + 1));
            }
        }
        for i in 1..self.sep[w].len() {
            self.sep[w][i] += self.sep[w][i - 1];
        }
    }

    fn rec(&mut self, r: usize, n: usize, g: &mut [Vec<usize>]) -> usize {
        let mut c = !0;
        self.search_centroid(r, !0, n, g, &mut c);

        for i in 0..g[c].len() {
            let u = g[c][i];
            let tmp = self.sz[u];
            g[u].retain(|&w| w != c);
            self.ctr[u] = self.rec(u, tmp, g);
            self.sz[u] = tmp;
        }

        if g[c].len() == 0 {
            self.build_seq(!0, c, c, g);
        } else if g[c].len() == 1 {
            self.build_seq(self.head[g[c][0]], c, c, g);
            self.par[self.ctr[g[c][0]]] = c;
            self.ch[c][0] = self.ctr[g[c][0]];
        } else {
            let mut pq = BinaryHeap::new();
            for &u in &g[c] {
                self.link[u] = !0;
                let new = self.cur;
                self.cur += 1;
                self.build_seq(self.head[u], new, !0, g);
                self.sz[new] = self.sz[u];
                self.ctr[new] = new;
                self.head[new] = self.head[u];
                self.tail[new] = self.tail[u];
                self.ch[new][0] = self.ctr[u];
                self.par[self.ctr[u]] = new;
                self.cv[new] = c;
                pq.push(Reverse((self.sz[new], new)));
            }
            while pq.len() >= 2 {
                let Reverse((_, u)) = pq.pop().unwrap();
                let Reverse((_, v)) = pq.pop().unwrap();
                if pq.is_empty() {
                    self.link[self.tail[u]] = self.head[v];
                    self.tail[u] = self.tail[v];
                    self.ch[c] = [self.ctr[u], self.ctr[v]];
                    self.par[self.ctr[u]] = c;
                    self.par[self.ctr[v]] = c;
                    self.build_seq(self.head[u], c, c, g);
                } else {
                    let new = self.cur;
                    self.cur += 1;
                    self.sz[new] = self.sz[u] + self.sz[v];
                    self.ch[new] = [self.ctr[u], self.ctr[v]];
                    self.par[self.ctr[u]] = new;
                    self.par[self.ctr[v]] = new;
                    self.ctr[new] = new;
                    self.head[new] = self.head[u];
                    self.tail[new] = self.tail[v];
                    self.link[self.tail[u]] = self.head[v];
                    self.cv[new] = c;
                    self.build_seq(self.head[new], new, !0, g);
                    pq.push(Reverse((self.sz[new], new)));
                }
            }
        }

        for i in 0..g[c].len() {
            let u = g[c][i];
            g[u].push(c);
        }

        c
    }

    /// seq に含まれる 頂点 v の index を取得する。
    pub fn point(&self, v: usize) -> Vec<(usize, usize)> {
        self.pos[v].clone()
    }

    /// 頂点 v からの距離が l 以上 r 未満である頂点を列挙する。  
    /// 出力: (v_i, l_i, r_i) のタプルの列  
    /// seq\[v_i\]\[l_i..r_i\] は、重複なくこれを列挙している。
    pub fn range(&self, mut v: usize, l: usize, r: usize) -> Vec<(usize, usize, usize)> {
        let mut res = vec![];
        if l >= r {
            return res;
        }
        let f = |v: usize, i: usize| self.sep[v][i.min(self.sep[v].len() - 1)];
        res.push((v, f(v, l), f(v, r)));
        let vv = v;
        while self.par[v] != !0 {
            let p = self.par[v];
            let d = self.hld.dist(self.cv[vv], self.cv[p]);
            if p < self.n && l <= d && d < r {
                res.push((p, 0, 1));
            }
            for &u in &self.ch[p] {
                if u != v && u != !0 {
                    let d = self.hld.dist(self.cv[vv], self.cv[u]);
                    let l = l.saturating_sub(d);
                    let r = r.saturating_sub(d);
                    res.push((u, f(u, l), f(u, r)));
                }
            }
            v = p;
        }
        res.retain(|&(_, l, r)| l < r);
        res
    }
}
