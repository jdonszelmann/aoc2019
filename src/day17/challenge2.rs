use crate::day11::challenge1::get_cpu;

fn main_func(input: &str) -> u64 {
    let mut cpu = get_cpu(input);
    cpu.set_program_byte(0, 2);

    cpu.extend_to_input(
        "A,B,A,C,B,C,B,A,C,B\n"
            .chars()
            .map(|i| i as u8 as isize)
            .collect(),
    );
    cpu.extend_to_input(
        "L,6,R,8,R,12,L,6,L,8\n"
            .chars()
            .map(|i| i as u8 as isize)
            .collect(),
    );
    cpu.extend_to_input(
        "L,10,L,8,R,12\n"
            .chars()
            .map(|i| i as u8 as isize)
            .collect(),
    );
    cpu.extend_to_input(
        "L,8,L,10,L,6,L,6\n"
            .chars()
            .map(|i| i as u8 as isize)
            .collect(),
    );

    cpu.extend_to_input("n\n".chars().map(|i| i as u8 as isize).collect());

    cpu.run();

    let res = cpu.outputbuffer.last().expect("no last");
    *res as u64
}

#[cfg(test)]
mod test {
    use crate::day17::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 1075882);
        println!("challenge 17.2: {}", result);
    }
}
