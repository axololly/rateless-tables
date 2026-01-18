use crate::{index::IndexGenerator, symbol::{CodedSymbol, HashedSymbol, Op, Symbol}};

struct SymbolMap {
    source: usize,
    coded: usize
}

#[derive(Default)]
struct SymbolMapping(Vec<SymbolMap>);

impl SymbolMapping {
    fn fix_head(&mut self) {
        let mut current = 0;

        loop {
            let mut child = current * 2 + 1;

            if child >= self.0.len() {
                break;
            }

            if child + 1 < self.0.len() &&
                self.0[child + 1].coded < self.0[child].coded
            {
                child += 1;
            }

            if self.0[current].coded <= self.0[child].coded {
                break;
            }

            self.0.swap(current, child);
            
            current = child;
        }
    }

    fn fix_tail(&mut self) {
        let mut current = self.0.len() - 1;
        
        loop {
            let parent = current.saturating_sub(1) / 2;

            if current == parent ||
                self.0[parent].coded <= self.0[current].coded
            {
                break;
            }

            self.0.swap(parent, current);

            current = parent;
        }
    }
}

#[derive(Default)]
pub(crate) struct CodingWindow<T: Symbol> {
    pub symbols: Vec<HashedSymbol<T>>,
    
    generators: Vec<IndexGenerator>,
    queue: SymbolMapping,
    next_index: usize
}

impl<T: Symbol> CodingWindow<T> {
    pub fn add_symbol(&mut self, symbol: T) {
        let hash = symbol.get_hash();
        
        let hashed_symbol = HashedSymbol {
            hash,
            symbol
        };

        let index_gen = IndexGenerator::new(hash, 0);

        self.add_hashed_symbol_with_mapping(hashed_symbol, index_gen);
    }

    pub(crate) fn add_hashed_symbol_with_mapping(&mut self, symbol: HashedSymbol<T>, index_gen: IndexGenerator) {
        let coded = index_gen.last_index as usize;
        
        self.symbols.push(symbol);
        
        self.generators.push(index_gen);

        self.queue.0.push(SymbolMap {
            source: self.symbols.len() - 1,
            coded
        });

        self.queue.fix_tail();
    }

    pub(crate) fn apply_window(&mut self, mut symbol: CodedSymbol<T>, op: Op) -> CodedSymbol<T> {
        if self.queue.0.is_empty() {
            self.next_index += 1;

            return symbol;
        }

        while self.queue.0[0].coded == self.next_index {
            let hashed_symbol = &self.symbols[self.queue.0[0].source];
            
            symbol.apply(hashed_symbol, op);

            let next_gen = &mut self.generators[self.queue.0[0].source];

            self.queue.0[0].coded = next_gen.next().unwrap() as usize;

            self.queue.fix_head();
        }

        self.next_index += 1;

        symbol
    }

    pub(crate) fn reset(&mut self) {
        self.symbols.clear();
        self.generators.clear();
        self.queue.0.clear();

        self.next_index = 0;
    }
}

#[derive(Default)]
pub struct Encoder<T: Symbol>(CodingWindow<T>);

impl<T: Symbol> Encoder<T> {
    pub fn add_symbol(&mut self, symbol: T) {
        self.0.add_symbol(symbol);
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }
}

impl<T: Symbol> Extend<T> for Encoder<T> {
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        for symbol in iter {
            self.add_symbol(symbol);
        }
    }
}

impl<T: Symbol + PartialEq> Iterator for Encoder<T> {
    type Item = CodedSymbol<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.0.apply_window(CodedSymbol::default(), Op::Add);

        Some(symbol)
    }
}
