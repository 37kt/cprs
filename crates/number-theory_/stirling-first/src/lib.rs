use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

/// 第 1 種スターリング数
/// (1, 2, ..., n) の順列をちょうど k 個の巡回置換に分解する方法の数
pub fn stirling_first<const P: u32>(n: usize) -> FormalPowerSeries<P> {
    if n == 0 {
        return fps![1];
    } else if n == 1 {
        return fps![0, 1];
    }

    let log = 63 - n.leading_zeros() as usize;
    let mut f = fps![0, 1];
    for i in (0..log).rev() {
        let m = n >> i;
        f *= f.clone().taylor_shift((m >> 1).into());
        if m & 1 == 1 {
            f = (&f << 1) + f * StaticModInt::new(m - 1);
        }
    }

    f
}

/// 符号付き第 1 種スターリング数
pub fn signed_stirling_first<const P: u32>(n: usize) -> FormalPowerSeries<P> {
    let mut f = stirling_first::<P>(n);
    for i in 0..=n {
        f[i] = if (n - i) % 2 == 0 { f[i] } else { -f[i] };
    }
    f
}
