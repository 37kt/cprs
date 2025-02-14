// https://nyaannyaan.github.io/library/fps/fps-compositional-inverse.hpp

use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;
use power_projection::power_projection;

/// f(g(x)) = x を満たす g(x) mod x^d を求める
pub fn compositional_inverse<const P: u32>(
    f: &FormalPowerSeries<P>,
    d: usize,
) -> FormalPowerSeries<P> {
    assert!(f.len() >= 2 && f[1].val() != 0);
    if d < 2 {
        return fps![0, f[1].inv()].pre(d);
    }
    let n = d - 1;
    let mut h: FormalPowerSeries<P> = power_projection(f, &fps![1], n) * StaticModInt::new(n);
    for k in 1..=n {
        h[k] /= k;
    }
    h.reverse();
    h *= h[0].inv();
    let mut g = (h.log(d) * (-StaticModInt::new(n)).inv()).exp(d);
    g *= f[1].inv();
    (g << 1).pre(d)
}
