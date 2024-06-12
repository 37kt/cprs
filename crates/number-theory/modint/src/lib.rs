use std::{
    fmt,
    hash::Hash,
    iter::{Product, Sum},
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
    sync::atomic::{self, AtomicU32, AtomicU64},
};

use algebraic::{One, Zero};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StaticModInt<const P: u32>(u32);

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DynamicModInt(u32);

pub type ModInt998244353 = StaticModInt<998_244_353>;
pub type ModInt1000000007 = StaticModInt<1_000_000_007>;

pub trait ModInt:
    Default
    + Zero
    + One
    + FromStr
    + From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<isize>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + From<usize>
    + Copy
    + Eq
    + Hash
    + fmt::Display
    + fmt::Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    fn modulus() -> u32;
    fn raw(val: u32) -> Self;
    fn val(self) -> u32;
    fn inv(self) -> Self;
    fn pow(self, k: usize) -> Self;
    fn sqrt(self) -> Option<Self>;
}

const fn mul(x: u32, y: u32, m: u32) -> u32 {
    (x as u64 * y as u64 % m as u64) as u32
}

const fn pow(x: u32, mut n: u32, m: u32) -> u32 {
    if m == 1 {
        return 0;
    }
    let mut r = 1u64;
    let mut y = (x % m) as u64;
    while n != 0 {
        if n & 1 != 0 {
            r = r * y % m as u64;
        }
        y = y * y % m as u64;
        n >>= 1;
    }
    r as u32
}

const fn is_prime(n: u32) -> bool {
    match n {
        _ if n <= 1 => return false,
        2 | 7 | 61 => return true,
        _ if n & 1 == 0 => return false,
        _ => {}
    }
    let mut d = n - 1;
    while d & 1 == 0 {
        d >>= 1;
    }
    let a = [2, 7, 61];
    let mut i = 0;
    while i < 3 {
        let mut t = d;
        let mut y = pow(a[i], t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y as u64 * y as u64 % n as u64) as u32;
            t <<= 1;
        }
        if y != n - 1 && t & 1 == 0 {
            return false;
        }
        i += 1;
    }
    true
}

const fn extgcd(mut a: u32, b: u32) -> (u32, u32) {
    a = a % b;
    if a == 0 {
        return (b, 0);
    }

    let mut s = b as i64;
    let mut t = a as i64;
    let mut m0 = 0;
    let mut m1 = 1;
    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        let tmp = s;
        s = t;
        t = tmp;
        let tmp = m0;
        m0 = m1;
        m1 = tmp;
    }
    if m0 < 0 {
        m0 += b as i64 / s;
    }
    (s as u32, m0 as u32)
}

const fn primitive_root(m: u32) -> u32 {
    match m {
        2 => return 1,
        167_772_161 => return 3,
        469_762_049 => return 3,
        754_974_721 => return 11,
        998_244_353 => return 3,
        _ => {}
    }
    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    let mut i = 3;
    while i < std::u32::MAX {
        if i as u64 * i as u64 > x as u64 {
            break;
        }
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
        i += 2;
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }
    let mut g = 2;
    loop {
        let mut i = 0;
        while i < cnt {
            if pow(g, (m - 1) / divs[i], m) == 1 {
                break;
            }
            i += 1;
        }
        if i == cnt {
            break g;
        }
        g += 1;
    }
}

const fn ntt_info(
    m: u32,
) -> (
    u32,
    usize,
    [u32; 30],
    [u32; 30],
    [u32; 30],
    [u32; 30],
    [u32; 30],
    [u32; 30],
) {
    let g = primitive_root(m);
    let rank2 = (m - 1).trailing_zeros() as usize;
    let mut root = [0; 30];
    let mut iroot = [0; 30];
    let mut rate2 = [0; 30];
    let mut irate2 = [0; 30];
    let mut rate3 = [0; 30];
    let mut irate3 = [0; 30];

    root[rank2] = pow(g, (m - 1) >> rank2, m);
    iroot[rank2] = extgcd(root[rank2], m).1;
    let mut i = rank2;
    while i > 0 {
        i -= 1;
        root[i] = mul(root[i + 1], root[i + 1], m);
        iroot[i] = mul(iroot[i + 1], iroot[i + 1], m);
    }

    let mut prod = 1;
    let mut iprod = 1;
    let mut i = 0;
    while i + 2 <= rank2 {
        rate2[i] = mul(root[i + 2], prod, m);
        irate2[i] = mul(iroot[i + 2], iprod, m);
        prod = mul(prod, iroot[i + 2], m);
        iprod = mul(iprod, root[i + 2], m);
        i += 1;
    }

    let mut prod = 1;
    let mut iprod = 1;
    let mut i = 0;
    while i + 3 <= rank2 {
        rate3[i] = mul(root[i + 3], prod, m);
        irate3[i] = mul(iroot[i + 3], iprod, m);
        prod = mul(prod, iroot[i + 3], m);
        iprod = mul(iprod, root[i + 3], m);
        i += 1;
    }

    (g, rank2, root, iroot, rate2, irate2, rate3, irate3)
}

