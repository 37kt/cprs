use combination::Combination;
use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

/// ベル数  
/// b(n) := 大きさ n の集合を分割する方法の数
/// b(0), b(1), ..., b(n) を返す
pub fn bell_number<const P: u32>(n: usize) -> FormalPowerSeries<P> {
    // ベル数の EGF は exp(exp(x) - 1)

    let comb = Combination::<StaticModInt<P>>::new();
    comb.expand(n);

    let mut f = fps![0; n + 1];
    for i in 1..=n {
        f[i] = comb.fact_inv(i);
    }

    let mut f = f.exp(n + 1);
    for i in 0..=n {
        f[i] *= comb.fact(i);
    }

    f
}
