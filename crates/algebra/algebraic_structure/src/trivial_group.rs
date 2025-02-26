use algebraic_traits::define_algebra;

define_algebra! {
    pub,
    name: TrivialGroup,
    value: (),
    op: |_, _| (),
    unit: (),
    inv: |_| (),
    associative,
    commutative,
    idempotent,
}
