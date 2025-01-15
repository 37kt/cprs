use std::{cmp::Reverse, collections::BinaryHeap};

use csr_array::CSRArray;
use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;

pub struct StaticTopTree {
    pub hld: HeavyLightDecomposition,
    pub par: Vec<usize>,
    pub lch: Vec<usize>,
    pub rch: Vec<usize>,
    pub top: Vec<usize>,
    pub bottom: Vec<usize>,
    pub is_compress: Vec<bool>,
}

impl StaticTopTree {
    pub fn new<V: Clone, E: Clone>(g: &UndirectedGraph<V, E>, root: usize) -> Self {
        let n = g.len();
        let mut stt = Self {
            hld: HeavyLightDecomposition::new(g, root),
            par: Vec::with_capacity(n * 2 - 1),
            lch: Vec::with_capacity(n * 2 - 1),
            rch: Vec::with_capacity(n * 2 - 1),
            top: Vec::with_capacity(n * 2 - 1),
            bottom: Vec::with_capacity(n * 2 - 1),
            is_compress: Vec::with_capacity(n * 2 - 1),
        };
        for v in 0..n {
            stt.par.push(!0);
            stt.lch.push(!0);
            stt.rch.push(!0);
            stt.top.push(stt.hld.parent(v));
            stt.bottom.push(v);
            stt.is_compress.push(false);
        }
        let children = stt.hld.children();
        stt.build_dfs(root, &children);
        assert!(stt.par.len() == n * 2 - 1);
        stt
    }

    // (height, index)
    fn build_dfs(&mut self, v: usize, children: &CSRArray<usize>) -> (usize, usize) {
        assert!(self.hld.leader(v) == v);

        let path = {
            let mut path = vec![v];
            let mut v = v;
            while children[v].len() > 0 {
                v = children[v][0];
                path.push(v);
            }
            path
        };
        let mut stack = vec![(0, v)];
        for i in 1..path.len() {
            let mut pq = BinaryHeap::new();
            let mut v = path[i];
            pq.push(Reverse((0, v)));
            for &c in children[path[i - 1]].iter().skip(1) {
                pq.push(Reverse(self.build_dfs(c, children)));
            }
            while pq.len() >= 2 {
                let Reverse((h1, mut v1)) = pq.pop().unwrap();
                let Reverse((h2, mut v2)) = pq.pop().unwrap();
                if v2 == v {
                    std::mem::swap(&mut v1, &mut v2);
                }
                let v3 = self.new_node(v1, v2, self.top[v1], self.bottom[v2], false);
                if v == v1 {
                    v = v3;
                }
                pq.push(Reverse((h1.max(h2) + 1, v3)));
            }
            stack.push(pq.pop().unwrap().0);

            loop {
                let n = stack.len();
                if n >= 3 && (stack[n - 3].0 == stack[n - 2].0 || stack[n - 3].0 <= stack[n - 1].0)
                {
                    let hv = stack.pop().unwrap();
                    self.merge(&mut stack);
                    stack.push(hv);
                } else if n >= 2 && stack[n - 2].0 <= stack[n - 1].0 {
                    self.merge(&mut stack);
                } else {
                    break;
                }
            }
        }

        while stack.len() >= 2 {
            self.merge(&mut stack);
        }
        stack.pop().unwrap()
    }

    fn new_node(
        &mut self,
        lch: usize,
        rch: usize,
        top: usize,
        bottom: usize,
        is_compress: bool,
    ) -> usize {
        let v = self.par.len();
        self.par[lch] = v;
        self.par[rch] = v;
        self.par.push(!0);
        self.lch.push(lch);
        self.rch.push(rch);
        self.top.push(top);
        self.bottom.push(bottom);
        self.is_compress.push(is_compress);
        v
    }

    fn merge(&mut self, stack: &mut Vec<(usize, usize)>) {
        let (h2, v2) = stack.pop().unwrap();
        let (h1, v1) = stack.pop().unwrap();
        stack.push((
            h1.max(h2) + 1,
            self.new_node(v1, v2, self.top[v1], self.bottom[v2], true),
        ));
    }
}
