use crate::day11::challenge1::get_cpu;

fn main_func(input: &str, area: usize) -> u64 {
    let mut cpu = get_cpu(input);

    let mut output = vec![];
    for i in 0..area {
        for j in 0..area {
            cpu.add_to_input(i as isize);
            cpu.add_to_input(j as isize);
            cpu.run();

            let op = cpu.get_output()[0];
            output.push(op);

            cpu.reset()
        }
    }

    output.iter().sum::<isize>() as u64
}

#[cfg(test)]
mod test {
    use crate::day19::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 50);
        assert_eq!(result, 229);
        println!("challenge 19.1: {}", result);
    }
}
