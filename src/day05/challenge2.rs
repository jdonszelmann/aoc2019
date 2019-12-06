use crate::day02::challenge1::{Instruction, ADD, CPU, MUL, STOP};
use crate::day05::challenge1::{INP, OUT};

pub const BT: &Instruction = &|_cpu, pc, params| {
    if params[0] != 0 {
        *pc = params[1] as usize;
        false
    } else {
        true
    }
};

pub const BF: &Instruction = &|_cpu, pc, params| {
    if params[0] == 0 {
        *pc = params[1] as usize;
        false
    } else {
        true
    }
};

pub const LT: &Instruction = &|cpu, pc, params| {
    let dst = cpu.program[*pc + 3] as usize;
    if params[0] < params[1] {
        cpu.program[dst] = 1;
    } else {
        cpu.program[dst] = 0;
    }
    true
};

pub const EQ: &Instruction = &|cpu, pc, params| {
    let dst = cpu.program[*pc + 3] as usize;
    if params[0] == params[1] {
        cpu.program[dst] = 1;
    } else {
        cpu.program[dst] = 0;
    }
    true
};

fn get_output(program: &str, inputs: Vec<isize>) -> Vec<isize> {
    let mut cpu = CPU::from(program);

    cpu.set_input(inputs);

    cpu.add_instruction(1, 4, ADD);
    cpu.add_instruction(2, 4, MUL);
    cpu.add_instruction(99, 1, STOP);

    cpu.add_instruction(3, 2, INP);
    cpu.add_instruction(4, 2, OUT);

    cpu.add_instruction(5, 3, BT);
    cpu.add_instruction(6, 3, BF);
    cpu.add_instruction(7, 4, LT);
    cpu.add_instruction(8, 4, EQ);

    cpu.run();

    cpu.get_output().to_vec()
}

#[cfg(test)]
mod test {
    use crate::day05::challenge2::get_output;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = get_output(input, vec![5]);
        assert_eq!(result, vec![5525561]);
        println!("challenge 5.2: {:?}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(get_output("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
                              vec![8]), vec![1000]);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(get_output("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
                              vec![9]), vec![1001]);
    }

    #[test]
    fn test_main_3() {
        assert_eq!(get_output("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
                              vec![7]), vec![999]);
    }

    #[test]
    fn test_main_4() {
        assert_eq!(
            get_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]),
            vec![0]
        );
    }

    #[test]
    fn test_main_5() {
        assert_eq!(
            get_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![1]),
            vec![1]
        );
    }

    #[test]
    fn test_main_6() {
        assert_eq!(
            get_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![2]),
            vec![1]
        );
    }

    #[test]
    fn test_main_7() {
        assert_eq!(
            get_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0]),
            vec![0]
        );
    }

    #[test]
    fn test_main_8() {
        assert_eq!(
            get_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![1]),
            vec![1]
        );
    }

    #[test]
    fn test_main_9() {
        assert_eq!(
            get_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![2]),
            vec![1]
        );
    }

    #[test]
    fn test_main_10() {
        assert_eq!(get_output("3,3,1107,-1,8,3,4,3,99", vec![-99]), vec![1]);
    }

    #[test]
    fn test_main_11() {
        assert_eq!(get_output("3,3,1108,-1,8,3,4,3,99", vec![7]), vec![0]);
    }
}
