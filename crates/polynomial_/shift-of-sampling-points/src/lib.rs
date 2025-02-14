use formal_power_series::fps;
use modint::StaticModInt;

/// f(0), f(1), ..., f(n-1) から f(c+i), f(c+1+i), ..., f(c+m-1) を求める
pub fn shift_of_sampling_points<const P: u32>(
    y: &[StaticModInt<P>],
    c: StaticModInt<P>,
    m: usize,
) -> Vec<StaticModInt<P>> {
    let c = c.val() as usize;
    let n = y.len();
    if c < n {
        let mut res = y[c..].to_vec();
        res.truncate(m);
        if n < c + m {
            let mut suf = shift_of_sampling_points(y, n.into(), m - res.len());
            res.append(&mut suf);
        }
        return res;
    }

    if c + m > P as usize {
        let mut pre = shift_of_sampling_points(y, c.into(), P as usize - c);
        let mut suf = shift_of_sampling_points(y, 0.into(), m - pre.len());
        pre.append(&mut suf);
        return pre;
    }

    let mut fact_inv = vec![StaticModInt::new(1); n];
    let mut d = fps![1; n];
    for i in 2..n {
        fact_inv[n - 1] *= i;
    }
    fact_inv[n - 1] = fact_inv[n - 1].inv();
    for i in (1..n).rev() {
        fact_inv[i - 1] = fact_inv[i] * i;
    }
    for i in 0..n {
        d[i] = fact_inv[i] * fact_inv[n - 1 - i] * y[i];
        if (n - 1 - i) & 1 != 0 {
            d[i] = -d[i];
        }
    }

    let mut h = fps![0; m + n - 1];
    for i in 0..m + n - 1 {
        h[i] = StaticModInt::new(c + 1 + i - n).inv();
    }

    let dh = d * &h;

    let mut res = fps![0; m];
    let mut cur = StaticModInt::new(c);
    for i in 1..n {
        cur *= c - i;
    }
    for i in 0..m {
        res[i] = cur * dh[n - 1 + i];
        cur *= c + i + 1;
        cur *= h[i];
    }
    res.0
}
