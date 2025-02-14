use std::mem::swap;

fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

/// Σ_{i=0}^{n-1} floor((a*i+b)/m)
pub fn floor_sum_unsigned(mut n: u64, mut m: u64, mut a: u64, mut b: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut res = 0;
    loop {
        if a >= m {
            res += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            res += n * (b / m);
            b %= m;
        }

        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        swap(&mut m, &mut a);
    }
    res
}

/// Σ_{i=0}^{n-1} floor((a*i+b)/m)
pub fn floor_sum(n: u64, m: u64, mut a: i64, mut b: i64) -> i64 {
    let n = n as i64;
    let m = m as i64;
    if n == 0 {
        return 0;
    }
    let mut res = 0;
    if a < 0 {
        let a2 = safe_mod(a, m);
        res -= n * (n - 1) / 2 * ((a2 - a) / m);
        a = a2;
    }
    if b < 0 {
        let b2 = safe_mod(b, m);
        res -= n * ((b2 - b) / m);
        b = b2;
    }
    res + floor_sum_unsigned(n as u64, m as u64, a as u64, b as u64) as i64
}
