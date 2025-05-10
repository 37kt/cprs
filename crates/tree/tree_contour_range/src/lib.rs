use std::ops::{Range, RangeBounds};

use as_half_open_range::AsHalfOpenRange;
use centroid_decomposition::CentroidDecomposition;
use csr_array::CsrArray;
use graph::Edge;

#[derive(Clone)]
pub struct TreeContourRange {
    pos: CsrArray<usize>,
    comp: Vec<usize>,
    depth: Vec<usize>,
    sep: CsrArray<usize>,
}

impl TreeContourRange {
    pub fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {
        let n = g.len();

        let mut pos = vec![];
        let mut p = 0;

        let mut comp_id = 0;

        let mut comp = vec![];
        let mut depth = vec![];

        let mut d = vec![0; n];
        let mut used = vec![false; n];

        let mut cnt = vec![0; n + 2];

        let mut sep = vec![];

        CentroidDecomposition::solve(g, |cd| {
            d[cd.root] = 0;
            for &v in cd.vs {
                d[v] = d[cd.par[v]] + 1;
            }

            for vs in [&[cd.root], &cd.vs[..cd.mid], &cd.vs[cd.mid..]] {
                let mut d_max = 0;
                for &v in vs {
                    if !used[v] {
                        d_max = d[v] + 1;
                        cnt[d[v] + 1] = 1;
                    }
                }

                for i in 0..=d_max {
                    if cnt[i + 1] == 1 {
                        comp.push(comp_id);
                        depth.push(i);
                    }
                }

                cnt[0] = p;
                for i in 0..=d_max {
                    cnt[i + 1] += cnt[i];
                    sep.push((comp_id, cnt[i]));
                }
                p = cnt[d_max];

                for &v in vs {
                    if !used[v] {
                        pos.push((v, cnt[d[v]]));
                    }
                }

                cnt[..=d_max + 1].fill(0);
                comp_id += 1;
            }

            used[cd.root] = true;
        });

        Self {
            pos: CsrArray::new(n, pos),
            comp,
            depth,
            sep: CsrArray::new(comp_id, sep),
        }
    }

    pub fn point(&self, v: usize) -> impl Iterator<Item = usize> + '_ {
        self.pos[v].iter().copied()
    }

    pub fn range(
        &self,
        v: usize,
        range: impl RangeBounds<usize>,
    ) -> impl Iterator<Item = Range<usize>> + '_ {
        const COMPS: [&[usize]; 3] = [&[1, 2], &[!0, 1], &[!1, !0]];
        let (l, r) = range.as_half_open_range(0, 1 << 63);
        self.pos[v].iter().flat_map(move |&p| {
            let comp = self.comp[p];
            let depth = self.depth[p];
            COMPS[comp % 3]
                .iter()
                .map(move |&c| c.wrapping_add(comp))
                .filter_map(move |c| {
                    let l = l.saturating_sub(depth);
                    let r = r.saturating_sub(depth);
                    let l = self.sep[c][l.min(self.sep[c].len() - 1)];
                    let r = self.sep[c][r.min(self.sep[c].len() - 1)];
                    (l < r).then_some(l..r)
                })
        })
    }

    pub fn len(&self) -> usize {
        if self.sep.is_empty() {
            0
        } else {
            *self.sep[self.sep.len() - 1].last().unwrap()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
