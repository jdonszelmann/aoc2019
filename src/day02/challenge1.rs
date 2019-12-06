use crate::day02::challenge1::ParameterMode::{Immediate, Position};
use std::collections::HashMap;

pub enum ParameterMode {
    Position,
    Immediate,
}

/// returns true if the pc should be autoincremented
pub type Instruction = dyn Fn(&mut CPU, &mut usize, &[isize]) -> bool;

pub struct CPU<'c> {
    pub program: Vec<isize>,
    original_program: Vec<isize>,
    instructions: HashMap<usize, (&'c Instruction, usize)>,
    stopped: bool,

    pub input: Vec<isize>,
    pub inpoffset: isize,

    pub outputbuffer: Vec<isize>,
}

impl<'c> CPU<'c> {
    pub fn new(program: Vec<isize>) -> Self {
        Self {
            original_program: program.to_vec(),
            program,
            instructions: HashMap::new(),
            stopped: false,

            input: vec![],
            inpoffset: 0,

            outputbuffer: vec![],
        }
    }

    pub fn set_input(&mut self, inp: Vec<isize>) {
        self.input = inp;
    }

    pub fn reset(&mut self) {
        self.program = self.original_program.to_vec();
        self.stopped = false;
    }

    pub fn get_output(&self) -> &Vec<isize> {
        &self.outputbuffer
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

    pub fn add_instruction(&mut self, opcode: usize, length: usize, instruction: &'c Instruction) {
        self.instructions.insert(opcode, (instruction, length));
    }

    pub fn set_program_byte(&mut self, index: usize, value: isize) {
        self.program[index] = value;
    }

    pub fn get_program_byte(&self, index: usize) -> isize {
        self.program[index]
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        while !self.stopped {
            let code = self.program[pc];
            let strcode = format!("{:05}", code);

            let opcode: usize = strcode[3..5].parse().expect("should be int");

            let (instruction, length) = match self.instructions.get(&opcode) {
                Some(i) => *i,
                None => {
                    pc += 1;
                    continue;
                }
            };

            let mut params = vec![];
            let numparams = length - 1;
            for i in 0..numparams {
                let mode = match strcode.chars().nth(2 - i) {
                    Some('0') => Position,
                    Some('1') => Immediate,
                    _ => unimplemented!(),
                };

                params.push(match mode {
                    Position => {
                        let val = self.program[pc + i + 1];
                        if val < self.program.len() as isize && val >= 0 {
                            self.program[val as usize]
                        } else {
                            panic!("Index out of bounds");
                        }
                    }
                    Immediate => self.program[pc + i + 1],
                });
            }

            if instruction(self, &mut pc, &params) {
                pc += length;
            }
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

pub const MUL: &Instruction = &|cpu, pc, params| {
    let dst = cpu.program[*pc + 3];
    cpu.program[dst as usize] = params[0] * params[1];
    true
};

pub const ADD: &Instruction = &|cpu, pc, params| {
    let dst = cpu.program[*pc + 3];
    cpu.program[dst as usize] = params[0] + params[1];
    true
};

pub const STOP: &Instruction = &|cpu, _pc, _params| {
    cpu.stop();
    true
};

fn execute_1202(program: &str) -> isize {
    let mut cpu = CPU::from(program);
    cpu.add_instruction(1, 4, ADD);
    cpu.add_instruction(2, 4, MUL);
    cpu.add_instruction(99, 1, STOP);

    cpu.set_program_byte(1, 12);
    cpu.set_program_byte(2, 2);

    cpu.run();

    cpu.get_program_byte(0)
}

fn execute_normal(program: &str) -> String {
    let mut cpu = CPU::from(program);
    cpu.add_instruction(1, 4, ADD);
    cpu.add_instruction(2, 4, MUL);
    cpu.add_instruction(99, 1, STOP);

    cpu.run();

    cpu.program_to_string()
}

#[cfg(test)]
mod test {
    use crate::day02::challenge1::{execute_1202, execute_normal, CPU};

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

    #[test]
    fn test_main_5() {
        assert_eq!(CPU::from("1").program, vec![1]);
    }

    #[test]
    fn test_main_6() {
        assert_eq!(CPU::from("test,test").program, vec![]);
    }

    #[test]
    fn test_main_7() {
        assert_eq!(execute_normal("8,9,99"), "8,9,99");
    }

    #[test]
    #[should_panic]
    fn test_main_8() {
        execute_normal("55501");
    }

    #[test]
    #[should_panic]
    fn test_main_9() {
        execute_normal("1,10,1,1,99");
    }
}
