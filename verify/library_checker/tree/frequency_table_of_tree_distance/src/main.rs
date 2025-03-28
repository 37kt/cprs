// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance

use centroid_decomposition::CentroidDecomposition;
use convolution::convolution_ntt_friendly;
use graph::{GraphBuilder, UndirectedGraph};
use proconio::{fastout, input};
use static_modint::StaticModInt;

const P1: u32 = 998_244_353;
const P2: u32 = 1_004_535_809;
type Fp1 = StaticModInt<P1>;
type Fp2 = StaticModInt<P2>;

#[fastout]
fn main() {
    let m1inv_fp2 = Fp2::from_raw(P1).recip();

    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_edges(n, ab);
    let mut res = vec![0; n];
    let mut depth = vec![0; n];
    CentroidDecomposition::solve(&g, |cd| {
        depth[cd.root] = 0;
        let mut f1 = vec![vec![]; 2];
        let mut f2 = vec![vec![]; 2];
        for (i, &v) in cd.vs.iter().enumerate() {
            depth[v] = depth[cd.par[v]] + 1;
            let c = if i < cd.mid { 0 } else { 1 };
            if f1[c].len() <= depth[v] {
                f1[c].resize(depth[v] + 1, Fp1::from_raw(0));
                f2[c].resize(depth[v] + 1, Fp2::from_raw(0));
            }
            f1[c][depth[v]] += 1;
            f2[c][depth[v]] += 1;
        }
        let g1 = convolution_ntt_friendly(&f1[0], &f1[1]);
        let g2 = convolution_ntt_friendly(&f2[0], &f2[1]);
        for (i, (e1, e2)) in g1.into_iter().zip(g2).enumerate() {
            let x1 = e1;
            let x2 = (e2 - Fp2::from_raw(e1.val())) * m1inv_fp2;
            res[i] += (x1.val() as u64) + (x2.val() as u64) * (P1 as u64);
        }
    });
    res[1] += (n - 1) as u64;

    for x in &res[1..] {
        print!("{} ", x);
    }
    println!();
}
