#![allow(non_camel_case_types)]

pub enum Lexeme {
    LET,
    MUT,
    FN,
    RETURN,
    WHILE,
    DO,
    FOR,
    FOREACH,
    BREAK,
    IF,
    ELSE,
    SWITCH,
    CASE,
    STRUCT,
    ENUM,
    UNION,
    TRUE,
    FALSE,

    BRACE_OPEN,
    BRACE_CLOSE,
    PARENTHESES_OPEN,
    PARENTHESES_CLOSE,
    BRACKET_OPEN,
    BRACKET_CLOSE,
    SEMI_COLON,
    COMMA,

    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    CHAR,
    STRING,
    REFERENCE,

    UNSIGNED_INT_LITERAL,
    INT_LITERAL,
    FLOAT_LITERAL,
    CHAR_LITERAL,
    STRING_LITERAL,

    ASSIGN_OP,
    ASSIGN_OP_ASSIGN,
    PLUS_OP,
    PLUS_OP_ASSIGN,
    MINUS_OP,
    MINUS_OP_ASSIGN,
    TIMES_OP,
    TIMES_OP_ASSIGN,
    DIVIDE_OP,
    DIVIDE_OP_ASSIGN,
    MODULO_OP,
    MODULO_OP_ASSIGN,

    LOCICAL_AND_OP,
    LOGICAL_OR_OP,
    LOGICAL_NOT_OP,

    BINARY_AND_OP,
    BINARY_AND_OP_ASSIGN,
    BINARY_OR_OP,
    BINARY_OR_OP_ASSIGN,
    BINARY_XOR_OP,
    BINARY_XOR_OP_ASSIGN,
    BINARY_NOT_OP,
    BINARY_NOT_OP_ASSIGN,

    LEFT_SHIFT_OP,
    LEFT_SHIFT_OP_ASSIGN,
    RIGHT_SHIFT_OP,
    RIGHT_SHIFT_OP_ASSIGN,
    LEFT_ROT_OP,
    LEFT_ROT_OP_ASSIGN,
    RIGHT_ROT_OP,
    RIGHT_ROT_OP_ASSIGN,

    EQ_OP,
    NE_OP,
    LT_OP,
    LE_OP,
    GT_OP,
    GE_OP,
}

pub struct Lexer {
    file_content: String,
}

impl Lexer {
    pub fn new(file_content: String) -> Lexer {

        Lexer {
            file_content,
        }
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {

        None
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test_symbols() {

        todo!("test_symbols");
    }
}
