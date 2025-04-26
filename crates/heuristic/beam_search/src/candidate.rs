use std::cmp::Ordering;

use crate::index::Index;

#[derive(Clone)]
pub(crate) struct Candidate<Action> {
    pub action: Action,
    pub parent: Index,
    pub score: i64,
    pub hash: u64,
    pub valid: bool,
}

impl<Action> PartialEq for Candidate<Action> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<Action> Eq for Candidate<Action> {}

impl<Action> PartialOrd for Candidate<Action> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// BinaryHeap に入れたときに、低い(悪い)スコアが top に来るようにする
impl<Action> Ord for Candidate<Action> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
