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
    variable_states: HashMap<String, usize>,
}

pub enum AssignType {
    Zero,
    Variable(String),
    VariableIncremented(String),
}

impl WhileState {
    pub fn new() -> Self {
        WhileState {
            variable_states: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: String, value: usize) {
        self.variable_states.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<&usize> {
        self.variable_states.get(key)
    }
    pub fn do_instruction(&mut self, instruction: &WhileInstruction) {
        match instruction {
            WhileInstruction::Assign(var, assign) => match assign {
                AssignType::Zero => {
                    self.insert(var.to_string(), 0);
                }
                AssignType::Variable(other) => {
                    self.insert(
                        var.to_string(),
                        *self
                            .get(other)
                            .expect("Did not find right hand variable, write error handling later"),
                    );
                }
                AssignType::VariableIncremented(other) => {
                    self.insert(
                        var.to_string(),
                        *self
                            .get(other)
                            .expect("Did not find right hand variable, write error handling later")
                            + 1,
                    );
                }
            },
            WhileInstruction::If(first_var, second_var, if_block, else_block) => {
                if self.get(first_var) < self.get(second_var) {
                    if_block.run(self)
                } else {
                    else_block.run(self)
                }
            }
            WhileInstruction::For(var, block) => {
                for _i in 0..*self
                    .get(var)
                    .expect("The indexing variable was not found, write error handling later")
                {
                    block.run(self)
                }
            }
            WhileInstruction::While(first_var, second_var, block) => {
                while self.get(first_var) < self.get(second_var) {
                    block.run(self)
                }
            }
        }
    }
}

impl WhileProgram {
    pub fn from_vec(instructions: Vec<WhileInstruction>) -> Self {
        WhileProgram { instructions }
    }
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
        let mut state = WhileState::new();
        state.do_instruction(&instruction);
        state.do_instruction(&second_instruction);
        assert_eq!(state.get("x"), Some(&0));
        assert_eq!(state.get("refrigerator"), Some(&0));
    }

    #[test]
    fn assign_variable() {
        let instruction = WhileInstruction::Assign(String::from("x"), AssignType::Zero);
        let second_instruction = WhileInstruction::Assign(
            String::from("refrigerator"),
            AssignType::Variable(String::from("x")),
        );
        let mut state = WhileState::new();
        state.do_instruction(&instruction);
        state.do_instruction(&second_instruction);
        assert_eq!(state.get("x"), Some(&0));
        assert_eq!(state.get("refrigerator"), Some(&0));
    }

    #[test]
    fn assign_variable_incremented() {
        let instruction = WhileInstruction::Assign(String::from("x"), AssignType::Zero);
        let second_instruction = WhileInstruction::Assign(
            String::from("refrigerator"),
            AssignType::VariableIncremented(String::from("x")),
        );
        let mut state = WhileState::new();
        state.do_instruction(&instruction);
        state.do_instruction(&second_instruction);
        assert_eq!(state.get("x"), Some(&0));
        assert_eq!(state.get("refrigerator"), Some(&1));
    }

    #[test]
    fn run_assignment_program() {
        let program = WhileProgram::from_vec(vec![
            WhileInstruction::Assign(String::from("x"), AssignType::Zero),
            WhileInstruction::Assign(
                String::from("refrigerator"),
                AssignType::VariableIncremented(String::from("x")),
            ),
        ]);
        let mut state = WhileState::new();
        program.run(&mut state);
        assert_eq!(state.get("x"), Some(&0));
        assert_eq!(state.get("refrigerator"), Some(&1));
    }

    #[test]
    fn run_if_program() {
        let program = WhileProgram::from_vec(vec![
            WhileInstruction::Assign(String::from("x"), AssignType::Zero),
            WhileInstruction::Assign(
                String::from("refrigerator"),
                AssignType::VariableIncremented(String::from("x")),
            ),
            WhileInstruction::If(
                String::from("x"),
                String::from("refrigerator"),
                Box::new(WhileProgram::from_vec(vec![WhileInstruction::Assign(
                    String::from("x"),
                    AssignType::VariableIncremented(String::from("x")),
                )])),
                Box::new(WhileProgram::from_vec(vec![WhileInstruction::Assign(
                    String::from("y"),
                    AssignType::VariableIncremented(String::from("x")),
                )])),
            ),
        ]);
        let mut state = WhileState::new();
        program.run(&mut state);
        assert_eq!(state.get("x"), Some(&1));
        assert_eq!(state.get("refrigerator"), Some(&1));
        assert_eq!(state.get("y"), None);
    }

    #[test]
    fn run_for_program() {
        let program = WhileProgram::from_vec(vec![WhileInstruction::For(
            String::from("x"),
            Box::new(WhileProgram::from_vec(vec![WhileInstruction::For(
                String::from("x"),
                Box::new(WhileProgram::from_vec(vec![
                    WhileInstruction::Assign(
                        String::from("x"),
                        AssignType::VariableIncremented(String::from("x")),
                    ),
                    WhileInstruction::Assign(
                        String::from("y"),
                        AssignType::VariableIncremented(String::from("y")),
                    ),
                ])),
            )])),
        )]);
        let mut state = WhileState::new();
        state.insert(String::from("x"), 3);
        state.insert(String::from("y"), 0);
        program.run(&mut state);
        assert_eq!(state.get("x"), Some(&24));
        assert_eq!(state.get("y"), Some(&21));
    }

    #[test]
    fn run_while_program() {
        let program = WhileProgram::from_vec(vec![WhileInstruction::While(
            String::from("y"),
            String::from("x"),
            Box::new(WhileProgram::from_vec(vec![
                WhileInstruction::Assign(
                    String::from("y"),
                    AssignType::VariableIncremented(String::from("y")),
                ),
                WhileInstruction::For(
                    String::from("x"),
                    Box::new(WhileProgram::from_vec(vec![
                        WhileInstruction::Assign(
                            String::from("x"),
                            AssignType::VariableIncremented(String::from("x")),
                        ),
                        WhileInstruction::Assign(
                            String::from("y"),
                            AssignType::VariableIncremented(String::from("y")),
                        ),
                    ])),
                ),
            ])),
        )]);
        let mut state = WhileState::new();
        state.insert(String::from("x"), 4);
        state.insert(String::from("y"), 0);
        program.run(&mut state);
        assert_eq!(state.get("x"), Some(&64));
        assert_eq!(state.get("y"), Some(&64));
    }
}
