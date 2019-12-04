fn main_func(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use crate::day8::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        //        assert_eq!(result, value);
        println!("challenge 8.2: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(main_func(""), 0);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(main_func(""), 0);
    }
}
