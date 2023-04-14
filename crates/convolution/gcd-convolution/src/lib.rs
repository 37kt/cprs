use std::ops::{Add, MulAssign, Sub};

pub fn multiple_zeta<T>(a: &mut Vec<T>)
where
    T: Clone + Add<Output = T>,
{
    let n = a.len() - 1;
    let mut is_prime = vec![true; n + 1];
    for p in 2..=n {
        if is_prime[p] {
            for q in (p * 2..=n).step_by(p) {
                is_prime[q] = false;
            }
            for i in (1..=n / p).rev() {
                a[i] = a[i].clone() + a[i * p].clone();
            }
        }
    }
}

pub fn multiple_moebius<T>(a: &mut Vec<T>)
where
    T: Clone + Sub<Output = T>,
{
    let n = a.len() - 1;
    let mut is_prime = vec![true; n + 1];
    for p in 2..=n {
        if is_prime[p] {
            for q in (p * 2..=n).step_by(p) {
                is_prime[q] = false;
            }
            for i in 1..=n / p {
                a[i] = a[i].clone() - a[i * p].clone();
            }
        }
    }
}

pub fn gcd_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + MulAssign,
{
    assert_eq!(a.len(), b.len());
    multiple_zeta(&mut a);
    multiple_zeta(&mut b);
    for i in 1..a.len() {
        a[i] *= b[i].clone();
    }
    multiple_moebius(&mut a);
    a
}
