use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegisterInstruction {
    INC(usize, usize),
    DEC(usize, usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegisterMachine {
    instructions: Vec<RegisterInstruction>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramState {
    current_instruction: usize,
    registers: Vec<usize>,
    halted: bool,
}

impl RegisterMachine {
    pub fn from_vec(instructions: Vec<RegisterInstruction>) -> Self {
        RegisterMachine { instructions }
    }
}

impl ProgramState {
    pub fn from_vec(registers: Vec<usize>) -> Self {
        ProgramState {
            current_instruction: 0,
            registers,
            halted: false,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculation() {
        let register_machine = RegisterMachine::from_vec(vec![
            RegisterInstruction::INC(2, 1),
            RegisterInstruction::DEC(2, 2, 3),
            RegisterInstruction::INC(0, 1),
        ]);
        let mut input_state = ProgramState::from_vec(vec![5, 3]);
        assert_eq!(input_state.compute(&register_machine), 6);
    }
}
