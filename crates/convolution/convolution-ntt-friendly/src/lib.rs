use convolution_naive::convolution_naive;
use modint::StaticModInt;

pub fn ntt<const P: u32>(a: &mut [StaticModInt<P>]) {
    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);
    let n = a.len();
    assert_eq!(n.count_ones(), 1);
    let h = n.trailing_zeros() as usize;

    let mut len = 0;
    if (h - len) % 2 == 1 {
        let p = 1 << h - len - 1;
        let mut rot = StaticModInt::raw(1);
        for (s, b) in a.chunks_exact_mut(p * 2).enumerate() {
            let (b0, b1) = b.split_at_mut(p);
            for (x, y) in b0.iter_mut().zip(b1.iter_mut()) {
                let l = *x;
                let r = *y * rot;
                *x = l + r;
                *y = l - r;
            }
            rot *= StaticModInt::raw(StaticModInt::<P>::RATE2[(!s).trailing_zeros() as usize]);
        }
        len += 1;
    }
    let mod2 = (P as u64) * (P as u64);
    while len < h {
        let p = 1 << h - len - 2;
        let mut rot = StaticModInt::<P>::raw(1);
        let imag = StaticModInt::<P>::raw(StaticModInt::<P>::ROOT[2]);
        for (s, b) in a.chunks_exact_mut(p * 4).enumerate() {
            let rot2 = rot * rot;
            let rot3 = rot2 * rot;
            let (b0, b1) = b.split_at_mut(p);
            let (b1, b2) = b1.split_at_mut(p);
            let (b2, b3) = b2.split_at_mut(p);
            for (((x, y), z), w) in b0
                .iter_mut()
                .zip(b1.iter_mut())
                .zip(b2.iter_mut())
                .zip(b3.iter_mut())
            {
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
            rot *= StaticModInt::raw(StaticModInt::<P>::RATE3[(!s).trailing_zeros() as usize]);
        }
        len += 2;
    }
}

pub fn ntt_inv<const P: u32>(a: &mut [StaticModInt<P>]) {
    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);
    let n = a.len();
    assert_eq!(n.count_ones(), 1);
    let h = n.trailing_zeros() as usize;

    let mut len = h;
    if len % 2 == 1 {
        let p = 1 << h - len;
        let mut irot = StaticModInt::<P>::raw(1);
        for (s, b) in a.chunks_exact_mut(p * 2).enumerate() {
            let (b0, b1) = b.split_at_mut(p);
            for (x, y) in b0.iter_mut().zip(b1.iter_mut()) {
                let l = *x;
                let r = *y;
                *x = l + r;
                *y = StaticModInt::<P>::from((P + l.val() - r.val()) as u64 * irot.val() as u64);
            }
            irot *=
                StaticModInt::<P>::raw(StaticModInt::<P>::IRATE2[(!s).trailing_zeros() as usize]);
        }
        len -= 1;
    }
    while len > 0 {
        let p = 1 << h - len;
        let mut irot = StaticModInt::<P>::raw(1);
        let iimag = StaticModInt::<P>::raw(StaticModInt::<P>::IROOT[2]);
        for (s, b) in a.chunks_exact_mut(p * 4).enumerate() {
            let irot2 = irot * irot;
            let irot3 = irot2 * irot;
            let (b0, b1) = b.split_at_mut(p);
            let (b1, b2) = b1.split_at_mut(p);
            let (b2, b3) = b2.split_at_mut(p);
            for (((x, y), z), w) in b0
                .iter_mut()
                .zip(b1.iter_mut())
                .zip(b2.iter_mut())
                .zip(b3.iter_mut())
            {
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
            irot *=
                StaticModInt::<P>::raw(StaticModInt::<P>::IRATE3[(!s).trailing_zeros() as usize]);
        }
        len -= 2;
    }

    let inv_n = StaticModInt::<P>::new(n).inv();
    for x in a.iter_mut() {
        *x *= inv_n;
    }
}

pub fn ntt_doubling<const P: u32>(a: &mut Vec<StaticModInt<P>>) {
    let n = a.len();
    a.append(&mut a.clone());
    ntt_inv(&mut a[n..]);
    let mut r = StaticModInt::new(1);
    let zeta = StaticModInt::new(StaticModInt::<P>::G).pow((P - 1) as usize / (n << 1));
    for i in n..n * 2 {
        a[i] *= r;
        r *= zeta;
    }
    ntt(&mut a[n..]);
}

pub fn convolution_ntt_friendly<const P: u32>(
    mut a: Vec<StaticModInt<P>>,
    mut b: Vec<StaticModInt<P>>,
) -> Vec<StaticModInt<P>> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    if n.min(m) <= 60 {
        return convolution_naive(&a, &b);
    }
    let len = n + m - 1;
    let z = 1 << 64 - (len - 1).leading_zeros();
    a.resize(z, 0.into());
    b.resize(z, 0.into());
    ntt(&mut a);
    ntt(&mut b);
    for i in 0..z {
        a[i] *= b[i];
    }
    ntt_inv(&mut a);
    a.truncate(len);
    a
}
