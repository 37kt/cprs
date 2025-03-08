pub(crate) const fn mul_mod(x: u32, y: u32, m: u32) -> u32 {
    ((x as u64) * (y as u64) % (m as u64)) as u32
}

pub(crate) const fn pow_mod(x: u32, mut e: usize, m: u32) -> u32 {
    let mut res = 1;
    let mut x = (x % m) as u64;
    let m = m as u64;
    while e > 0 {
        if e & 1 != 0 {
            res = res * x % m;
        }
        x = x * x % m;
        e >>= 1;
    }
    res as u32
}

pub(crate) const fn is_prime(x: u32) -> bool {
    match x {
        ..=1 => return false,
        2 | 7 | 61 => return true,
        _ if x & 1 == 0 => return false,
        _ => {}
    }

    let mut d = x - 1;
    while d & 1 == 0 {
        d >>= 1;
    }

    let a = [2, 7, 61];
    let mut i = 0;
    while i < a.len() {
        let mut t = d;
        let mut y = pow_mod(a[i], t as _, x);
        while t != x - 1 && y != 1 && y != x - 1 {
            y = mul_mod(y, y, x);
            t <<= 1;
        }
        if y != x - 1 && t & 1 == 0 {
            return false;
        }
        i += 1;
    }

    true
}

pub(crate) const fn primitive_root(m: u32) -> u32 {
    match m {
        2 => return 1,
        167_772_161 => return 3,
        469_762_049 => return 3,
        754_974_721 => return 11,
        998_244_353 => return 3,
        _ => {}
    }

    let mut pf = [0; 20];
    pf[0] = 2;
    let mut n = 1;
    let mut x = m - 1;
    while x & 1 == 0 {
        x >>= 1;
    }

    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            pf[n] = i;
            n += 1;
            while x % i == 0 {
                x /= i;
            }
        }
        i += 2;
    }
    if x > 1 {
        pf[n] = x;
        n += 1;
    }

    let mut g = 2;
    loop {
        let mut i = 0;
        while i < n {
            if pow_mod(g, ((m - 1) / pf[i]) as _, m) == 1 {
                break;
            }
            i += 1;
        }
        if i == n {
            break g;
        }
        g += 1;
    }
}

pub(crate) const fn inv_mod(x: u32, m: u32) -> u32 {
    let (mut a, mut b, mut x, mut y) = (1, 0, x, m);
    if m == 1 {
        return 0;
    }

    loop {
        match x {
            0 => panic!("gcd(x, m) is not 1."),
            1 => return a,
            _ => {}
        }
        b += a * (y / x);
        y %= x;

        match y {
            0 => panic!("gcd(x, m) is not 1."),
            1 => return m - b,
            _ => {}
        }
        a += b * (x / y);
        x %= y;
    }
}
