use std::{collections::HashMap, iter::repeat_n};

const INITIAL_PC: u64 = 0x1000;
const INITIAL_SP: u64 = 0x8000;
const INITIAL_X30: u64 = 0x9000;

use crate::{
    ast::{Register, Width},
    instruction::{Instruction, MemoryAddress, RegisterOrImmediate, RegisterOrSp},
    program::Program,
};

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    pub sp: u64,
    pub x30: u64,
    pub pc: u64,
    pub registers: HashMap<u8, u64>,
    pub stack: Vec<u8>,
}

impl State {
    fn new() -> Self {
        Self {
            sp: INITIAL_SP,
            x30: INITIAL_X30,
            pc: INITIAL_PC,
            registers: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn instruction_index(&self) -> usize {
        // we are dividing by 4 here since
        ((self.pc - INITIAL_PC) / 4) as usize
    }
}

pub struct Evaluator {
    program: Program,
    state: State,
    history: Vec<State>,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            state: State::new(),
            history: vec![],
        }
    }

    pub fn current(&self) -> State {
        self.state.clone()
    }

    pub fn next(&mut self) -> Result<State, String> {
        if self.state.instruction_index() >= self.program.instructions.len() {
            return Err("no more instructions to execute".into());
        }

        self.capture_state();

        let instruction = &self.program.instructions[self.state.instruction_index()];

        match instruction {
            Instruction::Mov {
                destination,
                source,
            } => {
                let value = match source {
                    RegisterOrImmediate::Immediate(v) => self.get_immediate_u64(v)?,
                    RegisterOrImmediate::Register(register) => {
                        if register.width != destination.width {
                            return Err(format!(
                                "register widths do not match. source: {:?}, dest: {:?}",
                                register.width, destination.width,
                            ));
                        }
                        self.get_register_value(register)
                    }
                };

                self.set_register_value(destination.number, value);
            }

            Instruction::Add {
                destination,
                first_source,
                second_source,
            } => match destination {
                RegisterOrSp::Sp => {
                    let first = match first_source {
                        RegisterOrSp::Sp => self.state.sp,
                        RegisterOrSp::Register(register) => self.get_register_value(register),
                    };

                    let second = match second_source {
                        RegisterOrImmediate::Immediate(v) => self.get_immediate_u64(v)?,

                        RegisterOrImmediate::Register(register) => {
                            self.get_register_value(register)
                        }
                    };

                    self.state.sp = first + second;
                }

                RegisterOrSp::Register(_) => panic!("we can't process registers for add"),
            },

            Instruction::Sub {
                destination,
                first_source,
                second_source,
            } => match destination {
                RegisterOrSp::Sp => {
                    let first = match first_source {
                        RegisterOrSp::Sp => self.state.sp,
                        RegisterOrSp::Register(register) => self.get_register_value(register),
                    };

                    let second = match second_source {
                        RegisterOrImmediate::Immediate(v) => self.get_immediate_u64(v)?,

                        RegisterOrImmediate::Register(register) => {
                            self.get_register_value(register)
                        }
                    };

                    self.state.sp = first - second;
                    let sp_offset = INITIAL_SP.saturating_sub(self.state.sp) as usize;
                    self.state.stack.splice(0..0, repeat_n(0u8, sp_offset));
                }

                RegisterOrSp::Register(_) => panic!("we can't process registers for sub"),
            },

            Instruction::Str { register, address } => {
                let register_value = self.get_register_value(register);
                let address_index = self.get_address_index(address);
                let bytes = register_value.to_le_bytes();
                self.state.stack[address_index..address_index + 8].copy_from_slice(&bytes);
            }

            Instruction::Ldr {
                destination,
                address,
            } => {
                let address_index = self.get_address_index(address);
                let value = u64::from_le_bytes(
                    self.state.stack[address_index..address_index + 8]
                        .try_into()
                        .unwrap(),
                );

                self.set_register_value(destination.number, value);
            }

            Instruction::Ret => self.state.sp = self.state.x30,
        }

        self.advance_pc();

        Ok(self.state.clone())
    }

    fn capture_state(&mut self) {
        self.history.push(self.current());
    }

