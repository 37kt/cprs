use combination::Combination;
use formal_power_series::{fps, FormalPowerSeries};

/// 第 2 種スターリング数  
/// n 個の区別できるものを k 個の区別できない箱に分割する方法の数  
/// S(0, k), S(1, k), ..., S(n, k) を返す
pub fn stirling_second_fixed_k<const P: u32>(n: usize, k: usize) -> FormalPowerSeries<P> {
    let mut f = fps![0; n + 1];
    if n < k {
        return f;
    }
    let comb = Combination::new();
    for i in 1..=n {
        f[i] = comb.fact_inv(i);
    }
    f = f.pow(k, n + 1) * comb.fact_inv(k);
    for i in k..=n {
        f[i] *= comb.fact(i);
    }
    f
}
