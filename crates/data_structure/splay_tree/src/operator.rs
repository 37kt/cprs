use algebraic_traits::{Algebraic, Magma, Monoid, Unital};
use std::marker::PhantomData;

pub struct FoldAct<A>(PhantomData<fn() -> A>);
pub struct Fold<M>(PhantomData<fn() -> M>);
pub struct Act<A>(PhantomData<fn() -> A>);
pub struct Noop<T>(PhantomData<fn() -> T>);

#[doc(hidden)]
pub trait Operator {
    type X: Clone;
    type P: Clone;
    type F: Clone + PartialEq;

    fn single(x: &Self::X) -> Self::P;
    fn op(x: &Self::P, y: &Self::P) -> Self::P;
    fn unit() -> Self::P;
    fn unit_act() -> Self::F;
    fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X;
    fn act_to_prod(x: &Self::P, f: &Self::F) -> Self::P;
    fn compose(f: &Self::F, g: &Self::F) -> Self::F;
}

#[doc(hidden)]
pub trait Foldable {}
impl<A> Foldable for FoldAct<A> {}
impl<M> Foldable for Fold<M> {}

#[doc(hidden)]
pub trait Actable {}
impl<A> Actable for FoldAct<A> {}
impl<A> Actable for Act<A> {}

impl<A> Operator for FoldAct<A>
where
    A: algebraic_traits::Act,
    A::Operand: Monoid,
    <A::Operand as Algebraic>::Value: Clone,
    A::Operator: Monoid,
    <A::Operator as Algebraic>::Value: Clone + PartialEq,
{
    type X = <A::Operand as Algebraic>::Value;
    type P = <A::Operand as Algebraic>::Value;
    type F = <A::Operator as Algebraic>::Value;

    fn single(x: &Self::X) -> Self::P {
        x.clone()
    }

    fn op(x: &Self::P, y: &Self::P) -> Self::P {
        A::Operand::op(x, y)
    }

    fn unit() -> Self::P {
        A::Operand::unit()
    }

    fn unit_act() -> Self::F {
        A::Operator::unit()
    }

    fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X {
        A::act(x, f)
    }

    fn act_to_prod(x: &Self::P, f: &Self::F) -> Self::P {
        A::act(x, f)
    }

    fn compose(f: &Self::F, g: &Self::F) -> Self::F {
        A::Operator::op(f, g)
    }
}

impl<M> Operator for Fold<M>
where
    M: Monoid,
    <M as Algebraic>::Value: Clone,
{
    type X = <M as Algebraic>::Value;
    type P = <M as Algebraic>::Value;
    type F = ();

    fn single(x: &Self::X) -> Self::P {
        x.clone()
    }

    fn op(x: &Self::P, y: &Self::P) -> Self::P {
        M::op(x, y)
    }

    fn unit() -> Self::P {
        M::unit()
    }

    fn unit_act() -> Self::F {}

    fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {
        x.clone()
    }

    fn act_to_prod(x: &Self::P, _: &Self::F) -> Self::P {
        x.clone()
    }

    fn compose(_: &Self::F, _: &Self::F) -> Self::F {}
}

impl<A> Operator for Act<A>
where
    A: algebraic_traits::Act,
    A::Operator: Monoid,
    <A::Operator as Algebraic>::Value: Clone + PartialEq,
    <A::Operand as Algebraic>::Value: Clone,
{
    type X = <A::Operand as Algebraic>::Value;
    type P = ();
    type F = <A::Operator as Algebraic>::Value;

    fn single(_: &Self::X) -> Self::P {}

    fn op(_: &Self::P, _: &Self::P) -> Self::P {}

    fn unit() -> Self::P {}

    fn unit_act() -> Self::F {
        A::Operator::unit()
    }

    fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X {
        A::act(x, f)
    }

    fn act_to_prod(_: &Self::P, _: &Self::F) -> Self::P {}

    fn compose(f: &Self::F, g: &Self::F) -> Self::F {
        A::Operator::op(f, g)
    }
}

impl<T> Operator for Noop<T>
where
    T: Clone,
{
    type X = T;
    type P = ();
    type F = ();

    fn single(_: &Self::X) -> Self::P {}

    fn op(_: &Self::P, _: &Self::P) -> Self::P {}

    fn unit() -> Self::P {}

    fn unit_act() -> Self::F {}

    fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {
        x.clone()
    }

    fn act_to_prod(_: &Self::P, _: &Self::F) -> Self::P {}

    fn compose(_: &Self::F, _: &Self::F) -> Self::F {}
}
