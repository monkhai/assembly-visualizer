pub struct Immediate(i64);

pub struct MemoryAddress {
    pub base: Register,
    pub offset: i64,
}

pub enum RegisterOrImmediate {
    Register(Register),
    Immediate(Immediate),
}

pub enum Instruction {
    Mov {
        destination: Register,
        source: RegisterOrImmediate,
    },
    Add {
        destination: Register,
        first_source: Register,
        second_source: RegisterOrImmediate,
    },
    Sub {
        destination: Register,
        first_source: Register,
        second_source: RegisterOrImmediate,
    },
    Str {
        value: Register,
        address: MemoryAddress,
    },
    Ldr {
        destination: Register,
        address: MemoryAddress,
    },
}

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

    Opcode(Opcode),
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
        "mov" => Some(Token::Opcode(Opcode::Mov)),
        "add" => Some(Token::Opcode(Opcode::Add)),
        "sub" => Some(Token::Opcode(Opcode::Sub)),
        "str" => Some(Token::Opcode(Opcode::Str)),
        "ldr" => Some(Token::Opcode(Opcode::Ldr)),

        _ => None,
    }
}