// reference: https://rsk0315.hatenablog.com/entry/2023/04/29/043512
fn rat_convert(x: u64, m: u64, d: u64) -> Option<(u64, u64)> {
    let n = m / (2 * d);
    if x < n && 1 < d {
        return Some((x, 1));
    }

    let mut l = (0, 1);
    let mut r = (1, 0);
    loop {
        let num = l.0 + r.0;
        let den = l.1 + r.1;

        let (i, q) = match (num * m).cmp(&(den * x)) {
            std::cmp::Ordering::Less => {
                // num/den < x/m
                // k = max {k: m (l.0 * k * r.0) < x (l.1 + k * r.1)}
                // k = max {k: k (m r.0 - x r.1) < x l.1 - m l.0}
                let k = (x * l.1 - m * l.0 - 1) / (m * r.0 - x * r.1);
                l.0 += k * r.0;
                l.1 += k * r.1;
                l
            }
            std::cmp::Ordering::Equal => return None,
            std::cmp::Ordering::Greater => {
                // num/den > x/m
                // k = max {k: m (k l.0 + r.0) > x (k l.1 + r.1)}
                // k = max {k: m r.0 - x r.1 > k (x l.1 - m l.0)}
                let k = (m * r.0 - x * r.1 - 1) / (x * l.1 - m * l.0);
                r.0 += k * l.0;
                r.1 += k * l.1;
                r
            }
        };

        if q * x < i * m {
            continue;
        }
        let p = q * x - i * m;
        if p < n && q < d {
            return Some((p, q));
        }
    }
}

impl<const P: u32> ModInt for StaticModInt<P> {
    #[inline(always)]
    fn modulus() -> u32 {
        P
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.0
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }

    fn pow(self, k: usize) -> Self {
        self.pow(k)
    }

    fn sqrt(self) -> Option<Self> {
        self.sqrt()
    }
}

impl<const P: u32> StaticModInt<P> {
    #[inline]
    pub fn new<T: Into<StaticModInt<P>>>(x: T) -> Self {
        x.into()
    }

    #[inline(always)]
    pub fn modulus() -> u32 {
        P
    }

    #[inline]
    pub fn raw(val: u32) -> Self {
        Self(val)
    }

    #[inline]
    pub fn val(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0);
        self.pow(P as usize - 2)
    }

    pub fn pow(mut self, mut k: usize) -> Self {
        let mut res = Self::from(1);
        while k != 0 {
            if k & 1 != 0 {
                res *= self;
            }
            k >>= 1;
            self *= self;
        }
        res
    }

    pub fn sqrt(self) -> Option<Self> {
        let p = Self::modulus() as usize;
        if self.val() < 2 {
            return Some(self);
        } else if self.pow(p - 1 >> 1).val() != 1 {
            return None;
        }
        let mut b = Self::from(1);
        while b.pow((p - 1 >> 1) as usize).val() == 1 {
            b += 1;
        }
        let mut e = (p - 1).trailing_zeros() as usize;
        let m = (p - 1) >> e;
        let mut x = self.pow(m - 1 >> 1);
        let mut y = self * x * x;
        x *= self;
        let mut z = b.pow(m);
        while y.val() != 1 {
            let mut j = 0;
            let mut t = y;
            while t.val() != 1 {
                j += 1;
                t *= t;
            }
            z = z.pow(1 << e - j - 1);
            x *= z;
            z *= z;
            y *= z;
            e = j;
        }
        Some(x)
    }
}

impl ModInt for DynamicModInt {
    #[inline(always)]
    fn modulus() -> u32 {
        BARRETT.modulus()
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.0
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }

    fn pow(self, k: usize) -> Self {
        self.pow(k)
    }

    fn sqrt(self) -> Option<Self> {
        self.sqrt()
    }
}

impl DynamicModInt {
    #[inline]
    pub fn new<T: Into<DynamicModInt>>(x: T) -> Self {
        x.into()
    }

    #[inline(always)]
    pub fn modulus() -> u32 {
        BARRETT.modulus()
    }

    #[inline]
    pub fn raw(val: u32) -> Self {
        Self(val)
    }

