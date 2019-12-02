use crate::day2::challenge1::{execute_code, program_to_vecusize};

fn find_inputs(input: &str, value: usize, max: usize) -> Option<(usize, usize)> {
    let program = program_to_vecusize(input);

    for i in 1..max {
        for j in 1..max {
            let mut new_program = program.clone();
            new_program[1] = i;
            new_program[2] = j;
            execute_code(&mut new_program);
            if new_program[0] == value {
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
        println!("challenge 2.1: {}", 100 * inputs.0 + inputs.1)
    }
}
