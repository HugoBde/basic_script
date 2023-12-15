#![allow(non_camel_case_types)]

use std::str;

use crate::symbol_table::Identifier;

#[derive(Debug, PartialEq)]

pub enum Lexeme {
    EOF,
    INVALID,

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
    COLON,
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
    // REFERENCE,
    INT_LITERAL(i64), // -?[0-9]+
    // FLOAT_LITERAL, -?[0-9]+.[0-9*]+
    CHAR_LITERAL,
    STRING_LITERAL(String),

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

    RETURN_TYPE_OP,

    LOGICAL_AND_OP,
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

    IDENTIFIER(Identifier),
}

pub struct Lexer {
    input:     Vec<u8>,
    pos:       usize,
    read_pos:  usize,
    curr_char: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {

        Lexer {
            input:     Vec::from(input.as_bytes()),
            pos:       0,
            read_pos:  1,
            curr_char: input.as_bytes()[0],
        }
    }

    pub fn lex(&mut self) -> Vec<Lexeme> {

        let mut output = Vec::new();

        loop {

            let lexeme = self.next_lexeme();

            if lexeme == Lexeme::EOF {

                break;
            }

            output.push(lexeme);
        }

        return output;
    }

    pub fn next_lexeme(&mut self) -> Lexeme {

        self.skip_whitespace();

        let lexeme = match self.curr_char {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {

                let word = self.read_word();

                match word {
                    "let" => Lexeme::LET,
                    "mut" => Lexeme::MUT,
                    "fn" => Lexeme::FN,
                    "return" => Lexeme::RETURN,
                    "true" => Lexeme::TRUE,
                    "false" => Lexeme::FALSE,
                    "i8" => Lexeme::I8,
                    "u8" => Lexeme::U8,
                    "i16" => Lexeme::I16,
                    "u16" => Lexeme::U16,
                    "i32" => Lexeme::I32,
                    "u32" => Lexeme::U32,
                    "i64" => Lexeme::I64,
                    "u64" => Lexeme::U64,
                    "char" => Lexeme::CHAR,
                    "string" => Lexeme::STRING,
                    _ => {

                        let identifier = Identifier::new(word);

                        Lexeme::IDENTIFIER(identifier)
                    }
                }
            }

            b'0'..=b'9' => {

                let int_value = self.read_int_literal();

                Lexeme::INT_LITERAL(int_value)
            }

            b'{' => Lexeme::BRACE_OPEN,
            b'}' => Lexeme::BRACE_CLOSE,
            b'(' => Lexeme::PARENTHESES_OPEN,
            b')' => Lexeme::PARENTHESES_CLOSE,
            b'[' => Lexeme::BRACKET_OPEN,
            b']' => Lexeme::BRACKET_CLOSE,
            b';' => Lexeme::SEMI_COLON,
            b':' => Lexeme::COLON,
            b',' => Lexeme::COMMA,

            b'"' => {

                let str_literal_value = self.read_str_literal();

                Lexeme::STRING_LITERAL(String::from(str_literal_value))
            }

            b'=' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::EQ_OP
                }
                _ => Lexeme::ASSIGN_OP,
            },

