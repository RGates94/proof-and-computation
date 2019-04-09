use std::collections::HashMap;

pub enum WhileInstruction {
    Assign(String, AssignType),
    If(String, String, Box<WhileProgram>, Box<WhileProgram>),
    For(String, Box<WhileProgram>),
    While(String, String, Box<WhileProgram>),
}

pub struct WhileProgram {
    instructions: Vec<WhileInstruction>,
}

pub struct WhileState {
    pub variable_states: HashMap<String, usize>,
}

pub enum AssignType {
    Zero,
    Variable(String),
    VariableIncremented(String),
}

impl WhileState {
    pub fn do_instruction(&mut self, instruction: &WhileInstruction) {
        match instruction {
            WhileInstruction::Assign(var, assign) => match assign {
                AssignType::Zero => {
                    self.variable_states.insert(var.to_string(), 0);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_zero() {
        let instruction = WhileInstruction::Assign(String::from("x"), AssignType::Zero);
        let second_instruction =
            WhileInstruction::Assign(String::from("refrigerator"), AssignType::Zero);
        let mut state = WhileState {
            variable_states: HashMap::new(),
        };
        state.do_instruction(&instruction);
        state.do_instruction(&second_instruction);
        assert_eq!(state.variable_states.get("x"), Some(&0));
        assert_eq!(state.variable_states.get("refrigerator"), Some(&0));
    }
}
