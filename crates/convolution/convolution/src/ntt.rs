use numeric_traits::Integer;
use static_modint::StaticModInt;

pub fn ntt<const P: u32>(f: &mut [StaticModInt<P>]) {
    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);

    let n = f.len();
    let lg = n.floor_log2();
    assert_eq!(n, 1 << lg, "length must be a power of 2");

    let mut l = 0;
    if (lg - l) % 2 == 1 {
        let p = 1 << (lg - l - 1);
        let mut rot = StaticModInt::from_raw(1);
        for (s, b) in f.chunks_exact_mut(p << 1).enumerate() {
            let (b0, b1) = b.split_at_mut(p);
            for (x, y) in b0.iter_mut().zip(b1) {
                let l = *x;
                let r = *y * rot;
                *x = l + r;
                *y = l - r;
            }
            rot *= StaticModInt::from_raw(
                StaticModInt::<P>::NTT_PRECALC.rate2[s.trailing_ones() as usize],
            );
        }
        l += 1;
    }

    let mod2 = (P as u64) * (P as u64);
    while l < lg {
        let p = 1 << (lg - l - 2);
        let mut rot = StaticModInt::<P>::from_raw(1);
        let imag = StaticModInt::<P>::from_raw(StaticModInt::<P>::NTT_PRECALC.root[2]);
        for (s, b) in f.chunks_exact_mut(p << 2).enumerate() {
            let rot2 = rot * rot;
            let rot3 = rot2 * rot;
            let (b0, b1) = b.split_at_mut(p);
            let (b1, b2) = b1.split_at_mut(p);
            let (b2, b3) = b2.split_at_mut(p);
            for (((x, y), z), w) in b0.iter_mut().zip(b1).zip(b2).zip(b3) {
                let a0 = x.val() as u64;
                let a1 = y.val() as u64 * rot.val() as u64;
                let a2 = z.val() as u64 * rot2.val() as u64;
                let a3 = w.val() as u64 * rot3.val() as u64;
                let a1na3imag =
                    StaticModInt::<P>::from(a1 + mod2 - a3).val() as u64 * imag.val() as u64;
                let na2 = mod2 - a2;
                *x = StaticModInt::from(a0 + a2 + a1 + a3);
                *y = StaticModInt::from(a0 + a2 + (mod2 * 2 - (a1 + a3)));
                *z = StaticModInt::from(a0 + na2 + a1na3imag);
                *w = StaticModInt::from(a0 + na2 + mod2 - a1na3imag);
            }
            rot *= StaticModInt::from_raw(
                StaticModInt::<P>::NTT_PRECALC.rate3[s.trailing_ones() as usize],
            );
        }
        l += 2;
    }
}

pub fn ntt_inv<const P: u32>(f: &mut [StaticModInt<P>]) {
    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);

    let n = f.len();
    let lg = n.floor_log2();
    assert_eq!(n, 1 << lg, "length must be a power of 2");

    let mut l = lg;
    if l % 2 == 1 {
        let p = 1 << (lg - l);
        let mut irot = StaticModInt::<P>::from_raw(1);
        for (s, b) in f.chunks_exact_mut(p << 1).enumerate() {
            let (b0, b1) = b.split_at_mut(p);
            for (x, y) in b0.iter_mut().zip(b1) {
                let l = *x;
                let r = *y;
                *x = l + r;
                *y = StaticModInt::<P>::from((P + l.val() - r.val()) as u64 * irot.val() as u64);
            }
            irot *= StaticModInt::<P>::from_raw(
                StaticModInt::<P>::NTT_PRECALC.irate2[s.trailing_ones() as usize],
            );
        }
        l -= 1;
    }

    while l > 0 {
        let p = 1 << (lg - l);
        let mut irot = StaticModInt::<P>::from_raw(1);
        let iimag = StaticModInt::<P>::from_raw(StaticModInt::<P>::NTT_PRECALC.iroot[2]);
        for (s, b) in f.chunks_exact_mut(p << 2).enumerate() {
            let irot2 = irot * irot;
            let irot3 = irot2 * irot;
            let (b0, b1) = b.split_at_mut(p);
            let (b1, b2) = b1.split_at_mut(p);
            let (b2, b3) = b2.split_at_mut(p);
            for (((x, y), z), w) in b0.iter_mut().zip(b1).zip(b2).zip(b3) {
                let a0 = x.val() as u64;
                let a1 = y.val() as u64;
                let a2 = z.val() as u64;
                let a3 = w.val() as u64;
                let a2na3iimag =
                    StaticModInt::<P>::from((P as u64 + a2 - a3) * iimag.val() as u64).val() as u64;
                *x = StaticModInt::<P>::from(a0 + a1 + a2 + a3);
                *y = StaticModInt::<P>::from(
                    (a0 + (P as u64 - a1) + a2na3iimag) * irot.val() as u64,
                );
                *z = StaticModInt::<P>::from(
                    (a0 + a1 + (P as u64 - a2) + (P as u64 - a3)) * irot2.val() as u64,
                );
                *w = StaticModInt::<P>::from(
                    (a0 + (P as u64 - a1) + (P as u64 - a2na3iimag)) * irot3.val() as u64,
                );
            }
            irot *= StaticModInt::<P>::from_raw(
                StaticModInt::<P>::NTT_PRECALC.irate3[s.trailing_ones() as usize],
            );
        }
        l -= 2;
    }

    let recip_n = StaticModInt::<P>::from_raw(n as u32).recip();
    for x in f.iter_mut() {
        *x *= recip_n;
    }
}

/// 長さ n の DFT から長さ 2n の DFT を求める
pub fn ntt_doubling<const P: u32>(f: &mut Vec<StaticModInt<P>>) {
    let n = f.len();
    f.extend_from_within(..);
    ntt_inv(&mut f[n..]);
    let mut r = StaticModInt::from_raw(1);
    let zeta = StaticModInt::from_raw(StaticModInt::<P>::NTT_PRECALC.primitive_root)
        .pow((P - 1) as usize / (n << 1));
    for x in &mut f[n..] {
        *x *= r;
        r *= zeta;
    }
    ntt(&mut f[n..]);
}
