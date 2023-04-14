use std::ops::{Add, MulAssign, Sub};

pub fn superset_zeta<T>(a: &mut Vec<T>)
where
    T: Clone + Add<Output = T>,
{
    let log = a.len().trailing_zeros();
    assert_eq!(1 << log, a.len());
    for i in 0..log {
        let i = 1 << i;
        for j in (0..a.len()).step_by(i << 1) {
            for k in 0..i {
                a[j + k] = a[j + k].clone() + a[i + j + k].clone();
            }
        }
    }
}

pub fn superset_moebius<T>(a: &mut Vec<T>)
where
    T: Clone + Sub<Output = T>,
{
    let log = a.len().trailing_zeros();
    assert_eq!(1 << log, a.len());
    for i in 0..log {
        let i = 1 << i;
        for j in (0..a.len()).step_by(i << 1) {
            for k in 0..i {
                a[j + k] = a[j + k].clone() - a[i + j + k].clone();
            }
        }
    }
}

pub fn and_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + MulAssign,
{
    assert_eq!(a.len(), b.len());
    superset_zeta(&mut a);
    superset_zeta(&mut b);
    for i in 0..a.len() {
        a[i] *= b[i].clone();
    }
    superset_moebius(&mut a);
    a
}
