use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
};

pub type NopHashSet<V> = HashSet<V, BuildHasherDefault<NopHasher>>;
pub type NopHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NopHasher>>;

#[derive(Default)]
pub struct NopHasher {
    hash: u64,
}

impl Hasher for NopHasher {
    fn write(&mut self, _: &[u8]) {
        panic!()
    }

    #[inline]
    fn write_u64(&mut self, n: u64) {
        self.hash = n;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}
