use crate::ast::{
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
            Some(b',') => Token::Comma,
            Some(b'[') => Token::LBracket,
            Some(b']') => Token::RBracket,
            Some(b'x' | b'w') => {
                return self.read_register();
            }

            Some(byte) if byte == b'#' => {
                self.read_char(); // skip the #
                self.read_number()
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
          Some(byte) if byte.is_ascii_digit()
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
                }
                return Token::Label(word.to_owned());
            }
        };
        return token;
    }

    fn read_register(&mut self) -> Token {
        let start = self.position;
        while matches!(self.current_char, Some(b'x' | b'w' | b'0'..=b'9')) {
            self.read_char();
        }

        let length = self.position - start;
        if length < 2 || length > 3 {
            panic!(
                "someone submitted a register with the wrong length! {:?}",
                self.position - start
            )
        }

        let word = &self.code[start..self.position];
        if word == "x30" {
            return Token::X30;
        };

        let width_char = word.chars().next();
        let width = match width_char {
            None => panic!("the fuck you mean we don't have a first word?"),
            Some('x') => Width::X64,
            Some('w') => Width::W32,
            Some(char) => panic!("invalid register width: {char}"),
        };

        let register = Register { number: 0, width };
        Token::Register(register)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current_char, Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_comma() {
        let code = "sp, sp,";

        let mut lexer = Lexer::new(code.to_owned());

        let mut token = lexer.next_token();
        assert_eq!(token, Token::Sp);
        token = lexer.next_token();
        assert_eq!(token, Token::Comma);
    }

    #[test]
    fn test_next_token() {
        let code = "
          _main:
            sub sp, sp, #16
            str x30, [sp]
            mov w0, #21
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
            Token::Int(21),
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
        ];

        let mut lexer = Lexer::new(code.to_owned());
        for test in tests {
            let t = lexer.next_token();
            println!("got {:#?} for {:#?}", t, test);
            assert_eq!(t, test);
        }
    }
}
