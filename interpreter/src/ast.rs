pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub struct MemoryAddress {
    pub base: RegisterOrSp,
    pub offset: Option<i64>,
}

#[derive(Debug, PartialEq)]
pub enum RegisterOrImmediate {
    Register(Register),
    Immediate(i64),
}

#[derive(Debug, PartialEq)]
pub enum RegisterOrSp {
    Register(Register),
    Sp,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov {
        destination: Register,
        source: RegisterOrImmediate,
    },
    Add {
        destination: RegisterOrSp,
        first_source: RegisterOrSp,
        second_source: RegisterOrImmediate,
    },
    Sub {
        destination: RegisterOrSp,
        first_source: RegisterOrSp,
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
    Ret,
}

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
    pub number: i8,
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
