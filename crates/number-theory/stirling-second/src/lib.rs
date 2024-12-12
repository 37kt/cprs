use combination::Combination;
use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

/// 第 2 種スターリング数  
/// n 個の区別できるものを k 個の区別できない箱に分割する方法の数
/// S(0, k), S(1, k), ..., S(n, k) を返す
pub fn stirling_second<const P: u32>(n: usize) -> FormalPowerSeries<P> {
    let comb = Combination::<StaticModInt<P>>::new();
    let mut f = fps![0; n + 1];
    let mut g = fps![0; n + 1];
    for i in 0..=n {
        f[i] = StaticModInt::from(i).pow(n) * comb.fact_inv(i);
        g[i] = if i & 1 == 1 {
            -comb.fact_inv(i)
        } else {
            comb.fact_inv(i)
        };
    }
    (f * g).pre(n + 1)
}
