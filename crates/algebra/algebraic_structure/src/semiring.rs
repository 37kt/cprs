use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, CommutativeMonoid, Monoid, Semiring};

use crate::{AddOperator, MulOperator};

pub struct SemiringImpl<Additive, Multiplicative>(
    Infallible,
    PhantomData<fn() -> Additive>,
    PhantomData<fn() -> Multiplicative>,
)
where
    Additive: CommutativeMonoid,
    Multiplicative: Monoid<Element = Additive::Element>;

impl<Additive, Multiplicative> Algebraic for SemiringImpl<Additive, Multiplicative>
where
    Additive: CommutativeMonoid,
    Multiplicative: Monoid<Element = Additive::Element>,
{
    type Element = Additive::Element;
}

impl<Additive, Multiplicative> Semiring for SemiringImpl<Additive, Multiplicative>
where
    Additive: CommutativeMonoid,
    Multiplicative: Monoid<Element = Additive::Element>,
{
    type Additive = Additive;
    type Multiplicative = Multiplicative;
}

pub type AddMulOperator<T> = SemiringImpl<AddOperator<T>, MulOperator<T>>;
