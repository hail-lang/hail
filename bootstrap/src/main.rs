//! The bootstrap compiler for Hail.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use std::ops::Range;

use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(#[allow(missing_docs)] #[allow(missing_debug_implementations)] pub grammar);

/// A source location.
#[derive(Clone, Debug, PartialEq)]
pub struct Loc {
    /// The file of the location.
    pub file: u32,

    /// The span of the location.
    pub span: Range<usize>,
}

impl Loc {
    /// Creates a new location.
    #[inline(always)]
    pub fn new(file: u32, span: Range<usize>) -> Self {
        Self { file, span }
    }
}

fn main() {
    println!("Hello, world!");
}
