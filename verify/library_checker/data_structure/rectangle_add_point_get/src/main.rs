// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_add_point_get

use algebraic_structure::magma::AddOperator;
use dual_segment_tree::DualSegmentTree;
use proconio::fastout;
use proconio::input;
use proconio::read_value;
use wavelet_matrix::WaveletMatrixImpl;

enum Query {
    Add(u32, u32, u32, u32, i64),
    Get(usize),
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut qs = Vec::with_capacity(n + q);
    let mut ps = vec![];
    let mut get_count = 0;
    for qi in 0..n + q {
        let t = if qi < n { 0 } else { read_value!(usize) };
        if t == 0 {
            input! {
                xl: u32,
                yl: u32,
                xr: u32,
                yr: u32,
                w: i64,
            }
            qs.push(Query::Add(xl, yl, xr, yr, w));
        } else {
            input! {
                x: u32,
                y: u32,
            }
            ps.push((x, y));
            qs.push(Query::Get(get_count));
            get_count += 1;
        }
    }

    let (wm, mut segs) = WaveletMatrixImpl::<u32, u32, false, false, true>::new(&ps, |_| {
        DualSegmentTree::<AddOperator<i64>>::new(ps.len())
    });

    for q in qs {
        match q {
            Query::Add(xl, yl, xr, yr, w) => {
                wm.count_with(xl..xr, yl..yr, |d, range, inv| {
                    let w = if inv { -w } else { w };
                    segs[d].apply_range(range, w);
                });
            }
            Query::Get(i) => {
                let mut s = 0;
                wm.access_with(i, |d, i| {
                    s += segs[d].get(i);
                });
                println!("{}", s);
            }
        }
    }
}
