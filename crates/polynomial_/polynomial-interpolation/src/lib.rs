use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

/// 多項式補間  
///
/// P(x\[i\]) = y\[i\] となる多項式 P(x) を求める。
pub fn polynomial_interpolation<const P: u32>(
    x: &[StaticModInt<P>],
    y: &[StaticModInt<P>],
) -> FormalPowerSeries<P> {
    assert_eq!(x.len(), y.len());
    let m = x.len();
    if m == 0 {
        return fps![];
    }
    let m2 = 1 << 64 - (m - 1).leading_zeros();
    let mut mul = vec![fps![1]; m2 + m2];
    for i in 0..m {
        mul[m2 + i] = fps![-x[i], 1];
    }
    for i in (1..m2).rev() {
        mul[i] = &mul[i << 1 | 0] * &mul[i << 1 | 1];
    }
    let mut g = vec![fps![]; m2 + m2];
    g[1] = mul[1].differential().div_mod(&mul[1]).1;
    for i in 2..m2 + m {
        g[i] = g[i >> 1].div_mod(&mul[i]).1;
    }
    for i in 0..m {
        g[m2 + i] = fps![y[i] / g[m2 + i][0]];
    }
    for i in (1..m2).rev() {
        g[i] = &g[i << 1] * &mul[i << 1 | 1] + &g[i << 1 | 1] * &mul[i << 1];
    }
    g[1].clone()
}
