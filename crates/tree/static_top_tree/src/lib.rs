use std::{cmp::Reverse, collections::BinaryHeap};

use csr_array::CsrArray;
use heavy_light_decomposition::HeavyLightDecomposition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SttNodeType {
    Value,
    Rake,
    Compress,
}

#[derive(Debug, Clone, Copy)]
pub struct SttNode {
    pub ty: SttNodeType,
    pub par: usize,
    pub lch: usize,
    pub rch: usize,
}

#[derive(Clone)]
pub struct StaticTopTree {
    pub nodes: Vec<SttNode>,
}

impl StaticTopTree {
    pub fn new(hld: &HeavyLightDecomposition) -> Self {
        let n = hld.len();
        let root = hld.root();
        let mut nodes = Vec::with_capacity(n * 2 - 1);
        nodes.extend(
            std::iter::repeat(SttNode {
                ty: SttNodeType::Value,
                par: !0,
                lch: !0,
                rch: !0,
            })
            .take(n),
        );
        let mut stt = Self { nodes };
        let children = hld.children();
        stt.dfs_build(root, hld, &children);
        stt
    }

    fn dfs_build(
        &mut self,
        v: usize,
        hld: &HeavyLightDecomposition,
        children: &CsrArray<usize>,
    ) -> (usize, usize) {
        assert!(hld.head(v) == v);

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
                pq.push(Reverse(self.dfs_build(c, hld, children)));
            }
            while pq.len() >= 2 {
                let Reverse((h1, mut v1)) = pq.pop().unwrap();
                let Reverse((h2, mut v2)) = pq.pop().unwrap();
                if v2 == v {
                    std::mem::swap(&mut v1, &mut v2);
                }
                let v3 = self.new_node(SttNodeType::Rake, v1, v2);
                if v == v1 {
                    v = v3;
                }
                pq.push(Reverse((h1.max(h2) + 1, v3)));
            }
            stack.push(pq.pop().unwrap().0);

            loop {
                if matches!(&stack[..], &[.., (h0, _), (h1, _), (h2, _)] if (h0 == h1 || h0 <= h2))
                {
                    let x = stack.pop().unwrap();
                    self.compress_last_two(&mut stack);
                    stack.push(x);
                } else if matches!(&stack[..], &[.., (h0, _), (h1, _)] if h0 <= h1) {
                    self.compress_last_two(&mut stack);
                } else {
                    break;
                }
            }
        }

        while stack.len() >= 2 {
            self.compress_last_two(&mut stack);
        }
        stack.pop().unwrap()
    }

    fn new_node(&mut self, ty: SttNodeType, lch: usize, rch: usize) -> usize {
        let v = self.nodes.len();
        self.nodes.push(SttNode {
            ty,
            par: !0,
            lch,
            rch,
        });
        self.nodes[lch].par = v;
        self.nodes[rch].par = v;
        v
    }

    fn compress_last_two(&mut self, stack: &mut Vec<(usize, usize)>) {
        let (h2, v2) = stack.pop().unwrap();
        let (h1, v1) = stack.pop().unwrap();
        stack.push((h1.max(h2) + 1, self.new_node(SttNodeType::Compress, v1, v2)));
    }
}
