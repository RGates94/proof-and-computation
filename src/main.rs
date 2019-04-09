enum RegisterInstruction {
    INC(usize, usize),
    DEC(usize, usize, usize),
}

struct RegisterMachine {
    instructions: Vec<RegisterInstruction>,
}

struct RegisterComputer {
    program: RegisterMachine,
    current_state: ProgramState,
}

#[derive(Debug)]
struct ProgramState {
    current_instruction: usize,
    registers: Vec<usize>,
    halted: bool,
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
    fn compute(&mut self, machine: &RegisterMachine) {
        while let Some(instruction) = machine.instructions.get(self.current_instruction) {
            self.do_instruction(instruction);
        };
        self.halted = true;
    }
}

fn main() {
    println!("Hello Proof and Computation, we will produce some proofs and computations!");
    let register_machine = RegisterMachine {
        instructions: vec![RegisterInstruction::INC(2, 1),RegisterInstruction::DEC(2, 2, 3),RegisterInstruction::INC(0, 1)]
    };
    let mut basic_program_state = ProgramState {
        current_instruction: 0,
        registers: vec![5, 3],
        halted: false,
    };
    println!("{:?}", basic_program_state);
    basic_program_state.compute(&register_machine);
    println!("{:?}", basic_program_state);
}
