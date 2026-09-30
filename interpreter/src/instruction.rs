use crate::ast::Register;

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
        register: Register,
        address: MemoryAddress,
    },
    Ldr {
        destination: Register,
        address: MemoryAddress,
    },
    Ret,
}
