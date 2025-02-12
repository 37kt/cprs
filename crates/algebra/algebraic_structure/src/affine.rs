use std::marker::PhantomData;

use algebraic_traits::{Algebraic, Associative, Field, Invertive, Magma, Semiring, Unital};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AffineOperator<T: Semiring> {
    _marker: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
