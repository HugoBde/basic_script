use std::collections::HashMap;

use crate::{lexer::Lexer, symbol_table::Identifier};

pub struct Compiler {
    symbol_table: HashMap<String, Identifier>,
    lexer: Lexer,
}
