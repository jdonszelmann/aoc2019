use std::collections::HashMap;

type Instruction = dyn Fn(&mut CPU, usize) -> usize;

pub struct CPU<'c> {
    program: Vec<usize>,
    original_program: Vec<usize>,
    instructions: HashMap<usize, &'c Instruction>,
    stopped: bool,
}

impl<'c> CPU<'c> {
    pub fn new(program: Vec<usize>) -> Self {
        Self {
            original_program: program.to_vec(),
            program,
            instructions: HashMap::new(),
            stopped: false,
        }
    }

    pub fn reset(&mut self) {
        self.program = self.original_program.to_vec();
        self.stopped = false;
    }

    /// Create cpu from string of integers
    pub fn from(program: &str) -> Self {
        Self::new(
            program
                .split_terminator(',')
                .map(|i| i.parse())
                .filter_map(Result::ok)
                .collect(),
        )
    }

    pub fn program_to_string(&self) -> String {
        self.program
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn get_program(self) -> Vec<usize> {
        self.program
    }

    pub fn add_instruction(&mut self, opcode: usize, instruction: &'c Instruction) {
        self.instructions.insert(opcode, instruction);
    }

    pub fn set_program_byte(&mut self, index: usize, value: usize) {
        self.program[index] = value;
    }

    pub fn get_program_byte(&self, index: usize) -> usize {
        self.program[index]
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        while !self.stopped {
            let opcode = self.program[pc];
            let instruction = match self.instructions.get(&opcode) {
                Some(i) => *i,
                None => {
                    pc += 1;
                    continue;
                }
            };

            let length = instruction(self, pc);
            pc += length;
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

pub const MUL: &Instruction = &|cpu, pc| {
    let dst = cpu.program[pc + 3];
    cpu.program[dst] = cpu.program[cpu.program[pc + 1]] * cpu.program[cpu.program[pc + 2]];
    4
};

pub const ADD: &Instruction = &|cpu, pc| {
    let dst = cpu.program[pc + 3];
    cpu.program[dst] = cpu.program[cpu.program[pc + 1]] + cpu.program[cpu.program[pc + 2]];
    4
};

pub const STOP: &Instruction = &|cpu, _pc| {
    cpu.stop();
    1
};

fn execute_1202(program: &str) -> usize {
    let mut cpu = CPU::from(program);
    cpu.add_instruction(1, ADD);
    cpu.add_instruction(2, MUL);
    cpu.add_instruction(99, STOP);

    cpu.set_program_byte(1, 12);
    cpu.set_program_byte(2, 2);

    cpu.run();

    cpu.get_program_byte(0)
}

fn execute_normal(program: &str) -> String {
    let mut cpu = CPU::from(program);
    cpu.add_instruction(1, ADD);
    cpu.add_instruction(2, MUL);
    cpu.add_instruction(99, STOP);

    cpu.run();

    cpu.program_to_string()
}

#[cfg(test)]
mod test {
    use crate::day2::challenge1::{execute_1202, execute_normal};

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = execute_1202(input);
        assert_eq!(result, 3306701);
        println!("challenge 2.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(execute_normal("1,0,0,0,99"), "2,0,0,0,99");
    }

    #[test]
    fn test_main_2() {
        assert_eq!(execute_normal("2,3,0,3,99"), "2,3,0,6,99");
    }

    #[test]
    fn test_main_3() {
        assert_eq!(execute_normal("2,4,4,5,99,0"), "2,4,4,5,99,9801");
    }

    #[test]
    fn test_main_4() {
        assert_eq!(execute_normal("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }
}