            b'+' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::PLUS_OP_ASSIGN
                }
                _ => Lexeme::PLUS_OP,
            },

            b'-' => match self.peek_char() {
                b'>' => {

                    self.read_char();

                    Lexeme::RETURN_TYPE_OP
                }
                b'=' => {

                    self.read_char();

                    Lexeme::MINUS_OP_ASSIGN
                }
                _ => Lexeme::MINUS_OP,
            },

            b'*' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::TIMES_OP_ASSIGN
                }
                _ => Lexeme::TIMES_OP,
            },

            b'/' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::DIVIDE_OP_ASSIGN
                }
                _ => Lexeme::DIVIDE_OP,
            },

            b'%' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::MODULO_OP_ASSIGN
                }
                _ => Lexeme::MODULO_OP,
            },

            b'!' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::NE_OP
                }
                _ => Lexeme::LOGICAL_NOT_OP,
            },

            b'&' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::BINARY_AND_OP_ASSIGN
                }
                b'&' => {

                    self.read_char();

                    Lexeme::LOGICAL_AND_OP
                }
                _ => Lexeme::BINARY_AND_OP,
            },

            b'|' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::BINARY_OR_OP_ASSIGN
                }
                b'|' => {

                    self.read_char();

                    Lexeme::LOGICAL_OR_OP
                }
                _ => Lexeme::BINARY_OR_OP,
            },

            b'^' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::BINARY_XOR_OP_ASSIGN
                }
                _ => Lexeme::BINARY_XOR_OP,
            },

            b'~' => match self.peek_char() {
                b'=' => {

                    self.read_char();

                    Lexeme::BINARY_NOT_OP_ASSIGN
                }
                _ => Lexeme::BINARY_NOT_OP,
            },

            b'<' => match self.peek_char() {
                b'<' => {

                    self.read_char();

                    match self.peek_char() {
                        b'<' => {

                            self.read_char();

                            match self.peek_char() {
                                b'=' => {

                                    self.read_char();

                                    Lexeme::LEFT_ROT_OP_ASSIGN
                                }
                                _ => Lexeme::LEFT_ROT_OP,
                            }
                        }
                        b'=' => {

                            self.read_char();

                            Lexeme::LEFT_SHIFT_OP_ASSIGN
                        }
                        _ => Lexeme::LEFT_SHIFT_OP,
                    }
                }
                b'=' => {

                    self.read_char();

                    Lexeme::LE_OP
                }
                _ => Lexeme::LT_OP,
            },

            b'>' => match self.peek_char() {
                b'>' => {

                    self.read_char();

                    match self.peek_char() {
                        b'>' => {

                            self.read_char();

                            match self.peek_char() {
                                b'=' => {

                                    self.read_char();

                                    Lexeme::RIGHT_ROT_OP_ASSIGN
                                }
                                _ => Lexeme::RIGHT_ROT_OP,
                            }
                        }
                        b'=' => {

                            self.read_char();

                            Lexeme::RIGHT_SHIFT_OP_ASSIGN
                        }
                        _ => Lexeme::RIGHT_SHIFT_OP,
                    }
                }
                b'=' => {

                    self.read_char();

                    Lexeme::GE_OP
                }
                _ => Lexeme::GT_OP,
            },

            0 => Lexeme::EOF,
            _ => Lexeme::INVALID,
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

    fn read_str_literal(&mut self) -> &str {

        let start = self.pos;

        loop {

            match self.peek_char() {
                b'\\' => self.read_char(),
                b'"' => break,
                _ => self.read_char(),
            }
        }

        let end = self.pos;

        self.read_char();

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

        self.curr_char = if self.read_pos >= self.input.len() { 0 } else { self.input[self.read_pos] };

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

        let input = String::from("({[]});,");

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_lexeme(), Lexeme::PARENTHESES_OPEN);

        assert_eq!(lexer.next_lexeme(), Lexeme::BRACE_OPEN);

        assert_eq!(lexer.next_lexeme(), Lexeme::BRACKET_OPEN);

        assert_eq!(lexer.next_lexeme(), Lexeme::BRACKET_CLOSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::BRACE_CLOSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::PARENTHESES_CLOSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::SEMI_COLON);

        assert_eq!(lexer.next_lexeme(), Lexeme::COMMA);

        assert_eq!(lexer.next_lexeme(), Lexeme::EOF);
    }

    #[test]

    fn test_lexer_operators() {

        let input = String::from("=+-/*%");

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_lexeme(), Lexeme::ASSIGN_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::PLUS_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::MINUS_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::DIVIDE_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::TIMES_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::MODULO_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::EOF);
    }

    #[test]

    fn test_lexer_keywords() {

        let input = String::from("let mut fn return true false foo");

        let mut lexer = Lexer::new(input);

        let foo_identifier = Identifier::new("foo");

        assert_eq!(lexer.next_lexeme(), Lexeme::LET);

        assert_eq!(lexer.next_lexeme(), Lexeme::MUT);

        assert_eq!(lexer.next_lexeme(), Lexeme::FN);

        assert_eq!(lexer.next_lexeme(), Lexeme::RETURN);

        assert_eq!(lexer.next_lexeme(), Lexeme::TRUE);

        assert_eq!(lexer.next_lexeme(), Lexeme::FALSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::IDENTIFIER(foo_identifier));

        assert_eq!(lexer.next_lexeme(), Lexeme::EOF);
    }

    #[test]

    fn test_lexer_realistic_input() {

        let input = String::from(
            r#"fn main() -> u8 {
            let a = 5;
            let b : u8 = a + 3;

            return (b-8);
        }"#,
        );

        let mut lexer = Lexer::new(input);

        let main_identifier = Identifier::new("main");

        let a_identifier = Identifier::new("a");

        let b_identifier = Identifier::new("b");

        assert_eq!(lexer.next_lexeme(), Lexeme::FN);

        assert_eq!(lexer.next_lexeme(), Lexeme::IDENTIFIER(main_identifier.clone()));

        assert_eq!(lexer.next_lexeme(), Lexeme::PARENTHESES_OPEN);

        assert_eq!(lexer.next_lexeme(), Lexeme::PARENTHESES_CLOSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::RETURN_TYPE_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::U8);

        assert_eq!(lexer.next_lexeme(), Lexeme::BRACE_OPEN);

        assert_eq!(lexer.next_lexeme(), Lexeme::LET);

        assert_eq!(lexer.next_lexeme(), Lexeme::IDENTIFIER(a_identifier.clone()));

        assert_eq!(lexer.next_lexeme(), Lexeme::ASSIGN_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::INT_LITERAL(5));

        assert_eq!(lexer.next_lexeme(), Lexeme::SEMI_COLON);

        assert_eq!(lexer.next_lexeme(), Lexeme::LET);

        assert_eq!(lexer.next_lexeme(), Lexeme::IDENTIFIER(b_identifier.clone()));

        assert_eq!(lexer.next_lexeme(), Lexeme::COLON);

        assert_eq!(lexer.next_lexeme(), Lexeme::U8);

        assert_eq!(lexer.next_lexeme(), Lexeme::ASSIGN_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::IDENTIFIER(a_identifier.clone()));

        assert_eq!(lexer.next_lexeme(), Lexeme::PLUS_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::INT_LITERAL(3));

        assert_eq!(lexer.next_lexeme(), Lexeme::SEMI_COLON);

        assert_eq!(lexer.next_lexeme(), Lexeme::RETURN);

        assert_eq!(lexer.next_lexeme(), Lexeme::PARENTHESES_OPEN);

        assert_eq!(lexer.next_lexeme(), Lexeme::IDENTIFIER(b_identifier.clone()));

        assert_eq!(lexer.next_lexeme(), Lexeme::MINUS_OP);

        assert_eq!(lexer.next_lexeme(), Lexeme::INT_LITERAL(8));

        assert_eq!(lexer.next_lexeme(), Lexeme::PARENTHESES_CLOSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::SEMI_COLON);

        assert_eq!(lexer.next_lexeme(), Lexeme::BRACE_CLOSE);

        assert_eq!(lexer.next_lexeme(), Lexeme::EOF);
    }
}
