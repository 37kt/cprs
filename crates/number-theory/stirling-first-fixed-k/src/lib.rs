use combination::Combination;
use formal_power_series::{fps, FormalPowerSeries};

/// 第 1 種スターリング数  
/// (1, 2, ..., n) の順列をちょうど k 個の巡回置換に分解する方法の数  
/// c(0, k), c(1, k), ..., c(n, k) を返す
pub fn stirling_first_fixed_k<const P: u32>(n: usize, k: usize) -> FormalPowerSeries<P> {
    let mut f = fps![0; n + 1];
    if n < k {
        return f;
    }
    let comb = Combination::new();
    comb.expand(n);
    for i in 1..=n {
        f[i] = comb.inv(i);
    }
    f = f.pow(k, n + 1) * comb.fact_inv(k);
    for i in k..=n {
        f[i] *= comb.fact(i);
    }
    f
}

/// 符号付き第 1 種スターリング数
/// s(0, k), s(1, k), ..., s(n, k) を返す
pub fn signed_stirling_first_fixed_k<const P: u32>(n: usize, k: usize) -> FormalPowerSeries<P> {
    let mut f = stirling_first_fixed_k::<P>(n, k);
    for i in k..=n {
        f[i] = if (i - k) % 2 == 0 { f[i] } else { -f[i] };
    }
    f
}
