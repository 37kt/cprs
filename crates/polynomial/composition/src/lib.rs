// https://nyaannyaan.github.io/library/fps/fps-composition.hpp

#![allow(non_snake_case)]

use convolution_ntt_friendly::{ntt, ntt_inv};
use formal_power_series::{fps, FormalPowerSeries};

/// f(g(x)) mod x^d を求める
pub fn composition<const M: u32>(
    f: &FormalPowerSeries<M>,
    g: &FormalPowerSeries<M>,
    d: usize,
) -> FormalPowerSeries<M> {
    let mut f = f.clone();
    let mut g = g.clone();
    f.resize(d, 0.into());
    g.resize(d, 0.into());
    let n = d - 1;
    let k = 1;
    let mut h = 1;
    while h < n + 1 {
        h *= 2;
    }
    let mut Q = fps![0; h * k];
    for i in 0..=n {
        Q[i] = -g[i];
    }
    let mut P = dfs(Q, &f, n, h, k).pre(d);
    P.reverse();
    P
}

fn dfs<const M: u32>(
    mut Q: FormalPowerSeries<M>,
    f: &FormalPowerSeries<M>,
    n: usize,
    h: usize,
    k: usize,
) -> FormalPowerSeries<M> {
    if n == 0 {
        let mut T = fps![0; k + 1];
        for i in 0..k {
            T[i] = Q[i];
        }
        T[k] += 1;
        T.reverse();
        let mut T = T.inv(k + 1);
        T.reverse();
        let u = f * T;
        let mut P = fps![0; h * k];
        for i in 0..f.len() {
            P[k - 1 - i] = u[i + k];
        }
        return P;
    }
    let mut nQ = fps![0; 4 * h * k];
    let mut nR = fps![0; 2 * h * k];
    for i in 0..k {
        for j in 0..=n {
            nQ[i * 2 * h + j] = Q[i * h + j];
        }
    }
    nQ[k * 2 * h] += 1;
    ntt(&mut nQ);
    for i in (0..4 * h * k).step_by(2) {
        nQ.swap(i, i + 1);
    }
    for i in 0..2 * h * k {
        nR[i] = nQ[i * 2] * nQ[i * 2 + 1];
    }
    ntt_inv(&mut nR);
    nR[0] -= 1;
    Q = fps![0; h * k];
    for i in 0..2 * k {
        for j in 0..=n / 2 {
            Q[i * h / 2 + j] = nR[i * h + j];
        }
    }
    let P = dfs(Q, f, n / 2, h / 2, k * 2);
    let mut nP = fps![0; 4 * h * k];
    for i in 0..2 * k {
        for j in 0..=n / 2 {
            nP[i * 2 * h + j * 2 + n % 2] = P[i * h / 2 + j];
        }
    }
    ntt(&mut nP);
    let mut i = 1;
    while i < 4 * h * k {
        nQ[i..i * 2].reverse();
        i *= 2;
    }
    for i in 0..4 * h * k {
        nP[i] *= nQ[i];
    }
    ntt_inv(&mut nP);
    let mut P = fps![0; h * k];
    for i in 0..k {
        for j in 0..=n {
            P[i * h + j] = nP[i * 2 * h + j];
        }
    }
    P
}
