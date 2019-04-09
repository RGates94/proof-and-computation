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
