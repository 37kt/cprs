use ac_library_rs::ModInt998244353 as M;

fn root() -> (Vec<M>, Vec<M>) {
    const P: u32 = 998_244_353;
    let mut r = vec![M::new(0); 24];
    let mut ir = vec![M::new(0); 24];
    for i in 0..24 {
        r[i] = -M::new(3).pow(((P - 1) >> (i + 2)) as u64);
        ir[i] = r[i].inv();
    }
    (r, ir)
}

pub fn ntt(a: &mut [M]) {
    let n = a.len();
    assert!(n.count_ones() == 1);

    let (r, _) = root();

    let mut d = a.len() / 2;
    while d != 0 {
        let mut c = M::new(1);
        for (i, t) in (0..n).step_by(2 * d).zip(1u32..) {
            for (i, j) in (i..i + d).zip(i + d..) {
                let x = a[i];
                let y = a[j] * c;
                a[i] = x + y;
                a[j] = x - y;
            }
            c *= r[t.trailing_zeros() as usize];
        }
        d /= 2;
    }
}

pub fn intt(a: &mut [M]) {
    let n = a.len();
    assert!(n.count_ones() == 1);

    let (_, ir) = root();

    let mut d = 1;
    while d != n {
        let mut c = M::new(1);
        for (i, t) in (0..n).step_by(2 * d).zip(1u32..) {
            for (i, j) in (i..i + d).zip(i + d..) {
                let x = a[i];
                let y = a[j];
                a[i] = x + y;
                a[j] = (x - y) * c;
            }
            c *= ir[t.trailing_zeros() as usize];
        }
        d *= 2;
    }

    let iz = M::new(n).inv();
    for i in 0..a.len() {
        a[i] *= iz;
    }
}

pub fn convolution(a: &[M], b: &[M]) -> Vec<M> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    let n = a.len();
    let m = b.len();

    if n.min(m) <= 60 {
        let mut res = vec![M::new(0); n + m - 1];
        for i in 0..n {
            for j in 0..m {
                res[i + j] += a[i] * b[j];
            }
        }
        return res;
    }

    let mut a = a.to_vec();
    let mut b = b.to_vec();
    let mut z = 1;
    while z < n + m - 1 {
        z <<= 1;
    }
    a.resize(z, M::new(0));
    b.resize(z, M::new(0));
    ntt(&mut a);
    ntt(&mut b);
    for i in 0..a.len() {
        a[i] *= b[i];
    }
    intt(&mut a);
    a.truncate(n + m - 1);

    a
}
