use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

pub fn polynomial_interpolation<const P: u32>(
    xs: &[StaticModInt<P>],
    ys: &[StaticModInt<P>],
) -> FormalPowerSeries<P> {
    assert_eq!(xs.len(), ys.len());
    let m = xs.len();
    if m == 0 {
        return fps![];
    }
    let m2 = 1 << 64 - (m - 1).leading_zeros();
    let mut mul = vec![fps![1]; m2 + m2];
    for i in 0..m {
        mul[m2 + i] = fps![-xs[i], 1];
    }
    for i in (1..m2).rev() {
        mul[i] = &mul[i << 1 | 0] * &mul[i << 1 | 1];
    }
    let mut g = vec![fps![]; m2 + m2];
    g[1] = mul[1].differential() % &mul[1];
    for i in 2..m2 + m {
        g[i] = &g[i >> 1] % &mul[i];
    }
    for i in 0..m {
        g[m2 + i] = fps![ys[i] / g[m2 + i][0]];
    }
    for i in (1..m2).rev() {
        g[i] = &g[i << 1] * &mul[i << 1 | 1] + &g[i << 1 | 1] * &mul[i << 1];
    }
    g[1].clone()
}
