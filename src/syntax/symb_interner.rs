use string_interner::{symbol::SymbolU32, StringInterner};

use super::symb::Symbols;

/// An implementation of `Symbols` with an `StringInterner`.
pub struct SymbolsInterner {
    interner: StringInterner,
}

impl SymbolsInterner {
    /// Creates an empty `SymbolsInterner`.
    pub fn new() -> Self {
        Self {
            interner: StringInterner::default(),
        }
    }
}

impl Symbols for SymbolsInterner {
    type Symb = SymbolU32;

    fn resolve(&self, symbol: Self::Symb) -> Option<&str> {
        self.interner.resolve(symbol)
    }

    fn get_or_store(&mut self, string: &str) -> Self::Symb {
        self.interner.get_or_intern(string)
    }
}
