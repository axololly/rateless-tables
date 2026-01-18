extern crate alloc;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use core::{cmp::Ordering, fmt::{Debug, Display, Formatter, Result as FmtResult}, hash::{Hash, Hasher}, marker::PhantomData};

use crate::Symbol;

pub trait Bytes {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Default, Deserialize, Serialize)]
pub struct AsBytes<T> {
    _marker: PhantomData<T>,

    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>
}

impl<T: Bytes> AsBytes<T> {
    pub fn new(value: T) -> Self {
        Self {
            _marker: PhantomData,
            bytes: value.to_bytes()
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into(self) -> T {
        T::from_bytes(&self.bytes)
    }
}

impl<T> PartialEq for AsBytes<T> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<T> Eq for AsBytes<T> {}

impl<T> PartialOrd for AsBytes<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for AsBytes<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl<T> Hash for AsBytes<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &i in &self.bytes {
            state.write_u8(i);
        }
    }
}

impl<T: Display + Bytes> Display for AsBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", T::from_bytes(&self.bytes))
    }
}

impl<T: Debug + Bytes> Debug for AsBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.bytes)
    }
}

impl<T> Clone for AsBytes<T> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            bytes: self.bytes.clone()
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct AsVariableBytes<T> {
    _marker: PhantomData<T>,

    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>
}

impl<T: Bytes> AsVariableBytes<T> {
    pub fn new(value: T) -> Self {
        Self {
            _marker: PhantomData,
            bytes: value.to_bytes()
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into(self) -> T {
        T::from_bytes(&self.bytes)
    }
}

impl<T> PartialEq for AsVariableBytes<T> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<T> Eq for AsVariableBytes<T> {}

impl<T> PartialOrd for AsVariableBytes<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for AsVariableBytes<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl<T> Hash for AsVariableBytes<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &i in &self.bytes {
            state.write_u8(i);
        }
    }
}

impl<T: Display + Bytes> Display for AsVariableBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", T::from_bytes(&self.bytes))
    }
}

impl<T: Debug + Bytes> Debug for AsVariableBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.bytes)
    }
}

impl<T> Clone for AsVariableBytes<T> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            bytes: self.bytes.clone()
        }
    }
}

fn naive_xor(s1: &[u8], s2: &[u8]) -> Vec<u8> {
    let (shorter, longer) = if s1.len() < s2.len() {
        (&s1, &s2)
    }
    else {
        (&s2, &s1)
    };

    let mut result = longer.to_vec();

    for (i, v) in shorter.iter().enumerate() {
        result[i] ^= v;
    }

    result
}

impl<T: Default + PartialEq + Ord> Symbol for AsBytes<T> {
    fn xor(&self, other: &Self) -> Self {
        Self {
            _marker: PhantomData,
            bytes: naive_xor(&self.bytes, &other.bytes)
        }
    }
}

impl<T: Default + PartialEq + Ord> Symbol for AsVariableBytes<T> {
    fn xor(&self, other: &Self) -> Self {
        let mut result = naive_xor(&self.bytes, &other.bytes);

        let mut i = result.len();

        while i > 0 && result[i - 1] == 0 {
            i -= 1;
        }
    
        result.truncate(i);

        Self {
            _marker: PhantomData,
            bytes: result
        }
    }
}
