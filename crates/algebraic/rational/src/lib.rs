use std::{
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
    },
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

use algebraic::{One, Zero};

static AUTO_REDUCE: AtomicBool = AtomicBool::new(true);

pub trait ZTrait:
    Copy
    + PartialEq
    + PartialOrd
    + Eq
    + Ord
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Rem<Output = Self>
    + BitXor<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Debug
    + Display
    + std::iter::Sum
    + From<i32>
{
    fn abs(&self) -> Self {
        if *self < Self::zero() {
            -*self
        } else {
            *self
        }
    }

    fn to_f64(&self) -> f64;
}

impl ZTrait for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ZTrait for i64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ZTrait for i128 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

#[derive(Clone, Copy)]
pub struct Rational<T>
where
    T: ZTrait,
{
    pub num: T,
    pub den: T,
}

fn gcd<T: ZTrait>(mut a: T, mut b: T) -> T {
    a = a.abs();
    b = b.abs();
    while b != T::zero() {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

impl<T> Default for Rational<T>
where
    T: ZTrait,
{
    fn default() -> Self {
        Self {
            num: T::zero(),
            den: T::one(),
        }
    }
}

impl<T> Display for Rational<T>
where
    T: ZTrait,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl<T> Debug for Rational<T>
where
    T: ZTrait,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl<T> From<T> for Rational<T>
where
    T: ZTrait,
{
    fn from(x: T) -> Self {
        Self {
            num: x,
            den: T::one(),
        }
    }
}

impl<T> PartialEq for Rational<T>
where
    T: ZTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == other.num * self.den
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl<T> Eq for Rational<T> where T: ZTrait {}

impl<T> PartialOrd for Rational<T>
where
    T: ZTrait,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Rational<T>
where
    T: ZTrait,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}

impl<T> Rational<T>
where
    T: ZTrait,
{
    pub fn set_auto_reduce(auto_reduce: bool) {
        AUTO_REDUCE.store(auto_reduce, SeqCst);
    }

    pub fn new(num: T, den: T) -> Self {
        let mut res = Self { num, den };
        res.normalize();
        res
    }

    pub fn raw(num: T, den: T) -> Self {
        Self { num, den }
    }

    pub fn num(&self) -> T {
        self.num
    }

    pub fn den(&self) -> T {
        self.den
    }

    pub fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den,
        }
    }

    pub fn normalize(&mut self) {
        assert!(self.num != T::zero() || self.den != T::zero());
        if self.den == T::zero() {
            self.num = if self.num > T::zero() {
                T::one()
            } else {
                -T::one()
            };
            self.den = T::zero();
            return;
        }
        if self.den < T::zero() {
            self.num = -self.num;
            self.den = -self.den;
        }
        if self.num == T::zero() {
            self.den = T::one();
        }
        if AUTO_REDUCE.load(SeqCst) {
            self.reduce();
        }
    }

    pub fn reduce(&mut self) {
        let g = gcd(self.num, self.den);
        self.num /= g;
        self.den /= g;
    }

    pub fn to_f64(&self) -> f64 {
        self.num.to_f64() / self.den.to_f64()
    }

    pub fn floor(&self) -> T {
        div::div_floor(self.num, self.den)
    }

    pub fn ceil(&self) -> T {
        div::div_ceil(self.num, self.den)
    }
}

impl<T> Neg for Rational<T>
where
    T: ZTrait,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            num: -self.num,
            den: self.den,
        }
    }
}

impl<T> Neg for &Rational<T>
where
    T: ZTrait,
{
    type Output = Rational<T>;
    fn neg(self) -> Self::Output {
        Rational {
            num: -self.num,
            den: self.den,
        }
    }
}

impl<T> AddAssign<Self> for Rational<T>
where
    T: ZTrait,
{
    fn add_assign(&mut self, rhs: Self) {
        self.num = self.num * rhs.den + rhs.num * self.den;
        self.den *= rhs.den;
        self.normalize();
    }
}

impl<T> SubAssign<Self> for Rational<T>
where
    T: ZTrait,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl<T> MulAssign<Self> for Rational<T>
where
    T: ZTrait,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.num *= rhs.num;
        self.den *= rhs.den;
        self.normalize();
    }
}

impl<T> DivAssign<Self> for Rational<T>
where
    T: ZTrait,
{
    fn div_assign(&mut self, rhs: Self) {
        self.num *= rhs.den;
        self.den *= rhs.num;
        self.normalize();
    }
}

macro_rules! impl_ops {
    ($(
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl<T> $trait_assign<&Rational<T>> for Rational<T>
        where
            T: ZTrait,
        {
            fn $fn_assign(&mut self, rhs: &Rational<T>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<T, U: Into<Rational<T>>> $trait<U> for Rational<T>
        where
            T: ZTrait,
        {
            type Output = Rational<T>;
            fn $fn(mut self, rhs: U) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<T> $trait<&Rational<T>> for Rational<T>
        where
            T: ZTrait,
        {
            type Output = Rational<T>;
            fn $fn(self, rhs: &Rational<T>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<T, U: Into<Rational<T>>> $trait<U> for &Rational<T>
        where
            T: ZTrait,
        {
            type Output = Rational<T>;
            fn $fn(self, rhs: U) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<T> $trait<&Rational<T>> for &Rational<T>
        where
            T: ZTrait,
        {
            type Output = Rational<T>;
            fn $fn(self, rhs: &Rational<T>) -> Self::Output {
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

impl<T> Zero for Rational<T>
where
    T: ZTrait,
{
    fn zero() -> Self {
        Self::from(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.num == T::zero()
    }
}

impl<T> One for Rational<T>
where
    T: ZTrait,
{
    fn one() -> Self {
        Self::from(T::one())
    }

    fn is_one(&self) -> bool {
        self.num == T::one() && self.den == T::one()
    }
}
