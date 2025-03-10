// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum

use algebraic_structure::magma::AddOperator;
use fenwick_tree::FenwickTree;
use proconio::fastout;
use proconio::input;
use proconio::read_value;
use wavelet_matrix::WaveletMatrixImpl;

enum Query {
    Add(usize, i64),
    Sum(u32, u32, u32, u32),
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut qs = vec![];
    let mut ps = vec![];
    let mut ws = vec![];
    for qi in 0..n + q {
        let t = if qi < n { 0 } else { read_value!(usize) };
        if t == 0 {
            input! {
                x: u32,
                y: u32,
                w: i64,
            }
            let i = ps.len();
            ps.push((x, y));
            if qi < n {
                ws.push(w);
            } else {
                qs.push(Query::Add(i, w));
                ws.push(0);
            }
        } else {
            input! {
                xl: u32,
                yl: u32,
                xr: u32,
                yr: u32,
            }
            qs.push(Query::Sum(xl, yl, xr, yr));
        }
    }

    let (wm, mut bits) = WaveletMatrixImpl::<u32, u32, false, false, true>::new(&ps, |idx| {
        FenwickTree::<AddOperator<i64>>::from_iter(idx.iter().map(|&i| ws[i]))
    });

    for q in qs {
        match q {
            Query::Add(i, w) => {
                wm.access_with(i, |d, i| {
                    bits[d].add(i, w);
                });
            }
            Query::Sum(xl, yl, xr, yr) => {
                let mut s = 0;
                wm.count_with(xl..xr, yl..yr, |d, range, inv| {
                    let t = bits[d].fold(range);
                    s += if inv { -t } else { t };
                });
                println!("{}", s);
            }
        }
    }
}
