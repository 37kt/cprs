use algebraic_structure::act::CountsumAffineOperator;

mod sequence_operator {
    use algebraic_traits::{Algebraic, Magma, Monoid, Unital};
    use std::marker::PhantomData;

    #[doc(hidden)]
    pub trait Operator {
        type X;
        type P;
        type F;

        fn single(x: &Self::X) -> Self::P;
        fn op(x: &Self::P, y: &Self::P) -> Self::P;
        fn unit() -> Self::P;
        fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X;
        fn act_to_prod(x: &Self::P, f: &Self::F) -> Self::P;
    }

    trait Foldable {}
    impl<A> Foldable for FoldAct<A> {}
    impl<M> Foldable for Fold<M> {}

    trait Actable {}
    impl<A> Actable for FoldAct<A> {}
    impl<A> Actable for Act<A> {}

    pub struct FoldAct<A>(PhantomData<fn() -> A>);
    pub struct Fold<M>(PhantomData<fn() -> M>);
    pub struct Act<A>(PhantomData<fn() -> A>);
    pub struct Noop<T>(PhantomData<fn() -> T>);

    impl<A> Operator for FoldAct<A>
    where
        A: algebraic_traits::Act,
        A::Operand: Monoid,
        <A::Operand as Algebraic>::Value: Clone,
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

        fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X {
            A::act(x, f)
        }

        fn act_to_prod(x: &Self::P, f: &Self::F) -> Self::P {
            A::act(x, f)
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

        fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {
            x.clone()
        }

        fn act_to_prod(x: &Self::P, _: &Self::F) -> Self::P {
            x.clone()
        }
    }

    impl<A> Operator for Act<A>
    where
        A: algebraic_traits::Act,
    {
        type X = <A::Operand as Algebraic>::Value;
        type P = ();
        type F = <A::Operator as Algebraic>::Value;

        fn single(_: &Self::X) -> Self::P {}

        fn op(_: &Self::P, _: &Self::P) -> Self::P {}

        fn unit() -> Self::P {}

        fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X {
            A::act(x, f)
        }

        fn act_to_prod(_: &Self::P, _: &Self::F) -> Self::P {}
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

        fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {
            x.clone()
        }

        fn act_to_prod(_: &Self::P, _: &Self::F) -> Self::P {}
    }

    pub struct SplayTreeNode<O>
    where
        O: Operator,
    {
        val: O::X,
        prod: O::P,
        prod_rev: O::P,
        act: O::F,
        ch: [Option<Box<SplayTreeNode<O>>>; 2],
        len: usize,
        rev: bool,
    }

    pub struct SplayTree<O>
    where
        O: Operator,
    {
        root: Option<Box<SplayTreeNode<O>>>,
    }

    impl<O> Default for SplayTree<O>
    where
        O: Operator,
    {
        fn default() -> Self {
            Self { root: None }
        }
    }
}

fn main() {
    let size = std::mem::size_of::<
        sequence_operator::SplayTreeNode<sequence_operator::FoldAct<CountsumAffineOperator<i32>>>,
    >();
    eprintln!("{}", size);

    let seq = sequence_operator::SplayTree::<sequence_operator::Noop<i32>>::default();
}
