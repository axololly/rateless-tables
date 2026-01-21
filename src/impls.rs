use std::hash::{DefaultHasher, Hasher};

use crate::symbol::Symbol;

fn hash(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();

    hasher.write(bytes);

    hasher.finish()
}

impl Symbol for u8 {
    fn get_hash(&self) -> u64 {
        hash(&[*self])
    }

    fn xor(&self, other: &Self) -> Self {
        *self ^ *other
    }
}

macro_rules! impl_symbol_for {
    ($($type:tt)*) => {
        impl Symbol for $($type)* {
            fn get_hash(&self) -> u64 {
                hash(&self.to_le_bytes())
            }

            fn xor(&self, other: &Self) -> Self {
                *self ^ *other
            }
        }
    };
}

impl_symbol_for! { i16 }
impl_symbol_for! { u16 }
impl_symbol_for! { i32 }
impl_symbol_for! { u32 }
impl_symbol_for! { i64 }
impl_symbol_for! { u64 }
impl_symbol_for! { i128 }
impl_symbol_for! { u128 }
