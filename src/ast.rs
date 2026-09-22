#[derive(Debug, PartialEq)]
pub enum Instruction {
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

    Instruction(Instruction),
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

        // instructions
        "mov" => Some(Token::Instruction(Instruction::Mov)),
        "add" => Some(Token::Instruction(Instruction::Add)),
        "sub" => Some(Token::Instruction(Instruction::Sub)),
        "str" => Some(Token::Instruction(Instruction::Str)),
        "ldr" => Some(Token::Instruction(Instruction::Ldr)),

        _ => None,
    }
}
