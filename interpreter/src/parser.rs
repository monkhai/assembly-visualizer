use std::collections::HashMap;

use crate::{
    ast::{
        Opcode, Register,
        Token::{self},
    },
    instruction::{
        Instruction, MemoryAddress,
        RegisterOrImmediate::{self, Immediate},
        RegisterOrSp,
    },
    lexer::Lexer,
    program::Program,
};

pub struct Parser {
    pub lexer: Lexer,
    program: Program,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let program = Program {
            instructions: vec![],
            label_map: HashMap::new(),
        };
        Self { lexer, program }
    }

    pub fn parse(mut self) -> Result<Program, &'static str> {
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Opcode(opcode) => self.read_instruction(opcode)?,
                Token::LabelDef(def) => self.handle_label_def(def),
                _ => return Err("unexpected token at the top level"),
            }
        }

        Ok(self.program)
    }

    fn read_instruction(&mut self, opcode: Opcode) -> Result<(), &'static str> {
        match opcode {
            Opcode::Mov => {
                let destination = self.read_register()?;
                self.read_comma()?;
                let source = self.read_register_or_immediate()?;

                self.program.instructions.push(Instruction::Mov {
                    destination,
                    source,
                });
            }

            Opcode::Add => {
                let destination = self.read_register_or_sp()?;
                self.read_comma()?;
                let first_source = self.read_register_or_sp()?;
                self.read_comma()?;
                let second_source = self.read_register_or_immediate()?;

                self.program.instructions.push(Instruction::Add {
                    destination,
                    first_source,
                    second_source,
                });
            }

            Opcode::Sub => {
                let destination = self.read_register_or_sp()?;
                self.read_comma()?;
                let first_source = self.read_register_or_sp()?;
                self.read_comma()?;
                let second_source = self.read_register_or_immediate()?;

                self.program.instructions.push(Instruction::Sub {
                    destination,
                    first_source,
                    second_source,
                });
            }

            Opcode::Str => {
                let register = self.read_register()?;
                self.read_comma()?;
                let address = self.read_address()?;
                self.program
                    .instructions
                    .push(Instruction::Str { register, address });
            }

            Opcode::Ldr => {
                let destination = self.read_register()?;
                self.read_comma()?;
                let address = self.read_address()?;
                self.program.instructions.push(Instruction::Ldr {
                    destination,
                    address,
                });
            }

            Opcode::Ret => self.program.instructions.push(Instruction::Ret),
        }
        Ok(())
    }

    fn read_register(&mut self) -> Result<Register, &'static str> {
        let Token::Register(destination) = self.lexer.next_token() else {
            return Err("expected a register");
        };

        Ok(destination)
    }

    fn read_register_or_sp(&mut self) -> Result<RegisterOrSp, &'static str> {
        let value = match self.lexer.next_token() {
            Token::Register(register) => RegisterOrSp::Register(register),
            Token::Sp => RegisterOrSp::Sp,
            _ => return Err("expected a register or Sp"),
        };
        Ok(value)
    }

    fn read_register_or_immediate(&mut self) -> Result<RegisterOrImmediate, &'static str> {
        let source = match self.lexer.next_token() {
            Token::Int(value) => Immediate(value),
            Token::Register(register) => RegisterOrImmediate::Register(register),
            _ => return Err("expected a register or immediate"),
        };
        Ok(source)
    }

    fn read_address(&mut self) -> Result<MemoryAddress, &'static str> {
        let Token::LBracket = self.lexer.next_token() else {
            return Err("expected '['");
        };
        let base = self.read_register_or_sp()?;
        let offset = self.read_address_offset()?;
        Ok(MemoryAddress { base, offset })
    }

    fn read_address_offset(&mut self) -> Result<Option<i64>, &'static str> {
        match self.lexer.next_token() {
            Token::Comma => {
                let Token::Int(offset) = self.lexer.next_token() else {
                    return Err("expected an offset");
                };
                let Token::RBracket = self.lexer.next_token() else {
                    return Err("expected a ']'");
                };
                Ok(Some(offset))
            }
            Token::RBracket => Ok(None),
            _ => Err("an unexpected token after "),
        }
    }

    fn read_comma(&mut self) -> Result<(), &'static str> {
        let Token::Comma = self.lexer.next_token() else {
            return Err("expected a comma");
        };
        Ok(())
    }

    fn handle_label_def(&mut self, def: String) {
        self.program
            .label_map
            .insert(def, self.program.instructions.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, RegisterOrImmediate::Immediate};

    use super::*;

    #[test]
    fn get_correct_instructions() {
        let code = "
          _main:
            sub sp, sp, #16
            str x30, [sp]
            mov w0, #42
            ldr x30, [sp]
            add sp, sp, #16
            ret
          ";
        let expected_instructions = vec![
            Instruction::Sub {
                destination: RegisterOrSp::Sp,
                first_source: RegisterOrSp::Sp,
                second_source: Immediate(16),
            },
            Instruction::Str {
                register: Register {
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

        let expected_map = HashMap::from([("_main".to_owned(), 0)]);

        let lexer = Lexer::new(code.to_owned());
        let parser = Parser::new(lexer);
        let program = parser.parse().expect("valid program");
        assert_eq!(program.instructions.len(), expected_instructions.len());
        for (instruction, test) in program.instructions.iter().zip(expected_instructions) {
            assert_eq!(instruction, &test);
        }

        assert_eq!(program.label_map, expected_map);
    }
}
