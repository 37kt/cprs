#[macro_export]
macro_rules! define_algebra {
    (name: $name:ident, value: $value:ty) => {
        enum $name {}
        impl $crate::Algebraic for $name {
            type Value = $value;
        }
    };

    ($vis:vis, name: $name:ident, value: $value:ty) => {
        $vis enum $name {}
        impl $crate::Algebraic for $name {
            type Value = $value;
        }
    };

    (name: $name:ident, value: $value:ty, $($rest:tt)*) => {
        define_algebra!(name: $name, value: $value);
        define_algebra!(@impl $name, $($rest)*);
    };

    ($vis:vis, name: $name:ident, value: $value:ty, $($rest:tt)*) => {
        define_algebra!($vis, name: $name, value: $value);
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, op: $op:expr, $($rest:tt)*) => {
        impl $crate::Magma for $name {
            fn op(x: &Self::Value, y: &Self::Value) -> Self::Value {
                $op(x, y)
            }
        }
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, unit: $unit:expr, $($rest:tt)*) => {
        impl $crate::Unital for $name {
            fn unit() -> Self::Value {
                $unit
            }
        }
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, inv: $inv:expr, $($rest:tt)*) => {
        impl $crate::Invertive for $name {
            fn inv(x: &Self::Value) -> Self::Value {
                $inv(x)
            }
        }
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, associative, $($rest:tt)*) => {
        impl $crate::Associative for $name {}
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, commutative, $($rest:tt)*) => {
        impl $crate::Commutative for $name {}
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, idempotent, $($rest:tt)*) => {
        impl $crate::Idempotent for $name {}
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident $(,)?) => {};
}

#[macro_export]
macro_rules! define_act {
    (name: $name:ident, operand: $operand:ty, operator: $operator:ty, act: $act:expr $(,)*) => {
        enum $name {}
        impl $crate::Act for $name {
            type Operand = $operand;
            type Operator = $operator;
            fn act(
                x: &<Self::Operand as $crate::Algebraic>::Value,
                f: &<Self::Operator as $crate::Algebraic>::Value,
            ) -> <Self::Operand as $crate::Algebraic>::Value {
                $act(x, f)
            }
        }
    };

    ($vis:vis, name: $name:ident, operand: $operand:ty, operator: $operator:ty, act: $act:expr $(,)*) => {
        $vis enum $name {}
        impl $crate::Act for $name {
            type Operand = $operand;
            type Operator = $operator;
            fn act(
                x: &<Self::Operand as $crate::Algebraic>::Value,
                f: &<Self::Operator as $crate::Algebraic>::Value,
            ) -> <Self::Operand as $crate::Algebraic>::Value {
                $act(x, f)
            }
        }
    };
}
