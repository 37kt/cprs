// verification-helper: PROBLEM https://yukicoder.me/problems/no/952

use monotone_minima::monotone_minima;
use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    let mut dp = vec![INF; n + 2];
    dp[0] = 0;
    let mut res = vec![0; n];
    for k in 1..=n {
        let f = |i: usize, j: usize| {
            if i > j {
                dp[j] + (b[i - 1] - b[j]).pow(2)
            } else {
                INF
            }
        };
        let select = |i, j, k| f(i, j) > f(i, k);
        let mn = monotone_minima(n + 2, n + 2, select);
        let mut dpn = vec![INF; n + 2];
        for i in k..=n + 1 {
            dpn[i] = f(i, mn[i]);
        }
        dp = dpn;
        res[n - k] = dp[n + 1];
    }
    for i in 0..n {
        println!("{}", res[i]);
    }
}