    fn advance_pc(&mut self) {
        self.state.pc = self.state.pc + 4;
    }

    fn get_register_value(&self, register: &Register) -> u64 {
        if register.number == 30 {
            return self.state.x30;
        }

        let full = self
            .state
            .registers
            .get(&register.number)
            .copied()
            .unwrap_or(0);

        let val = match register.width {
            Width::X64 => full,
            Width::W32 => (full as u32) as u64,
        };

        return val;
    }

    fn set_register_value(&mut self, register: u8, value: u64) {
        if register == 30 {
            self.state.x30 = value;
        } else {
            self.state.registers.insert(register, value);
        }
    }

    fn get_address_index(&self, address: &MemoryAddress) -> usize {
        let base = match &address.base {
            RegisterOrSp::Sp => self.state.sp,
            RegisterOrSp::Register(register) => self.get_register_value(&register),
        };
        let offset = address.offset.unwrap_or(0);
        let effective_address = base.checked_add_signed(offset).expect(
            "address calculation is out of range. offset is negative and larger than whole stack",
        );

        (self.state.sp - effective_address) as usize
    }

    fn get_immediate_u64(&self, value: &i64) -> Result<u64, String> {
        u64::try_from(*value).map_err(|_| "negative stack subtraction amount".into())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        ast::{Register, Width},
        evaluator::{Evaluator, INITIAL_PC, INITIAL_SP, INITIAL_X30, State},
        instruction::{Instruction, RegisterOrImmediate},
        program::Program,
    };

    struct Test {
        program: Program,
        registers: HashMap<u8, u64>,
        expected: State,
    }

    fn stack(size: usize, values: &[(usize, u64)]) -> Vec<u8> {
        let mut bytes = vec![0; size];
        for &(offset, value) in values {
            bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    fn registers(values: &[(u8, u64)]) -> HashMap<u8, u64> {
        let mut map: HashMap<u8, u64> = HashMap::new();
        for (k, v) in values {
            map.insert(*k, *v);
        }
        map
    }

    mod mov {
        use super::*;

        #[test]
        fn handle_immediate_as_source() {
            let test = Test {
                program: Program {
                    instructions: vec![Instruction::Mov {
                        destination: Register {
                            number: 0,
                            width: Width::W32,
                        },
                        source: RegisterOrImmediate::Immediate(42),
                    }],
                    label_map: HashMap::new(),
                },
                registers: registers(&[]),
                expected: State {
                    pc: INITIAL_PC + 4,
                    sp: INITIAL_SP,
                    x30: INITIAL_X30,
                    registers: registers(&[(0, 42)]),
                    stack: stack(0, &[]),
                },
            };

            let mut e = Evaluator::new(test.program);
            e.state.registers = test.registers;
            let state = e.next().expect("evaluation to work");
            assert_eq!(state, test.expected);
        }

        #[test]
        fn handle_register_as_source() {
            let test = Test {
                program: Program {
                    instructions: vec![Instruction::Mov {
                        destination: Register {
                            number: 0,
                            width: Width::W32,
                        },
                        source: RegisterOrImmediate::Register(Register {
                            number: 1,
                            width: Width::W32,
                        }),
                    }],
                    label_map: HashMap::new(),
                },
                registers: registers(&[(1, 42)]),
                expected: State {
                    pc: INITIAL_PC + 4,
                    stack: stack(0, &[]),
                    sp: INITIAL_SP,
                    x30: INITIAL_X30,
                    registers: registers(&[(0, 42), (1, 42)]),
                },
            };

            let mut e = Evaluator::new(test.program);
            e.state.registers = test.registers;
            let state = e.next().expect("evaluation to work");
            assert_eq!(state, test.expected);
        }

        #[test]
        fn handle_non_matching_widths() {
            let program = Program {
                instructions: vec![Instruction::Mov {
                    destination: Register {
                        number: 0,
                        width: Width::W32,
                    },
                    source: RegisterOrImmediate::Register(Register {
                        number: 1,
                        width: Width::X64,
                    }),
                }],
                label_map: HashMap::new(),
            };

            let mut e = Evaluator::new(program);
            let state = e.next();
            assert!(state.is_err());
        }
    }
}
