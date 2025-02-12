use algebraic_traits::{define_algebra, Magma, Unital};

define_algebra! {
    name: Add,
    element: i32,
    op: |x, y| x + y,
    unit: 0,
    associative,
    commutative,
}

fn main() {
    let a = Add::unit();
    let b = Add::unit();
    let c = Add::op(&a, &b);
    println!("{}", c);
}
