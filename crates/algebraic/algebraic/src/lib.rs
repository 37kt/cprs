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

pub trait Group: Monoid {
    fn inv(x: &Self::S) -> Self::S;
}

#[macro_export]
macro_rules! algebra {
    ($ident:ident, $ty:ty) => {
        #[derive(Clone)]
        enum $ident {}
        impl $crate::Algebra for $ident {
            type S = $ty;
        }
    };
}

#[macro_export]
macro_rules! act {
    ($ident:ident, $tar:ty, $act:expr) => {
        impl $crate::Act for $ident {
            type X = $tar;
            #[inline]
            fn act(f: &Self::S, x: &Self::X) -> Self::X {
                $act(f, x)
            }
        }
    };
}

#[macro_export]
macro_rules! monoid {
    ($ident:ident, $e:expr, $op:expr) => {
        impl $crate::Monoid for $ident {
            #[inline]
            fn e() -> Self::S {
                $e
            }
            #[inline]
            fn op(x: &Self::S, y: &Self::S) -> Self::S {
                $op(x, y)
            }
        }
    };
}

#[macro_export]
macro_rules! group {
    ($ident:ident, $e:expr, $op:expr, $inv:expr) => {
        impl $crate::Monoid for $ident {
            #[inline]
            fn e() -> Self::S {
                $e
            }
            #[inline]
            fn op(x: &Self::S, y: &Self::S) -> Self::S {
                $op(x, y)
            }
        }
        impl $crate::Group for $ident {
            #[inline]
            fn inv(x: &Self::S) -> Self::S {
                $inv(x)
            }
        }
    };
}
