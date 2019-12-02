
pub fn program_to_vecusize(input: &str) -> Vec<usize> {
    input
        .split_terminator(",")
        .map(|i| i.parse())
        .filter_map(Result::ok)
        .collect()
}

fn execute_1202(input: &str) -> usize {
    let mut program = program_to_vecusize(input);

    program[1] = 12;
    program[2] = 2;

    execute_code(&mut program);

    return program[0]
}

fn execute_normal(input: &str) -> String {
    let mut program = program_to_vecusize(input);

    execute_code(&mut program);

    program
        .iter()
        .map(|i| i.to_string()).collect::<Vec<String>>().join(",")
}

pub fn execute_code(program: &mut Vec<usize>) {

    for i in (0..program.len()).step_by(4) {
        match program[i + 0] {
            1 => {
                let src1 = program[i+1];
                let src2 = program[i+2];
                let dst = program[i+3];
                program[dst] = program[src1] + program[src2];
            },
            2 => {
                let src1 = program[i+1];
                let src2 = program[i+2];
                let dst = program[i+3];
                program[dst] = program[src1] * program[src2];
            },
            99 => break,
            _ => panic!("Shouldn't happen"),
        }
    }
}



#[cfg(test)]
mod test {
    use crate::day2::challenge2::{execute_1202, execute_normal};

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = execute_1202(input);
        assert_eq!(result, 3306701);
        println!("challenge 2`.2: {}", result);
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