    #[inline]
    pub fn val(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn inv(self) -> Self {
        let (g, x) = extgcd(self.0, Self::modulus());
        assert_eq!(g, 1);
        Self(x)
    }

    pub fn pow(mut self, mut k: usize) -> Self {
        let mut res = Self::from(1);
        while k != 0 {
            if k & 1 != 0 {
                res *= self;
            }
            k >>= 1;
            self *= self;
        }
        res
    }

    pub fn sqrt(self) -> Option<Self> {
        let p = Self::modulus() as usize;
        if self.val() < 2 {
            return Some(self);
        } else if self.pow(p - 1 >> 1).val() != 1 {
            return None;
        }
        let mut b = Self::from(1);
        while b.pow((p - 1 >> 1) as usize).val() == 1 {
            b += 1;
        }
        let mut e = (p - 1).trailing_zeros() as usize;
        let m = (p - 1) >> e;
        let mut x = self.pow(m - 1 >> 1);
        let mut y = self * x * x;
        x *= self;
        let mut z = b.pow(m);
        while y.val() != 1 {
            let mut j = 0;
            let mut t = y;
            while t.val() != 1 {
                j += 1;
                t *= t;
            }
            z = z.pow(1 << e - j - 1);
            x *= z;
            z *= z;
            y *= z;
            e = j;
        }
        Some(x)
    }

    pub fn set_modulus(modulus: u32) {
        BARRETT.set(modulus)
    }
}

struct Barrett {
    m: AtomicU32,
    im: AtomicU64,
}

impl Barrett {
    const fn new(m: u32) -> Self {
        Self {
            m: AtomicU32::new(m),
            im: AtomicU64::new((!0 / m as u64).wrapping_add(1)),
        }
    }

    #[inline]
    fn set(&self, m: u32) {
        let im = (!0 / m as u64).wrapping_add(1);
        self.m.store(m, atomic::Ordering::SeqCst);
        self.im.store(im, atomic::Ordering::SeqCst);
    }

    #[inline]
    fn modulus(&self) -> u32 {
        self.m.load(atomic::Ordering::SeqCst)
    }

    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        let m = self.m.load(atomic::Ordering::SeqCst);
        let im = self.im.load(atomic::Ordering::SeqCst);
        let mut z = a as u64;
        z *= b as u64;
        let x = (((z as u128) * (im as u128)) >> 64) as u64;
        let mut v = z.wrapping_sub(x.wrapping_mul(m as u64)) as u32;
        if m <= v {
            v = v.wrapping_add(m);
        }
        v
    }
}

static BARRETT: Barrett = Barrett::new(998_244_353);

impl<const P: u32> FromStr for StaticModInt<P> {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Self::from)
    }
}

impl FromStr for DynamicModInt {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Self::from)
    }
}

impl<const P: u32> fmt::Display for StaticModInt<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for DynamicModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: u32> fmt::Debug for StaticModInt<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((num, den)) = rat_convert(self.0 as u64, P as u64, 1025) {
            write!(f, "{}", num)?;
            if den != 1 {
                write!(f, "/{}", den)?;
            }
        } else if let Some((num, den)) = rat_convert((P - self.0) as u64, P as u64, 1025) {
            write!(f, "-{}", num)?;
            if den != 1 {
                write!(f, "/{}", den)?;
            }
        } else {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}

impl fmt::Debug for DynamicModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_from_integer {
    ($(($t1:ty, $t2:ty)),*) => {
        $(
            impl<const P: u32> From<$t1> for StaticModInt<P> {
                fn from(x: $t1) -> Self {
                    Self((x as $t2).rem_euclid(P as $t2) as u32)
                }
            }
            impl From<$t1> for DynamicModInt {
                fn from(x: $t1) -> Self {
                    Self((x as $t2).rem_euclid(Self::modulus() as $t2) as u32)
                }
            }
        )*
    };
}

impl_from_integer!(
    (i8, i32),
    (i16, i32),
    (i32, i32),
    (i64, i64),
    (isize, i64),
    (i128, i128),
    (u8, u32),
    (u16, u32),
    (u32, u32),
    (u64, u64),
    (usize, u64),
    (u128, u128)
);

impl<const P: u32, T: Into<Self>> AddAssign<T> for StaticModInt<P> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into().0;
        if self.0 >= P {
            self.0 -= P;
        }
    }
}

impl<T: Into<Self>> AddAssign<T> for DynamicModInt {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into().0;
        if self.0 >= Self::modulus() {
            self.0 -= Self::modulus();
        }
    }
}

impl<const P: u32, T: Into<Self>> SubAssign<T> for StaticModInt<P> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into().0;
        if self.0 < rhs {
            self.0 += P;
        }
        self.0 -= rhs;
    }
}

