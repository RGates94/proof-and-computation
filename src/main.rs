mod register_machine;
use register_machine::{ProgramState, RegisterInstruction, RegisterMachine};

fn main() {
    println!("Hello Proof and Computation, we will produce some proofs and computations!");
    let register_machine = RegisterMachine {
        instructions: vec![
            RegisterInstruction::INC(2, 1),
            RegisterInstruction::DEC(2, 2, 3),
            RegisterInstruction::INC(0, 1),
        ],
    };
    let mut basic_program_state = ProgramState {
        current_instruction: 0,
        registers: vec![5, 3],
        halted: false,
    };
    println!("{:?}", basic_program_state);
    println!("{}", basic_program_state.compute(&register_machine));
    println!("{:?}", basic_program_state);
}
