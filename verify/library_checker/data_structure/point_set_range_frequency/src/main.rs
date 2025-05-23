// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_frequency

use algebraic_structure::magma::AddOperator;
use fenwick_tree::FenwickTree;
use proconio::fastout;
use proconio::input;
use wavelet_matrix::WaveletMatrix2D;

enum Query {
    Set(usize, usize),
    Query(usize, usize, usize),
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut z = vec![];
    for (i, &x) in a.iter().enumerate() {
        z.push((i, x));
    }
    let mut qs = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                k: usize,
                v: usize,
            }
            z.push((k, v));
            qs.push(Query::Set(k, v));
        } else {
            input! {
                l: usize,
                r: usize,
                x: usize,
            }
            qs.push(Query::Query(l, r, x));
        }
    }

    let m = z.len();
    let (wm, mut ft) =
        WaveletMatrix2D::<usize, usize, true, false, true>::new_2d_with_containers(z, |ord| {
            FenwickTree::<AddOperator<i32>>::from_fn(m, |i| if ord[i] < n { 1 } else { 0 })
        });

    let mut b = (0..n).collect::<Vec<_>>();
    let mut i = n;
    for q in qs {
        match q {
            Query::Set(k, _) => {
                wm.access_with(b[k], |i, j| {
                    ft[i].add(j, -1);
                });
                b[k] = i;
                wm.access_with(b[k], |i, j| {
                    ft[i].add(j, 1);
                });
                i += 1;
            }
            Query::Query(l, r, x) => {
                let mut res = 0;
                wm.count_with(l..r, x..=x, |i, range, inv| {
                    if inv {
                        res -= ft[i].fold(range);
                    } else {
                        res += ft[i].fold(range);
                    }
                });
                println!("{}", res);
            }
        }
    }
}
