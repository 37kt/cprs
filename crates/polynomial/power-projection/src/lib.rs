// https://nyaannyaan.github.io/library/fps/pow-enumerate.hpp

#![allow(non_snake_case)]

use convolution_ntt_friendly::{ntt, ntt_doubling, ntt_inv};
use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

/// \[x^n\] f(x)^i g(x) を i=0,1,..m について列挙する  
/// n = f.len() - 1
pub fn power_projection<const M: u32>(
    f: &FormalPowerSeries<M>,
    g: &FormalPowerSeries<M>,
    m: usize,
) -> FormalPowerSeries<M> {
    let mut n = f.len() - 1;
    let mut k = 1;
    let mut g = g.clone();
    g.resize(n + 1, 0.into());
    let mut h = 1;
    while h < n + 1 {
        h *= 2;
    }
    let mut P = g;
    let mut Q = -f;
    Q[0] += 1;
    let mut nP = fps![];
    let mut nQ = fps![];

    let inv2 = StaticModInt::<M>::new(2).inv();
    while n > 0 {
        nP.clear();
        nQ.clear();

        let mut buf = fps![];
        for i in 0..=n {
            buf.resize(k, 0.into());
            for j in 0..k {
                buf[j] = P[i * k + j];
            }
            ntt_doubling(&mut buf);
            nP.append(&mut buf);

            let mut buf = fps![0; k];
            for j in 0..k {
                buf[j] = Q[i * k + j];
            }
            if i == 0 {
                for j in 0..k {
                    buf[j] -= 1;
                }
                ntt_doubling(&mut buf);
                for j in 0..k {
                    buf[j] += 1;
                    buf[k + j] -= 1;
                }
            } else {
                ntt_doubling(&mut buf);
            }
            nQ.append(&mut buf);
        }
        nP.resize(2 * h * 2 * k, 0.into());
        nQ.resize(2 * h * 2 * k, 0.into());
        let mut p;
        let mut q;

        let w = StaticModInt::<M>::new(StaticModInt::<M>::G).pow((M - 1) as usize / (2 * h));
        let iw = w.inv();
        let mut btr = vec![];
        if n % 2 != 0 {
            btr.resize(h, 0);
            let lg = h.trailing_zeros() as usize;
            for i in 0..h {
                btr[i] = (btr[i >> 1] >> 1) + ((i & 1) << (lg - 1));
            }
        }

        for j in 0..2 * k {
            p = fps![0; 2 * h];
            q = fps![0; 2 * h];
            for i in 0..h {
                p[i] = nP[i * 2 * k + j];
                q[i] = nQ[i * 2 * k + j];
            }
            ntt(&mut p);
            ntt(&mut q);
            for i in (0..2 * h).step_by(2) {
                q.swap(i, i + 1);
            }
            for i in 0..2 * h {
                p[i] *= q[i];
            }
            for i in 0..h {
                q[i] = q[i * 2] * q[i * 2 + 1];
            }
            if n % 2 == 0 {
                for i in 0..h {
                    p[i] = (p[i * 2] + p[i * 2 + 1]) * inv2;
                }
            } else {
                let mut c = inv2;
                buf.resize(h, 0.into());
                for &i in &btr {
                    buf[i] = (p[i * 2] - p[i * 2 + 1]) * c;
                    c *= iw;
                }
                std::mem::swap(&mut p, &mut buf);
            }
            p.truncate(h);
            q.truncate(h);
            ntt_inv(&mut p);
            ntt_inv(&mut q);
            for i in 0..h {
                nP[i * 2 * k + j] = p[i];
                nQ[i * 2 * k + j] = q[i];
            }
        }
        nP.resize((n / 2 + 1) * 2 * k, 0.into());
        nQ.resize((n / 2 + 1) * 2 * k, 0.into());
        std::mem::swap(&mut P, &mut nP);
        std::mem::swap(&mut Q, &mut nQ);
        n /= 2;
        h /= 2;
        k *= 2;
    }

    let mut S = fps![0; k];
    let mut T = fps![0; k];
    for i in 0..k {
        S[i] = P[i];
        T[i] = Q[i];
    }
    ntt_inv(&mut S);
    ntt_inv(&mut T);
    T[0] -= 1;
    if f[0].val() == 0 {
        S.reverse();
        S.pre(m + 1)
    } else {
        S.reverse();
        T.push(1.into());
        T.reverse();
        (S * T.inv(m + 1)).pre(m + 1)
    }
}
