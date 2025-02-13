use std::{
    convert::Infallible,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use algebraic_traits::{Algebraic, Associative, Field, Invertive, Magma, Semiring, Unital};

pub struct AffineOperator<T: Semiring>(Infallible, PhantomData<fn() -> T>);

pub struct Affine<T: Semiring>(pub T::Element, pub T::Element);

impl<T: Semiring> Algebraic for AffineOperator<T> {
    type Element = Affine<T>;
}

impl<T: Semiring> Magma for AffineOperator<T> {
    fn op(Affine(a, b): &Affine<T>, Affine(c, d): &Affine<T>) -> Affine<T> {
        Affine(T::mul(a, c), T::add(&T::mul(b, c), d))
    }
}

impl<T: Semiring> Unital for AffineOperator<T> {
    fn unit() -> Affine<T> {
        Affine(T::one(), T::zero())
    }
}

impl<T: Field> Invertive for AffineOperator<T>
where
    T::Additive: Invertive,
    T::Multiplicative: Invertive,
    T::Element: Clone,
{
    fn inv(Affine(a, b): &Affine<T>) -> Affine<T> {
        let recip_a = T::recip(a);
        Affine(recip_a.clone(), T::neg(&T::mul(b, &recip_a)))
    }
}

impl<T: Semiring> Associative for AffineOperator<T> {}

impl<T: Semiring> Debug for Affine<T>
where
    T::Element: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.0, self.1)
    }
}

impl<T: Semiring> Clone for Affine<T>
where
    T::Element: Clone,
{
    fn clone(&self) -> Self {
        Affine(self.0.clone(), self.1.clone())
    }
}

impl<T: Semiring> Copy for Affine<T> where T::Element: Copy {}

impl<T: Semiring> PartialEq for Affine<T>
where
    T::Element: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T: Semiring> Eq for Affine<T> where T::Element: Eq {}

impl<T: Semiring> Hash for Affine<T>
where
    T::Element: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}
