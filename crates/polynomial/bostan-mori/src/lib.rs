use std::mem::swap;

use convolution_ntt_friendly::{ntt, ntt_doubling, ntt_inv};
use formal_power_series::{fps, FormalPowerSeries};
use modint::StaticModInt;

pub fn bostan_mori<const P: u32>(
    mut p: FormalPowerSeries<P>,
    mut q: FormalPowerSeries<P>,
    mut k: usize,
) -> StaticModInt<P> {
    q.shrink();
    let mut res = 0.into();
    if p.len() >= q.len() {
        let r = &p / &q;
        p -= &r * &q;
        p.shrink();
        if k < r.len() {
            res += r[k];
        }
    }
    if p.len() == 0 {
        return res;
    }

    if StaticModInt::<P>::IS_NTT_FRIENDLY {
        let logn = 64 - (q.len() - 1).leading_zeros() as usize;
        let n = 1 << logn;
        p.resize(n * 2, 0.into());
        q.resize(n * 2, 0.into());
        ntt(&mut p);
        ntt(&mut q);
        let mut s = fps![0; n * 2];
        let mut t = fps![0; n * 2];
        let mut btr = vec![0; n];
        for i in 0..n {
            btr[i] = (btr[i >> 1] >> 1) + ((i & 1) << logn - 1);
        }
        let dw = StaticModInt::new(StaticModInt::<P>::G)
            .inv()
            .pow((P as usize - 1) / (n * 2));
        while k != 0 {
            let mut inv2 = StaticModInt::new(2).inv();
            t.resize(n, 0.into());
            for i in 0..n {
                t[i] = q[i << 1 | 0] * q[i << 1 | 1];
            }
            s.resize(n, 0.into());
            if k & 1 != 0 {
                for &i in &btr {
                    s[i] = (p[i << 1 | 0] * q[i << 1 | 1] - p[i << 1 | 1] * q[i << 1 | 0]) * inv2;
                    inv2 *= dw;
                }
            } else {
                for i in 0..n {
                    s[i] = (p[i << 1 | 0] * q[i << 1 | 1] + p[i << 1 | 1] * q[i << 1 | 0]) * inv2;
                }
            }
            swap(&mut p, &mut s);
            swap(&mut q, &mut t);
            k >>= 1;
            if k < n {
                break;
            }
            ntt_doubling(&mut p);
            ntt_doubling(&mut q);
        }
        ntt_inv(&mut p);
        ntt_inv(&mut q);
        res + (p * q.inv(q.len()))[k]
    } else {
        p.resize(q.len() - 1, 0.into());
        while k != 0 {
            let mut q2 = q.clone();
            for v in q2.iter_mut().skip(1).step_by(2) {
                *v = -*v;
            }
            let s = &p * &q2;
            let t = &q * q2;
            for i in (k & 1..s.len()).step_by(2) {
                p[i >> 1] = s[i];
            }
            for i in (0..t.len()).step_by(2) {
                q[i >> 1] = t[i];
            }
            k >>= 1;
        }
        res + p[0]
    }
}
