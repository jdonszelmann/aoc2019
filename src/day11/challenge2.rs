#[cfg(test)]
mod test {
    use crate::day11::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, true);
        assert_eq!(result, 250);
        println!("challenge 11.1: {}", result);
    }
}
