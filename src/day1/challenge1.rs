
pub fn calculate_fuel(input: i64) -> i64 {
    (input / 3) - 2
}

pub fn calculate_fuel_str(input: &str) -> i64{
    let mut fuel_counter_upper: i64 = 0;

    for i in input.split_ascii_whitespace() {
        let number: i64 = match i.parse() {
            Ok(i) => i,
            Err(_) => continue,
        };

        fuel_counter_upper += calculate_fuel(number);
    }

    fuel_counter_upper
}


#[cfg(test)]
mod test {
    use crate::day1::challenge1::calculate_fuel_str;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        println!("{}", calculate_fuel_str(input));
    }

    #[test]
    fn test_main_1() {
        let input = include_str!("input");
        assert_eq!(calculate_fuel_str("12"), 2);
    }

    #[test]
    fn test_main_2() {
        let input = include_str!("input");
        assert_eq!(calculate_fuel_str("14"), 2);
    }

    #[test]
    fn test_main_3() {
        let input = include_str!("input");
        assert_eq!(calculate_fuel_str("1969"), 654);
    }

    #[test]
    fn test_main_4() {
        let input = include_str!("input");
        assert_eq!(calculate_fuel_str("100756"), 33583);
    }
}
