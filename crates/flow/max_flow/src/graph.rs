#[derive(Clone, Copy)]
pub(crate) struct Edge {
    pub(crate) dst: usize,
    pub(crate) cap: i64,
    pub(crate) rev: usize,
}
