pub trait Algebra {
    type S;
}

pub trait Act: Algebra {
    type X;
    fn act(f: &Self::S, x: &Self::X) -> Self::X;
}

pub trait Monoid: Algebra {
    fn e() -> Self::S;
    fn op(x: &Self::S, y: &Self::S) -> Self::S;
}

#[macro_export]
macro_rules! algebra {
    ( $ident:ident, $ty:ty ) => {
        #[derive(Clone)]
        enum $ident {}
        impl $crate::Algebra for $ident {
            type S = $ty;
        }
    };
}

#[macro_export]
macro_rules! act {
    ( $ident:ident, $tar:ty, $act:expr ) => {
        impl $crate::Act for $ident {
            type X = $tar;
            fn act(f: &Self::S, x: &Self::X) -> Self::X {
                $act(f, x)
            }
        }
    };
}

#[macro_export]
macro_rules! monoid {
    ( $ident:ident, $e:expr, $op:expr ) => {
        impl $crate::Monoid for $ident {
            fn e() -> Self::S {
                $e
            }
            fn op(x: &Self::S, y: &Self::S) -> Self::S {
                $op(x, y)
            }
        }
    };
}
