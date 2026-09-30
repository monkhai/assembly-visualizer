use std::collections::HashMap;

use crate::instruction::Instruction;

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub label_map: HashMap<String, usize>,
}
