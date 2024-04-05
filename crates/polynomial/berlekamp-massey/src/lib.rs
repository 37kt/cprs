use modint::ModInt;

pub fn berlekamp_massey<T: ModInt>(a: &[T]) -> Vec<T> {
    let n = a.len();
    let mut b = Vec::with_capacity(n + 1);
    let mut c = Vec::with_capacity(n + 1);
    b.push(1.into());
    c.push(1.into());
    let mut y = 1.into();
    for ed in 1..=n {
        let l = c.len();
        let mut m = b.len();
        let mut x: T = 0.into();
        for i in 0..l {
            x += c[i] * a[ed - l + i];
        }
        b.push(0.into());
        m += 1;
        if x.val() == 0 {
            continue;
        }
        let freq = x / y;
        if l < m {
            let tmp = c.clone();
            c.resize(m, 0.into());
            c.rotate_right(m - l);
            for i in 0..m {
                c[m - 1 - i] -= freq * b[m - 1 - i];
            }
            b = tmp;
            y = x;
        } else {
            for i in 0..m {
                c[l - 1 - i] -= freq * b[m - 1 - i];
            }
        }
    }
    c.reverse();
    c
}
