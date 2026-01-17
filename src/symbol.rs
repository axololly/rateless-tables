use serde::{Deserialize, Serialize};

pub trait Symbol: Default {
    fn xor(&self, other: &Self) -> Self;
    fn hash(&self) -> u64;
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct HashedSymbol<T: Symbol> {
    pub symbol: T,
    pub hash: u64
}

#[derive(Clone, Copy, Default, Deserialize, PartialEq, Serialize)]
pub struct CodedSymbol<T: Symbol> {
    pub symbol: T,
    pub hash: u64,
    pub count: i64
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add = 1,
    Remove = -1
}

impl<T: Symbol> CodedSymbol<T> {
    pub fn apply(&mut self, symbol: &HashedSymbol<T>, op: Op) {
        self.symbol = self.symbol.xor(&symbol.symbol);
        self.hash ^= symbol.hash;
        self.count += op as i64;
    }
}
