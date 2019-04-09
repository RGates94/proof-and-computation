use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegisterInstruction {
    INC(usize, usize),
    DEC(usize, usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegisterMachine {
    pub instructions: Vec<RegisterInstruction>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramState {
    pub current_instruction: usize,
    pub registers: Vec<usize>,
    pub halted: bool,
}

impl ProgramState {
    fn do_instruction(&mut self, instruction: &RegisterInstruction) {
        match instruction {
            RegisterInstruction::INC(register, next) => {
                while let None = self.registers.get_mut(*register) {
                    self.registers.push(0);
                }
                self.registers[*register] += 1;
                self.current_instruction = *next;
            }
            RegisterInstruction::DEC(register, next, next_alt) => {
                while let None = self.registers.get_mut(*register) {
                    self.registers.push(0);
                }
                if self.registers[*register] > 0 {
                    self.registers[*register] -= 1;
                    self.current_instruction = *next;
                } else {
                    self.current_instruction = *next_alt;
                }
            }
        };
    }
    pub fn compute(&mut self, machine: &RegisterMachine) -> usize {
        while let Some(instruction) = machine.instructions.get(self.current_instruction) {
            self.do_instruction(instruction);
        }
        self.halted = true;
        *self.registers.get(0).unwrap_or(&0)
    }
}
