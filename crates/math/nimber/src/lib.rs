use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    mem::swap,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Nimber(usize);

impl FromStr for Nimber {
    type Err = <usize as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Nimber(s.parse::<usize>()?))
    }
}

impl<T, E> From<T> for Nimber
where
    T: TryInto<usize, Error = E>,
    E: Debug,
{
    fn from(x: T) -> Self {
        Nimber(x.try_into().unwrap())
    }
}

impl Nimber {
    pub fn new<T, E>(x: T) -> Self
    where
        T: TryInto<usize, Error = E>,
        E: Debug,
    {
        Self::from(x)
    }

    pub fn val(&self) -> usize {
        self.0
    }
}

impl Display for Nimber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Nimber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Neg for Nimber {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self
    }
}

impl Neg for &Nimber {
    type Output = Nimber;
    fn neg(self) -> Self::Output {
        *self
    }
}

impl AddAssign for Nimber {
    fn add_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl SubAssign for Nimber {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl MulAssign for Nimber {
    fn mul_assign(&mut self, rhs: Self) {
        PRECALC.with(|precalc| {
            SMALL.with(|small| {
                let mut res = 0;
                for d in 0..8 {
                    for e in 0..8 {
                        res ^= precalc[d][e][small[self.0 >> d * 8 & 255][rhs.0 >> e * 8 & 255]];
                    }
                }
                self.0 = res;
            })
        })
    }
}

macro_rules! impl_ops {
    ($(
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl $trait_assign<&Nimber> for Nimber {
            fn $fn_assign(&mut self, rhs: &Nimber) {
                self.$fn_assign(*rhs);
            }
        }
        impl $trait<&Nimber> for Nimber {
            type Output = Nimber;
            fn $fn(mut self, rhs: &Nimber) -> Self::Output {
                self.$fn_assign(*rhs);
                self
            }
        }
        impl $trait<Nimber> for &Nimber {
            type Output = Nimber;
            fn $fn(self, rhs: Nimber) -> Self::Output {
                (*self).$fn(rhs)
            }
        }
        impl $trait<Nimber> for Nimber {
            type Output = Nimber;
            fn $fn(mut self, rhs: Nimber) -> Self::Output {
                self.$fn_assign(rhs);
                self
            }
        }
        impl $trait<&Nimber> for &Nimber {
            type Output = Nimber;
            fn $fn(self, rhs: &Nimber) -> Self::Output {
                (*self).$fn(&*rhs)
            }
        }
    )*};
}

impl_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
}

fn rec(mut x: usize, mut y: usize) -> usize {
    if x == 0 || y == 0 {
        return 0;
    }
    if x < y {
        swap(&mut x, &mut y);
    }
    if y == 1 {
        return x;
    }
    for k in (0..6).rev() {
        let shift = 1 << k;
        let mask = (1 << shift) - 1;
        if y >> shift != 0 {
            let v00 = rec(x & mask, y & mask);
            let v01 = rec(x & mask, y >> shift);
            let v10 = rec(x >> shift, y & mask);
            let v11 = rec(x >> shift, y >> shift);
            return v00 ^ (v01 ^ v10 ^ v11) << shift ^ rec(v11, 1 << shift - 1);
        } else if x >> shift != 0 {
            return rec(x >> shift, y) << shift ^ rec(x & mask, y);
        }
    }
    unreachable!()
}

fn small() -> Box<[Box<[usize]>]> {
    let mut t = vec![vec![0; 256].into_boxed_slice(); 256].into_boxed_slice();
    for i in 0..256 {
        for j in 0..256 {
            t[i][j] = rec(i, j);
        }
    }
    t
}

fn precalc() -> Box<[Box<[Box<[usize]>]>]> {
    let mut t =
        vec![vec![vec![0; 256].into_boxed_slice(); 8].into_boxed_slice(); 8].into_boxed_slice();
    for d in 0..8 {
        for e in 0..8 {
            let p = rec(1 << d * 8, 1 << e * 8);
            for i in 0..256 {
                t[d][e][i] = rec(p, i);
            }
        }
    }
    t
}

thread_local! {
    static SMALL: Box<[Box<[usize]>]> = small();
    static PRECALC: Box<[Box<[Box<[usize]>]>]> = precalc();
}
