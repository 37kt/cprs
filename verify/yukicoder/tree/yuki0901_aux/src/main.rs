// verification-helper: PROBLEM https://yukicoder.me/problems/no/901

use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, i64); n - 1],
        q: usize,
        vs: [[usize]; q],
    }

    let hld = HeavyLightDecomposition::from_edges(&uvw, 0);
    let mut ws = vec![0; n - 1];
    for &(u, v, w) in &uvw {
        ws[hld.edge_index(u, v)] = w;
    }

    let mut sum_ws = vec![0; n];
    for i in 0..n - 1 {
        sum_ws[i + 1] = sum_ws[i] + ws[i];
    }

    for vs in vs {
        let (_, map) = hld.compress(&vs);
        let mut res = 0;
        for i in 0..map.len() {
            let s = map[i];
            let t = map[(i + 1) % map.len()];
            hld.path_query(s, t, false, |l, r, _| {
                res += sum_ws[r] - sum_ws[l];
            });
        }
        println!("{}", res / 2);
    }
}
