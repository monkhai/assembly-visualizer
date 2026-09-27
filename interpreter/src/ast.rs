#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mov,
    Add,
    Sub,
    Str,
    Ldr,
}

#[derive(Debug, PartialEq)]
pub enum Width {
    W32,
    X64,
}

#[derive(Debug, PartialEq)]
pub struct Register {
    pub number: i8,
    pub width: Width,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Label(String),
    LabelDef(String),

    Instruction(Opcode),
    Sp,
    X30,

    Comma,
    LBracket,
    RBracket,
    Eof,

    Ret,

    Int(i64),
    Register(Register),
}

pub fn keyword_token(word: &str) -> Option<Token> {
    match word {
        "ret" => Some(Token::Ret),
        "sp" => Some(Token::Sp),
        "x30" => Some(Token::X30),

        // instructions
        "mov" => Some(Token::Instruction(Opcode::Mov)),
        "add" => Some(Token::Instruction(Opcode::Add)),
        "sub" => Some(Token::Instruction(Opcode::Sub)),
        "str" => Some(Token::Instruction(Opcode::Str)),
        "ldr" => Some(Token::Instruction(Opcode::Ldr)),

        _ => None,
    }
}
