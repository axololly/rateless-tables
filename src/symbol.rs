use std::hash::{DefaultHasher, Hash, Hasher};
use serde::{Deserialize, Serialize};

pub trait Symbol: Default + Hash + PartialEq + Ord {
    fn xor(&self, other: &Self) -> Self;
    
    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);

        hasher.finish()
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub(crate) struct HashedSymbol<T: Symbol> {
    pub symbol: T,
    pub hash: u64
}

#[derive(Clone, Copy, Default, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CodedSymbol<T: Symbol> {
    pub(crate) symbol: T,
    pub(crate) hash: u64,
    pub(crate) count: i64
}

impl<T: Symbol> CodedSymbol<T> {
    pub fn symbol(&self) -> &T {
        &self.symbol
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add = 1,
    Remove = -1
}

impl<T: Symbol> CodedSymbol<T> {
    pub(crate) fn apply(&mut self, symbol: &HashedSymbol<T>, op: Op) {
        self.symbol = self.symbol.xor(&symbol.symbol);
        self.hash ^= symbol.hash;
        self.count += op as i64;
    }
}
