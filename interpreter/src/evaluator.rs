use crate::ast::Program;

pub struct Evaluator {
    program: Program,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        Instruction, MemoryAddress, Register, RegisterOrImmediate::Immediate, RegisterOrSp,
    };

    #[test]
    fn evaluate_the_thing() {
        let test = vec![
            Instruction::Sub {
                destination: RegisterOrSp::Sp,
                first_source: RegisterOrSp::Sp,
                second_source: Immediate(16),
            },
            Instruction::Str {
                value: Register {
                    number: 30,
                    width: crate::ast::Width::X64,
                },
                address: MemoryAddress {
                    base: RegisterOrSp::Sp,
                    offset: None,
                },
            },
            Instruction::Mov {
                destination: Register {
                    number: 0,
                    width: crate::ast::Width::W32,
                },
                source: Immediate(42),
            },
            Instruction::Ldr {
                destination: Register {
                    number: 30,
                    width: crate::ast::Width::X64,
                },
                address: MemoryAddress {
                    base: RegisterOrSp::Sp,
                    offset: None,
                },
            },
            Instruction::Add {
                destination: RegisterOrSp::Sp,
                first_source: RegisterOrSp::Sp,
                second_source: Immediate(16),
            },
            Instruction::Ret,
        ];
    }
}
