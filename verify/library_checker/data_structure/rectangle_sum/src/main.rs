// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum

use proconio::fastout;
use proconio::input;
use std::ops::Range;
use wavelet_matrix::WaveletMatrixImpl;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xyw: [((u32, u32), i64); n],
    }

    let (xy, w): (Vec<_>, Vec<_>) = xyw.into_iter().unzip();

    let (wm, css) = WaveletMatrixImpl::<_, _, false, false, true>::new(&xy, |idx| {
        std::iter::once(0)
            .chain(idx.iter().scan(0, |acc, &i| {
                *acc += w[i];
                Some(*acc)
            }))
            .collect::<Vec<_>>()
    });

    for _ in 0..q {
        input! {
            xl: u32,
            yl: u32,
            xr: u32,
            yr: u32,
        }
        let mut s = 0;
        wm.count_with(xl..xr, yl..yr, |d, rng, inv| {
            let Range { start: l, end: r } = rng;
            let t = css[d][r] - css[d][l];
            s += if inv { -t } else { t };
        });
        println!("{}", s);
    }
}
