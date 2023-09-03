#![allow(non_camel_case_types)]

use crate::symbol_table::Identifier;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    LET,
    MUT,
    FN,
    RETURN,
    // WHILE,
    // DO,
    // FOR,
    // FOREACH,
    // BREAK,
    // IF,
    // ELSE,
    // SWITCH,
    // CASE,
    // STRUCT,
    // ENUM,
    // UNION,
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

    // I8,
    // I16,
    // I32,
    // I64,
    // U8,
    // U16,
    // U32,
    // U64,
    // CHAR,
    // STRING,
    // REFERENCE,

    // UNSIGNED_INT_LITERAL,
    INT_LITERAL(i64),
    // FLOAT_LITERAL,
    CHAR_LITERAL,
    STRING_LITERAL,

    ASSIGN_OP,
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

    // LOCICAL_AND_OP,
    // LOGICAL_OR_OP,
    // LOGICAL_NOT_OP,
    //
    // BINARY_AND_OP,
    // BINARY_AND_OP_ASSIGN,
    // BINARY_OR_OP,
    // BINARY_OR_OP_ASSIGN,
    // BINARY_XOR_OP,
    // BINARY_XOR_OP_ASSIGN,
    // BINARY_NOT_OP,
    // BINARY_NOT_OP_ASSIGN,
    //
    // LEFT_SHIFT_OP,
    // LEFT_SHIFT_OP_ASSIGN,
    // RIGHT_SHIFT_OP,
    // RIGHT_SHIFT_OP_ASSIGN,
    // LEFT_ROT_OP,
    // LEFT_ROT_OP_ASSIGN,
    // RIGHT_ROT_OP,
    // RIGHT_ROT_OP_ASSIGN,
    //
    // EQ_OP,
    // NE_OP,
    // LT_OP,
    // LE_OP,
    // GT_OP,
    // GE_OP,
    IDENTIFIER { identifier: Identifier },
}

pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    read_pos: usize,
    curr_char: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: Vec::from(input.as_bytes()),
            pos: 0,
            read_pos: 1,
            curr_char: input.as_bytes()[0],
        }
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        self.skip_whitespace();

        let lexeme = match self.curr_char {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let word = self.read_word();

                match word {
                    "let" => Some(Lexeme::LET),
                    "mut" => Some(Lexeme::MUT),
                    "fn" => Some(Lexeme::FN),
                    "return" => Some(Lexeme::RETURN),
                    "true" => Some(Lexeme::TRUE),
                    "false" => Some(Lexeme::FALSE),
                    _ => {
                        let identifier = Identifier::new(word);
                        Some(Lexeme::IDENTIFIER { identifier })
                    }
                }
            }

            b'0'..=b'9' => {
                let int_value = self.read_int_literal();

                Some(Lexeme::INT_LITERAL(int_value))
            }

            b'{' => Some(Lexeme::BRACE_OPEN),
            b'}' => Some(Lexeme::BRACE_CLOSE),
            b'(' => Some(Lexeme::PARENTHESES_OPEN),
            b')' => Some(Lexeme::PARENTHESES_CLOSE),
            b'[' => Some(Lexeme::BRACKET_OPEN),
            b']' => Some(Lexeme::BRACKET_CLOSE),
            b';' => Some(Lexeme::SEMI_COLON),
            b',' => Some(Lexeme::COMMA),

            b'=' => Some(Lexeme::ASSIGN_OP),
            b'+' => Some(Lexeme::PLUS_OP),
            b'-' => Some(Lexeme::MINUS_OP),
            b'*' => Some(Lexeme::TIMES_OP),
            b'/' => Some(Lexeme::DIVIDE_OP),
            b'%' => Some(Lexeme::MODULO_OP),
            0 => None,
            _ => None,
        };

        self.read_char();

        lexeme
    }

    fn read_word(&mut self) -> &str {
        let start = self.pos;

        loop {
            match self.peek_char() {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => self.read_char(),
                _ => break,
            }
        }

        let end = self.pos;

        str::from_utf8(&self.input[start..=end]).unwrap()
    }

    fn read_int_literal(&mut self) -> i64 {
        let start = self.pos;

        while self.peek_char().is_ascii_digit() {
            self.read_char();
        }

        let end = self.pos;

        let int_literal_str = str::from_utf8(&self.input[start..=end]).unwrap();

        int_literal_str.parse().unwrap()
    }

    fn read_char(&mut self) {
        self.curr_char = if self.read_pos >= self.input.len() {
            0
        } else {
            self.input[self.read_pos]
        };

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            0
        } else {
            self.input[self.read_pos]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lexer_symbols() {
        let input = "({[]});,";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PARENTHESES_OPEN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::BRACE_OPEN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::BRACKET_OPEN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::BRACKET_CLOSE);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::BRACE_CLOSE);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PARENTHESES_CLOSE);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::SEMI_COLON);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::COMMA);
        assert_eq!(lexer.next_lexeme(), None);
    }

    #[test]
    fn test_lexer_operators() {
        let input = "+-=/*%";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PLUS_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::MINUS_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::ASSIGN_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::DIVIDE_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::TIMES_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::MODULO_OP);
        assert_eq!(lexer.next_lexeme(), None);
    }

    #[test]
    fn test_lexer_keywords() {
        let input = "let mut fn return true false foo";

        let mut lexer = Lexer::new(input);

        let foo_identifier = Identifier::new("foo");

        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::LET);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::MUT);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::FN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::RETURN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::TRUE);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::FALSE);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: foo_identifier
            }
        );
        assert_eq!(lexer.next_lexeme(), None);
    }

    #[test]
    fn test_lexer_realistic_input() {
        let input = r#"fn main() {
            let a = 5;
            let b = 3;
            let c = a + b;

            return (c-8);
        }"#;

        let mut lexer = Lexer::new(input);

        let main_identifier = Identifier::new("main");
        let a_identifier = Identifier::new("a");
        let b_identifier = Identifier::new("b");
        let c_identifier = Identifier::new("c");

        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::FN);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: main_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PARENTHESES_OPEN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PARENTHESES_CLOSE);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::BRACE_OPEN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::LET);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: a_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::ASSIGN_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::INT_LITERAL(5));
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::SEMI_COLON);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::LET);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: b_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::ASSIGN_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::INT_LITERAL(3));
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::SEMI_COLON);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::LET);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: c_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::ASSIGN_OP);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: a_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PLUS_OP);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: b_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::SEMI_COLON);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::RETURN);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PARENTHESES_OPEN);
        assert_eq!(
            lexer.next_lexeme().unwrap(),
            Lexeme::IDENTIFIER {
                identifier: c_identifier.clone()
            }
        );
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::MINUS_OP);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::INT_LITERAL(8));
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::PARENTHESES_CLOSE);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::SEMI_COLON);
        assert_eq!(lexer.next_lexeme().unwrap(), Lexeme::BRACE_CLOSE);
        assert_eq!(lexer.next_lexeme(), None);
    }
}
