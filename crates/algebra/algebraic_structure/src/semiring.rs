use std::marker::PhantomData;

use algebraic_traits::{Algebraic, CommutativeMonoid, Monoid, Semiring};

use crate::{add::AddOperator, mul::MulOperator};

pub struct SemiringImpl<Additive, Multiplicative>(
    PhantomData<fn() -> Additive>,
    PhantomData<fn() -> Multiplicative>,
)
where
    Additive: CommutativeMonoid,
    Multiplicative: Monoid<Value = Additive::Value>;

pub type AddMulOperator<T> = SemiringImpl<AddOperator<T>, MulOperator<T>>;

impl<Additive, Multiplicative> Algebraic for SemiringImpl<Additive, Multiplicative>
where
    Additive: CommutativeMonoid,
    Multiplicative: Monoid<Value = Additive::Value>,
{
    type Value = Additive::Value;
}

impl<Additive, Multiplicative> Semiring for SemiringImpl<Additive, Multiplicative>
where
    Additive: CommutativeMonoid,
    Multiplicative: Monoid<Value = Additive::Value>,
{
    type Additive = Additive;
    type Multiplicative = Multiplicative;
}
