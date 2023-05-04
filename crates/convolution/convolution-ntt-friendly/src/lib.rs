use convolution_naive::convolution_naive;
use modint::{NttInfo, StaticModInt};

pub fn ntt<const P: u32>(a: &mut [StaticModInt<P>]) {
    assert!(<() as NttInfo<P>>::IS_NTT_FRIENDLY);
    let n = a.len();
    assert_eq!(n.count_ones(), 1);
    let h = n.trailing_zeros() as usize;

    let mut len = 0;
    while len < h {
        if h - len == 1 {
            let p = 1 << h - len - 1;
            let mut rot = StaticModInt::raw(1);
            for s in 0..1 << len {
                let offset = s << h - len;
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p] * rot;
                    a[i + offset] = l + r;
                    a[i + offset + p] = l - r;
                }
                if s + 1 != 1 << len {
                    rot *= StaticModInt::raw(
                        <() as NttInfo<P>>::RATE2[(!s).trailing_zeros() as usize],
                    );
                }
            }
            len += 1;
        } else {
            let p = 1 << h - len - 2;
            let mut rot = StaticModInt::<P>::raw(1);
            let imag = StaticModInt::<P>::raw(<() as NttInfo<P>>::ROOT[2]);
            for s in 0..1 << len {
                let rot2 = rot * rot;
                let rot3 = rot2 * rot;
                let offset = s << h - len;
                for i in 0..p {
                    let mod2 = (StaticModInt::<P>::modulus() as u64).pow(2);
                    let a0 = a[i + offset].val() as u64;
                    let a1 = a[i + offset + p].val() as u64 * rot.val() as u64;
                    let a2 = a[i + offset + p * 2].val() as u64 * rot2.val() as u64;
                    let a3 = a[i + offset + p * 3].val() as u64 * rot3.val() as u64;
                    let a1na3imag =
                        StaticModInt::<P>::from(a1 + mod2 - a3).val() as u64 * imag.val() as u64;
                    let na2 = mod2 - a2;
                    a[i + offset] = StaticModInt::from(a0 + a2 + a1 + a3);
                    a[i + offset + p] = StaticModInt::from(a0 + a2 + (mod2 * 2 - (a1 + a3)));
                    a[i + offset + p * 2] = StaticModInt::from(a0 + na2 + a1na3imag);
                    a[i + offset + p * 3] = StaticModInt::from(a0 + na2 + mod2 - a1na3imag);
                }
                if s + 1 != 1 << len {
                    rot *= StaticModInt::raw(
                        <() as NttInfo<P>>::RATE3[(!s).trailing_zeros() as usize],
                    );
                }
            }
            len += 2;
        }
    }
}

pub fn ntt_inv<const P: u32>(a: &mut [StaticModInt<P>]) {
    assert!(<() as NttInfo<P>>::IS_NTT_FRIENDLY);
    let n = a.len();
    assert_eq!(n.count_ones(), 1);
    let h = n.trailing_zeros() as usize;

    let mut len = h;
    while len > 0 {
        if len == 1 {
            let p = 1 << h - len;
            let mut irot = StaticModInt::<P>::raw(1);
            for s in 0..1 << len - 1 {
                let offset = s << h - len + 1;
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];
                    a[i + offset] = l + r;
                    a[i + offset + p] = StaticModInt::<P>::from(
                        (StaticModInt::<P>::modulus() + l.val() - r.val()) as u64
                            * irot.val() as u64,
                    );
                }
                if s + 1 != 1 << len - 1 {
                    irot *= StaticModInt::<P>::raw(
                        <() as NttInfo<P>>::IRATE2[(!s).trailing_zeros() as usize],
                    );
                }
            }
            len -= 1;
        } else {
            let p = 1 << h - len;
            let mut irot = StaticModInt::<P>::raw(1);
            let iimag = StaticModInt::<P>::raw(<() as NttInfo<P>>::IROOT[2]);
            for s in 0..1 << len - 2 {
                let irot2 = irot * irot;
                let irot3 = irot2 * irot;
                let offset = s << h - len + 2;
                for i in 0..p {
                    let a0 = a[i + offset].val() as u64;
                    let a1 = a[i + offset + p].val() as u64;
                    let a2 = a[i + offset + p * 2].val() as u64;
                    let a3 = a[i + offset + p * 3].val() as u64;
                    let a2na3iimag = StaticModInt::<P>::from(
                        (StaticModInt::<P>::modulus() as u64 + a2 - a3) * iimag.val() as u64,
                    )
                    .val() as u64;
                    a[i + offset] = StaticModInt::<P>::from(a0 + a1 + a2 + a3);
                    a[i + offset + p] = StaticModInt::<P>::from(
                        (a0 + (StaticModInt::<P>::modulus() as u64 - a1) + a2na3iimag)
                            * irot.val() as u64,
                    );
                    a[i + offset + p * 2] = StaticModInt::<P>::from(
                        (a0 + a1
                            + (StaticModInt::<P>::modulus() as u64 - a2)
                            + (StaticModInt::<P>::modulus() as u64 - a3))
                            * irot2.val() as u64,
                    );
                    a[i + offset + p * 3] = StaticModInt::<P>::from(
                        (a0 + (StaticModInt::<P>::modulus() as u64 - a1)
                            + (StaticModInt::<P>::modulus() as u64 - a2na3iimag))
                            * irot3.val() as u64,
                    );
                }
                if s + 1 != 1 << len - 2 {
                    irot *= StaticModInt::<P>::raw(
                        <() as NttInfo<P>>::IRATE3[(!s).trailing_zeros() as usize],
                    );
                }
            }
            len -= 2;
        }
    }

    let inv_n = StaticModInt::<P>::new(n).inv();
    for x in a.iter_mut() {
        *x *= inv_n;
    }
}

pub fn convolution_ntt_friendly<const P: u32>(
    mut a: Vec<StaticModInt<P>>,
    mut b: Vec<StaticModInt<P>>,
) -> Vec<StaticModInt<P>> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    } else if n.min(m) <= 60 {
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
