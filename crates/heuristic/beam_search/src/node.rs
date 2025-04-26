use crate::index::Index;

pub(crate) struct Node<Action> {
    pub action: Action,
    pub parent: Index,
    pub child: Index,     // 長男
    pub left: Index,      // 兄
    pub right: Index,     // 弟
    pub count_cands: u32, // 子候補の Candidate の数
}
