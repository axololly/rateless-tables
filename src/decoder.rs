extern crate alloc;
use alloc::vec::Vec;

use crate::{encoder::CodingWindow, index::IndexGenerator, symbol::{CodedSymbol, HashedSymbol, Op, Symbol}};

#[derive(Default)]
pub struct Decoder<T: Symbol> {
    symbols: Vec<CodedSymbol<T>>,
    local: CodingWindow<T>,
    window: CodingWindow<T>,
    remote: CodingWindow<T>,
    decodable: Vec<usize>,
    decoded: usize
}

impl<T: Symbol> Decoder<T> {
    pub fn is_done(&self) -> bool {
        self.decoded == self.symbols.len()
    }

    pub fn local(&self) -> &[HashedSymbol<T>] {
        self.local.symbols.as_slice()
    }

    pub fn remote(&self) -> &[HashedSymbol<T>] {
        self.remote.symbols.as_slice()
    }

    pub fn add_symbol(&mut self, symbol: T) {
        let hashed_symbol = HashedSymbol {
            hash: symbol.hash(),
            symbol
        };

        self.add_hashed_symbol(hashed_symbol);
    }

    pub fn add_hashed_symbol(&mut self, symbol: HashedSymbol<T>) {
        self.window.add_hashed_symbol(symbol);
    }

    pub fn add_coded_symbol(&mut self, coded_symbol: CodedSymbol<T>) {
        let mut cs = coded_symbol;
        
        cs = self.window.apply_window(cs, Op::Remove);
        cs = self.remote.apply_window(cs, Op::Remove);
        cs = self.local.apply_window(cs, Op::Add);

        if (cs.count == 1 || cs.count == -1) && cs.hash == cs.symbol.hash()
            || cs.count == 0 && cs.hash == 0
        {
            self.decodable.push(self.symbols.len());
        }

        self.symbols.push(cs);
    }

    fn apply_new_symbol(&mut self, symbol: &HashedSymbol<T>, op: Op) -> IndexGenerator {
        let mut index_gen = IndexGenerator::new(symbol.hash, 0);

        while index_gen.last_index < self.symbols.len() as u64 {
            let idx = index_gen.last_index as usize;

            let cs = &mut self.symbols[idx];
            
            cs.apply(symbol, op);

            if (cs.count == -1 || cs.count == 1) && cs.hash == cs.symbol.hash() {
                self.decodable.push(idx);
            }

            index_gen.next();
        }

        index_gen
    }

    pub fn try_decode(&mut self) -> Result<(), i64> {
        for idx in core::mem::take(&mut self.decodable) {
            let cs = &self.symbols[idx];

            match cs.count {
                1 => {
                    let mut new = HashedSymbol::<T>::default();

                    new.symbol = new.symbol.xor(&cs.symbol);

                    new.hash = cs.hash;

                    let index_gen = self.apply_new_symbol(&new, Op::Remove);
                    
                    self.remote.add_hashed_symbol_with_mapping(new, index_gen);
                }

                0 => {}

                -1 => {
                    let mut new = HashedSymbol::<T>::default();

                    new.symbol = new.symbol.xor(&cs.symbol);

                    new.hash = cs.hash;

                    let index_gen = self.apply_new_symbol(&new, Op::Add);

                    self.local.add_hashed_symbol_with_mapping(new, index_gen);
                }

                invalid => return Err(invalid)
            }

            self.decoded += 1;
        }

        Ok(())
    }

    pub fn decode(&mut self) {
        if let Err(invalid) = self.try_decode() {
            panic!("invalid degree for decodable coded symbol: {invalid}");
        }
    }

    pub fn consume(self) -> (Vec<T>, Vec<T>) {
        let local = self.local
            .symbols
            .into_iter()
            .map(|s| s.symbol)
            .collect();

        let remote = self.remote
            .symbols
            .into_iter()
            .map(|s| s.symbol)
            .collect();

        (remote, local)
    }

    pub fn reset(&mut self) {
        self.symbols.clear();

        self.local.reset();
        self.remote.reset();
        self.window.reset();

        self.decoded = 0;
    }
}

impl<T: Symbol> Extend<T> for Decoder<T> {
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        for symbol in iter {
            self.add_symbol(symbol);
        }
    }
}
