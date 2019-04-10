use std::collections::HashMap;

pub enum WhileInstruction {
    Assign(String, AssignType),
    If(String, String, Box<WhileProgram>, Box<WhileProgram>),
    For(String, Box<WhileProgram>),
    While(String, String, Box<WhileProgram>),
}

pub struct WhileProgram {
    pub instructions: Vec<WhileInstruction>,
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
            WhileInstruction::Assign(var, assign) => {
                match assign {
                    AssignType::Zero => {
                        self.variable_states.insert(var.to_string(), 0);
                    }
                    AssignType::Variable(other) => {
                        self.variable_states.insert(
                            var.to_string(),
                            *self.variable_states.get(other).expect(
                                "Did not find right hand variable, write error handling later",
                            ),
                        );
                    }
                    AssignType::VariableIncremented(other) => {
                        self.variable_states.insert(
                            var.to_string(),
                            *self.variable_states.get(other).expect(
                                "Did not find right hand variable, write error handling later",
                            ) + 1,
                        );
                    }
                }
            }
            WhileInstruction::If(first_var, second_var, if_block, else_block) => {
                if self.variable_states.get(first_var) < self.variable_states.get(second_var) {
                    if_block.run(self)
                } else {
                    else_block.run(self)
                }
            }
            WhileInstruction::For(var, block) => {
                for _i in 0..*self
                    .variable_states
                    .get(var)
                    .expect("The indexing variable was not found, write error handling later")
                {
                    block.run(self)
                }
            }
            _ => {}
        }
    }
}

impl WhileProgram {
    pub fn run(&self, state: &mut WhileState) {
        for instruction in self.instructions.iter() {
            state.do_instruction(instruction)
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

    #[test]
    fn assign_variable() {
        let instruction = WhileInstruction::Assign(String::from("x"), AssignType::Zero);
        let second_instruction = WhileInstruction::Assign(
            String::from("refrigerator"),
            AssignType::Variable(String::from("x")),
        );
        let mut state = WhileState {
            variable_states: HashMap::new(),
        };
        state.do_instruction(&instruction);
        state.do_instruction(&second_instruction);
        assert_eq!(state.variable_states.get("x"), Some(&0));
        assert_eq!(state.variable_states.get("refrigerator"), Some(&0));
    }

    #[test]
    fn assign_variable_incremented() {
        let instruction = WhileInstruction::Assign(String::from("x"), AssignType::Zero);
        let second_instruction = WhileInstruction::Assign(
            String::from("refrigerator"),
            AssignType::VariableIncremented(String::from("x")),
        );
        let mut state = WhileState {
            variable_states: HashMap::new(),
        };
        state.do_instruction(&instruction);
        state.do_instruction(&second_instruction);
        assert_eq!(state.variable_states.get("x"), Some(&0));
        assert_eq!(state.variable_states.get("refrigerator"), Some(&1));
    }

    #[test]
    fn run_assignment_program() {
        let program = WhileProgram {
            instructions: vec![
                WhileInstruction::Assign(String::from("x"), AssignType::Zero),
                WhileInstruction::Assign(
                    String::from("refrigerator"),
                    AssignType::VariableIncremented(String::from("x")),
                ),
            ],
        };
        let mut state = WhileState {
            variable_states: HashMap::new(),
        };
        program.run(&mut state);
        assert_eq!(state.variable_states.get("x"), Some(&0));
        assert_eq!(state.variable_states.get("refrigerator"), Some(&1));
    }

    #[test]
    fn run_if_program() {
        let program = WhileProgram {
            instructions: vec![
                WhileInstruction::Assign(String::from("x"), AssignType::Zero),
                WhileInstruction::Assign(
                    String::from("refrigerator"),
                    AssignType::VariableIncremented(String::from("x")),
                ),
                WhileInstruction::If(
                    String::from("x"),
                    String::from("refrigerator"),
                    Box::new(WhileProgram {
                        instructions: vec![WhileInstruction::Assign(
                            String::from("x"),
                            AssignType::VariableIncremented(String::from("x")),
                        )],
                    }),
                    Box::new(WhileProgram {
                        instructions: vec![WhileInstruction::Assign(
                            String::from("y"),
                            AssignType::VariableIncremented(String::from("x")),
                        )],
                    }),
                ),
            ],
        };
        let mut state = WhileState {
            variable_states: HashMap::new(),
        };
        program.run(&mut state);
        assert_eq!(state.variable_states.get("x"), Some(&1));
        assert_eq!(state.variable_states.get("refrigerator"), Some(&1));
        assert_eq!(state.variable_states.get("y"), None);
    }

    #[test]
    fn run_for_program() {
        let program = WhileProgram {
            instructions: vec![
                WhileInstruction::Assign(String::from("x"), AssignType::Zero),
                WhileInstruction::Assign(
                    String::from("x"),
                    AssignType::VariableIncremented(String::from("x")),
                ),
                WhileInstruction::Assign(
                    String::from("x"),
                    AssignType::VariableIncremented(String::from("x")),
                ),
                WhileInstruction::Assign(
                    String::from("x"),
                    AssignType::VariableIncremented(String::from("x")),
                ),
                WhileInstruction::Assign(
                    String::from("y"),
                    AssignType::Zero,
                ),
                WhileInstruction::For(
                    String::from("x"),
                    Box::new(WhileProgram {
                        instructions: vec![WhileInstruction::For(
                            String::from("x"),
                            Box::new(WhileProgram {
                                instructions: vec![WhileInstruction::Assign(
                                    String::from("x"),
                                    AssignType::VariableIncremented(String::from("x")),
                                ),WhileInstruction::Assign(
                                    String::from("y"),
                                    AssignType::VariableIncremented(String::from("y")),
                                )],
                            }),
                        )],
                    }),
                ),
            ],
        };
        let mut state = WhileState {
            variable_states: HashMap::new(),
        };
        program.run(&mut state);
        assert_eq!(state.variable_states.get("x"), Some(&24));
        assert_eq!(state.variable_states.get("y"), Some(&21));
    }
}
