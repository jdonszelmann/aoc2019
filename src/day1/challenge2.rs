use crate::day1::challenge1::calculate_fuel;

pub fn calculate_total_fuel(input: i64) -> i64 {
    let mut last = calculate_fuel(input);
    if last <= 0 {
        return last;
    }

    let mut total = last;
    loop {
        let new = calculate_fuel(last);
        if new <= 0 {
            return total;
        }
        total += new;
        last = new;
    }
}

pub fn calculate_full_fuel(input: &str) -> i64 {
    let mut fuel_counter_upper = 0;

    for i in input.split_ascii_whitespace() {
        let number: i64 = match i.parse() {
            Ok(i) => i,
            Err(_) => continue,
        };

        fuel_counter_upper += calculate_total_fuel(number);
    }

    fuel_counter_upper
}

#[cfg(test)]
mod test {
    use crate::day1::challenge2::calculate_full_fuel;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = calculate_full_fuel(input);
        assert_eq!(result, 5010211);
        println!("challenge 1.2: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(calculate_full_fuel("14"), 2);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(calculate_full_fuel("1969"), 966);
    }

    #[test]
    fn test_main_3() {
        assert_eq!(calculate_full_fuel("100756"), 50346);
    }
}
