use algebraic_new::{
    monoid::AddMonoid,
    traits::{Magma, Unital},
};

fn main() {
    let a = AddMonoid::unit();
    let b = AddMonoid::unit();
    let c = AddMonoid::op(a, b);
    println!("{}", c);
}
