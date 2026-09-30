#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mov,
    Add,
    Sub,
    Str,
    Ldr,
    Ret,
}

#[derive(Debug, PartialEq)]
pub enum Width {
    W32,
    X64,
}

#[derive(Debug, PartialEq)]
pub struct Register {
    pub number: u8,
    pub width: Width,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Label(String),
    LabelDef(String),

    Opcode(Opcode),
    Sp,

    Comma,
    LBracket,
    RBracket,
    Eof,

    Int(i64),
    Register(Register),
}

pub fn keyword_token(word: &str) -> Option<Token> {
    match word {
        "sp" => Some(Token::Sp),

        // instructions
        "mov" => Some(Token::Opcode(Opcode::Mov)),
        "add" => Some(Token::Opcode(Opcode::Add)),
        "sub" => Some(Token::Opcode(Opcode::Sub)),
        "str" => Some(Token::Opcode(Opcode::Str)),
        "ldr" => Some(Token::Opcode(Opcode::Ldr)),
        "ret" => Some(Token::Opcode(Opcode::Ret)),

        _ => None,
    }
}
