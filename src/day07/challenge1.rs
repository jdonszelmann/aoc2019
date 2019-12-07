use crate::day02::challenge1::{ADD, CPU, MUL, STOP};
use crate::day05::challenge1::{INP, OUT};
use crate::day05::challenge2::{BF, BT, EQ, LT};

pub fn compute_permutation(cpu: &mut CPU, permutation: Vec<isize>) -> isize {
    cpu.reset();

    let mut last = 0;
    for i in permutation {
        cpu.set_input(vec![i, last]);
        cpu.run();
        let output = cpu.get_output();

        last = output[0];

        cpu.reset();
    }

    last
}

pub fn get_cpu(program: &str) -> CPU {
    let mut cpu = CPU::from(program);

    cpu.add_instruction(1, 4, ADD);
    cpu.add_instruction(2, 4, MUL);
    cpu.add_instruction(99, 1, STOP);
    cpu.add_instruction(3, 2, INP);
    cpu.add_instruction(4, 2, OUT);
    cpu.add_instruction(5, 3, BT);
    cpu.add_instruction(6, 3, BF);
    cpu.add_instruction(7, 4, LT);
    cpu.add_instruction(8, 4, EQ);

    cpu
}

/// fuck permutation algorithms; this works just fine
pub fn get_permutations(list: [isize; 5]) -> Vec<Vec<isize>> {
    let mut result = vec![];

    for i in &list {
        for j in &list {
            for k in &list {
                for l in &list {
                    for m in &list {
                        if i != j
                            && i != k
                            && i != l
                            && i != m
                            && j != k
                            && j != l
                            && j != m
                            && k != l
                            && k != m
                            && l != m
                        {
                            result.push(vec![*i, *j, *k, *l, *m])
                        }
                    }
                }
            }
        }
    }

    result
}

pub(crate) fn main_func(program: &str) -> isize {
    let mut cpu = get_cpu(program);

    let mut results = vec![];

    for i in get_permutations([0, 1, 2, 3, 4]) {
        results.push(compute_permutation(&mut cpu, i));
        cpu.reset();
    }

    *results.iter().max().expect("Should be a max")
}

#[cfg(test)]
mod test {
    use crate::day07::challenge1::{compute_permutation, get_cpu, main_func};

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        //        assert_eq!(result, value);
        println!("challenge 7.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        let mut cpu = get_cpu("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(compute_permutation(&mut cpu, vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
    }
}
