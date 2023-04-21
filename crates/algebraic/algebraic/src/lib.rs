pub trait Monoid {
    type S;
    fn e() -> Self::S;
    fn op(x: &Self::S, y: &Self::S) -> Self::S;
}

pub trait ActMonoid: Monoid {
    type X;
    fn act(f: &Self::S, x: &Self::X) -> Self::X;
}

#[macro_export]
macro_rules! monoid {
    ( $ident:ident, $ty:ty, $e:expr, $op:expr ) => {
        enum $ident {}
        impl Monoid for $ident {
            type S = $ty;
            fn e() -> $ty {
                $e
            }
            fn op(x: &$ty, y: &$ty) -> $ty {
                $op(x, y)
            }
        }
    };
}

#[macro_export]
macro_rules! act_monoid {
    ( $ident:ident, $f_ty:ty, $x_ty:ty, $e:expr, $op:expr, $act:expr ) => {
        monoid!($ident, $f_ty, $e, $op);
        impl ActMonoid for $ident
        where
            $ident: Monoid,
        {
            type X = $x_ty;
            fn act(f: &$f_ty, x: &$x_ty) -> $x_ty {
                $act(f, x)
            }
        }
    };
}
