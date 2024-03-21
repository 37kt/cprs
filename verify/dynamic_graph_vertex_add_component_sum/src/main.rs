// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_graph_vertex_add_component_sum

use algebraic::{algebra, monoid};
use offline_dynamic_connectivity::{OfflineDynamicConnectivity, RollbackUnionFindTrait};
use proconio::input;
use rollback_union_find_component_sum::RollbackUnionFindComponentSum;

algebra!(M, usize);
monoid!(M, 0, |a, b| a + b);

struct UF {
    uf: RollbackUnionFindComponentSum<M>,
}

impl RollbackUnionFindTrait for UF {
    type Query = usize;

    fn add_edge(&mut self, x: usize, y: usize) {
        self.uf.merge(x, y);
    }

    fn undo(&mut self) {
        self.uf.undo();
    }

    fn get(&mut self, query: Self::Query) {
        println!("{}", self.uf.sum(query));
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    a.resize(n + q, 0);
    let mut qs = vec![];
    for i in 0..q {
        input! {
            t: usize,
        }
        if t != 3 {
            input! {
                u: usize,
                v: usize,
            }
            qs.push((t, u, v));
            if t == 2 {
                a[n + i] = v;
            }
        } else {
            input! {
                v: usize,
            }
            qs.push((t, v, 0));
        }
    }
    let mut dc = OfflineDynamicConnectivity::new(UF {
        uf: RollbackUnionFindComponentSum::new(n + q, &a),
    });
    for i in 0..q {
        let (t, u, v) = qs[i];
        match t {
            0 => dc.add_edge(u, v),
            1 => dc.remove_edge(u, v),
            2 => dc.add_edge(u, n + i),
            3 => dc.get(u),
            _ => unreachable!(),
        }
    }
    dc.run();
}
