use crate::day2::challenge1::{ADD, CPU, MUL, STOP};

fn find_inputs(program: &str, value: usize, max: usize) -> Option<(usize, usize)> {
    let mut cpu = CPU::from(program);
    cpu.add_instruction(1, ADD);
    cpu.add_instruction(2, MUL);
    cpu.add_instruction(99, STOP);

    for i in 1..max {
        for j in 1..max {
            cpu.reset();
            cpu.set_program_byte(1, i);
            cpu.set_program_byte(2, j);

            cpu.run();
            if cpu.get_program_byte(0) == value {
                return Some((i, j));
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::day2::challenge2::find_inputs;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let inputs = find_inputs(input, 19690720, 100).unwrap();
        assert_eq!(inputs, (76, 21));
        println!("challenge 2.2: {}", 100 * inputs.0 + inputs.1)
    }
}