impl<T: Into<Self>> SubAssign<T> for DynamicModInt {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into().0;
        if self.0 < rhs {
            self.0 += Self::modulus();
        }
        self.0 -= rhs;
    }
}

impl<const P: u32, T: Into<Self>> MulAssign<T> for StaticModInt<P> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self((self.0 as u64 * rhs.into().0 as u64 % P as u64) as u32);
    }
}

impl<T: Into<Self>> MulAssign<T> for DynamicModInt {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self(BARRETT.mul(self.0, rhs.into().0));
    }
}

impl<const P: u32, T: Into<Self>> DivAssign<T> for StaticModInt<P> {
    fn div_assign(&mut self, rhs: T) {
        *self *= rhs.into().inv()
    }
}

impl<T: Into<Self>> DivAssign<T> for DynamicModInt {
    fn div_assign(&mut self, rhs: T) {
        *self = *self * rhs.into().inv()
    }
}

impl<const P: u32> Neg for StaticModInt<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self(0)
        } else {
            Self(P - self.0)
        }
    }
}

impl Neg for DynamicModInt {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self(0)
        } else {
            Self(Self::modulus() - self.0)
        }
    }
}

impl<const P: u32> Neg for &StaticModInt<P> {
    type Output = StaticModInt<P>;
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            StaticModInt(0)
        } else {
            StaticModInt(P - self.0)
        }
    }
}

impl Neg for &DynamicModInt {
    type Output = DynamicModInt;
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            DynamicModInt(0)
        } else {
            DynamicModInt(DynamicModInt::modulus() - self.0)
        }
    }
}

macro_rules! impl_ops {
    ($(
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl<const P: u32> $trait_assign<&StaticModInt<P>> for StaticModInt<P> {
            fn $fn_assign(&mut self, rhs: &StaticModInt<P>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const P: u32, T: Into<StaticModInt<P>>> $trait<T> for StaticModInt<P> {
            type Output = StaticModInt<P>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const P: u32> $trait<&StaticModInt<P>> for StaticModInt<P> {
            type Output = StaticModInt<P>;
            fn $fn(self, rhs: &StaticModInt<P>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const P: u32, T: Into<StaticModInt<P>>> $trait<T> for &StaticModInt<P> {
            type Output = StaticModInt<P>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const P: u32> $trait<&StaticModInt<P>> for &StaticModInt<P> {
            type Output = StaticModInt<P>;
            fn $fn(self, rhs: &StaticModInt<P>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
        impl $trait_assign<&DynamicModInt> for DynamicModInt {
            fn $fn_assign(&mut self, rhs: &DynamicModInt) {
                self.$fn_assign(*rhs);
            }
        }
        impl<T: Into<DynamicModInt>> $trait<T> for DynamicModInt {
            type Output = DynamicModInt;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl $trait<&DynamicModInt> for DynamicModInt {
            type Output = DynamicModInt;
            fn $fn(self, rhs: &DynamicModInt) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<T: Into<DynamicModInt>> $trait<T> for &DynamicModInt {
            type Output = DynamicModInt;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl $trait<&DynamicModInt> for &DynamicModInt {
            type Output = DynamicModInt;
            fn $fn(self, rhs: &DynamicModInt) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}

impl_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}

impl<const P: u32> Sum for StaticModInt<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::raw(0), |b, x| b + x)
    }
}

impl<const P: u32> Product for StaticModInt<P> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(1), |b, x| b * x)
    }
}

impl<'a, const P: u32> Sum<&'a Self> for StaticModInt<P> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::raw(0), |b, x| b + x)
    }
}

impl<'a, const P: u32> Product<&'a Self> for StaticModInt<P> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::from(1), |b, x| b * x)
    }
}

impl<const P: u32> StaticModInt<P> {
    pub const G: u32 = ntt_info(P).0;
    pub const RANK2: usize = ntt_info(P).1;
    pub const ROOT: [u32; 30] = ntt_info(P).2;
    pub const IROOT: [u32; 30] = ntt_info(P).3;
    pub const RATE2: [u32; 30] = ntt_info(P).4;
    pub const IRATE2: [u32; 30] = ntt_info(P).5;
    pub const RATE3: [u32; 30] = ntt_info(P).6;
    pub const IRATE3: [u32; 30] = ntt_info(P).7;
    pub const IS_NTT_FRIENDLY: bool = is_prime(P) && Self::RANK2 >= 21;
}

impl<const P: u32> Zero for StaticModInt<P> {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const P: u32> One for StaticModInt<P> {
    fn one() -> Self {
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }
}

impl Zero for DynamicModInt {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl One for DynamicModInt {
    fn one() -> Self {
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }
}
