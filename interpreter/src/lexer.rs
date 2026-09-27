use std::panic;

use crate::token::{
    Register,
    Token::{self},
    Width, keyword_token,
};

pub struct Lexer {
    pub code: String,
    current_char: Option<u8>,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let mut lexer = Self {
            current_char: None,
            position: 0,
            read_position: 0,
            code,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            None => Token::Eof,
            Some(0) => Token::Eof,
            Some(b',') => Token::Comma,
            Some(b'[') => Token::LBracket,
            Some(b']') => Token::RBracket,

            Some(byte) if byte == b'#' => {
                self.read_char(); // skip the #
                return self.read_number();
            }

            Some(byte) if byte.is_ascii_alphanumeric() || byte == b'_' || byte == b':' => {
                return self.read_identifier();
            }

            Some(byte) => panic!("unexpected charectar {:#?}", byte),
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.code.len() {
            self.current_char = Some(0);
        } else {
            self.current_char = self.code.as_bytes().get(self.read_position).copied();
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while matches!(
          self.current_char,
          Some(byte) if byte.is_ascii_digit() || byte.is_ascii_hexdigit() || byte == b'x',
        ) {
            self.read_char();
        }

        let text = &self.code[start..self.position];

        let value = if let Some(hex) = text.strip_prefix("0x") {
            i64::from_str_radix(hex, 16).expect("conversion to work")
        } else {
            text.parse::<i64>().expect("conversion to work")
        };

        Token::Int(value)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while matches!(
          self.current_char,
          Some(byte) if byte.is_ascii_alphanumeric() || byte == b'_' || byte == b':'
        ) {
            self.read_char();
        }

        let word = &self.code[start..self.position];
        let token = match keyword_token(word) {
            Some(token) => token,
            None => {
                if word.ends_with(":") {
                    let sanitized = word.strip_suffix(":").expect("to strip that shit");
                    return Token::Label(sanitized.to_owned());
                } else if let Some(register) = is_register(word) {
                    return register;
                }

                return Token::Label(word.to_owned());
            }
        };
        return token;
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current_char, Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.read_char();
        }
    }
}

fn is_register(word: &str) -> Option<Token> {
    if word.len() > 3 {
        return None;
    }

    let width_char = word.chars().next();
    let width = match width_char {
        None => panic!("the fuck you mean we don't have a first word?"),
        Some('x') => Width::X64,
        Some('w') => Width::W32,
        Some(_) => return None,
    };

    let number_chars = &word[1..];
    let number = number_chars
        .parse::<i8>()
        .expect("this to be a valid number doggo");

    if number == 30 {
        return Some(Token::X30);
    }

    if number > 29 || number < 0 {
        panic!("incorrect range value {number}")
    }
    let register = Register { number, width };
    Some(Token::Register(register))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn empty_input_returns_eof() {
        assert_eq!(Lexer::new("".into()).next_token(), Token::Eof);
    }

    #[test]
    fn reads_hex_immediate() {
        assert_eq!(Lexer::new("#0x1".into()).next_token(), Token::Int(1));
    }

    #[test]
    fn preserves_punctuation_after_number() {
        let mut lexer = Lexer::new("#1,]".into());
        assert_eq!(lexer.next_token(), Token::Int(1));
        assert_eq!(lexer.next_token(), Token::Comma);
    }

    #[test]
    fn reads_w_prefixed_label() {
        assert_eq!(
            Lexer::new("work:".into()).next_token(),
            Token::Label("work".into())
        );
    }

    #[test]
    fn reads_x_prefixed_label() {
        assert_eq!(
            Lexer::new("xyz:".into()).next_token(),
            Token::Label("xyz".into())
        );
    }

    #[test]
    fn test_next_token() {
        let code = "
          _main:
            sub sp, sp, #16
            str x30, [sp]
            mov w0, #42
            ldr x30, [sp]
            add sp, sp, #16
            ret
          ";

        let tests = vec![
            Token::Label("_main".to_owned()),
            //
            Token::Instruction(Instruction::Sub),
            Token::Sp,
            Token::Comma,
            Token::Sp,
            Token::Comma,
            Token::Int(16),
            //
            Token::Instruction(Instruction::Str),
            Token::X30,
            Token::Comma,
            Token::LBracket,
            Token::Sp,
            Token::RBracket,
            //
            Token::Instruction(Instruction::Mov),
            Token::Register(Register {
                number: 0,
                width: Width::W32,
            }),
            Token::Comma,
            Token::Int(42),
            //
            Token::Instruction(Instruction::Ldr),
            Token::X30,
            Token::Comma,
            Token::LBracket,
            Token::Sp,
            Token::RBracket,
            //
            Token::Instruction(Instruction::Add),
            Token::Sp,
            Token::Comma,
            Token::Sp,
            Token::Comma,
            Token::Int(16),
            Token::Ret,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(code.to_owned());
        for test in tests {
            let t = lexer.next_token();
            assert_eq!(t, test);
        }
    }
}
