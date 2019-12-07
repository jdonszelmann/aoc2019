use crate::day02::challenge1::CPU;
use crate::day07::challenge1::{get_cpu, get_permutations};

pub fn stop_cb(cpu: &mut CPU, _value: isize) {
    cpu.pause();
}

pub fn compute_permutation(cpu0: &mut CPU, permutation: Vec<isize>) -> isize {
    cpu0.output_cb = &stop_cb;

    let mut cpu1 = cpu0.clone();
    let mut cpu2 = cpu0.clone();
    let mut cpu3 = cpu0.clone();
    let mut cpu4 = cpu0.clone();

    cpu0.set_input(vec![permutation[0]]);
    cpu1.set_input(vec![permutation[1]]);
    cpu2.set_input(vec![permutation[2]]);
    cpu3.set_input(vec![permutation[3]]);
    cpu4.set_input(vec![permutation[4]]);

    let mut last = vec![0, 0, 0, 0, 0];

    loop {
        cpu0.add_to_input(last[4]);

        cpu0.resume();
        if cpu0.is_really_stopped() {
            break last[4];
        }

        last[0] = cpu0.get_output()[0];
        cpu1.add_to_input(last[0]);
        cpu0.outputbuffer = vec![];

        cpu1.resume();
        if cpu1.is_really_stopped() {
            break last[0];
        }

        last[1] = cpu1.get_output()[0];
        cpu2.add_to_input(last[1]);
        cpu1.outputbuffer = vec![];

        cpu2.resume();
        if cpu2.is_really_stopped() {
            break last[1];
        }

        last[2] = cpu2.get_output()[0];
        cpu3.add_to_input(last[2]);
        cpu2.outputbuffer = vec![];

        cpu3.resume();
        if cpu3.is_really_stopped() {
            break last[2];
        }

        last[3] = cpu3.get_output()[0];
        cpu4.add_to_input(last[3]);
        cpu3.outputbuffer = vec![];

        cpu4.resume();
        if cpu4.is_really_stopped() {
            break last[3];
        }

        last[4] = cpu4.get_output()[0];
        cpu4.outputbuffer = vec![];
    }
}

fn main_func(program: &str) -> isize {
    let mut cpu = get_cpu(program);

    let mut results = vec![];

    for i in get_permutations([5, 6, 7, 8, 9]) {
        results.push(compute_permutation(&mut cpu, i));
        cpu.reset();
    }

    *results.iter().max().expect("Should be a max")
}

#[cfg(test)]
mod test {
    use crate::day07::challenge1::get_cpu;
    use crate::day07::challenge2::{compute_permutation, main_func};

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        //        assert_eq!(result, value);
        println!("challenge 7.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        let mut cpu = get_cpu(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(
            compute_permutation(&mut cpu, vec![9, 8, 7, 6, 5]),
            139629729
        );
    }

    #[test]
    fn test_main_2() {
        let mut cpu = get_cpu("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        assert_eq!(compute_permutation(&mut cpu, vec![9, 7, 8, 5, 6]), 18216);
    }
}
