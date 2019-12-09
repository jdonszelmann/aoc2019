use crate::day02::challenge1::{Instruction, ADD, CPU, MUL, STOP};
use crate::day05::challenge1::{INP, OUT};
use crate::day05::challenge2::{BF, BT, EQ, LT};

pub const SETREL: &Instruction = &|cpu, params, _directparams| {
    cpu.relative_base += params[0];
    true
};

pub fn get_cpu(program: &str, input: isize) -> CPU {
    let mut cpu = CPU::from(program);
    cpu.add_to_input(input);

    cpu.add_instruction(1, 4, ADD);
    cpu.add_instruction(2, 4, MUL);
    cpu.add_instruction(99, 1, STOP);
    cpu.add_instruction(3, 2, INP);
    cpu.add_instruction(4, 2, OUT);
    cpu.add_instruction(5, 3, BT);
    cpu.add_instruction(6, 3, BF);
    cpu.add_instruction(7, 4, LT);
    cpu.add_instruction(8, 4, EQ);
    cpu.add_instruction(9, 2, SETREL);

    cpu
}

pub fn main_func(input: &str, number: isize) -> Vec<isize> {
    let mut cpu = get_cpu(input, number);
    cpu.run();

    cpu.get_output().to_vec()
}

#[cfg(test)]
mod test {
    use crate::day09::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 1);
        assert_eq!(result, vec![3780860499]);
        println!("challenge 9.1: {:?}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func(
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                0
            ),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func("1102,34915192,34915192,7,4,7,99,0", 0),
            vec![1219070632396864]
        );
    }

    #[test]
    fn test_main_3() {
        assert_eq!(
            main_func("104,1125899906842624,99", 0),
            vec![1125899906842624]
        );
    }
}
