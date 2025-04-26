#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub max_turn: usize,
    pub beam_width: usize,
    pub nodes_capacity: usize,
    pub minimize_turn: bool,
    pub hash_deduplication_scope: HashDeduplicationScope,
}

#[derive(Debug, Clone, Copy)]
pub enum HashDeduplicationScope {
    PerTurn,
    Global,
}
