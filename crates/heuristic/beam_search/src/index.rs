#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Index {
    x: u32,
}

impl Index {
    pub fn is_some(self) -> bool {
        self.x != !0
    }

    pub fn is_none(self) -> bool {
        self.x == !0
    }

    pub fn unwrap(self) -> usize {
        Option::<usize>::from(self).unwrap()
    }
}

impl From<usize> for Index {
    fn from(x: usize) -> Self {
        Index { x: x as u32 }
    }
}

impl From<Option<usize>> for Index {
    fn from(x: Option<usize>) -> Self {
        Index {
            x: x.unwrap_or(!0) as u32,
        }
    }
}

impl From<Index> for Option<usize> {
    fn from(index: Index) -> Self {
        if index.x == !0 {
            None
        } else {
            Some(index.x as usize)
        }
    }
}
