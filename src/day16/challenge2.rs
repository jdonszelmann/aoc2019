
fn main_func(input: &str, cycles: i64) -> String {
    let mut numbers: Vec<i64> = input
        .chars()
        .map(|i| char::to_digit(i, 10).expect("Couldn't convert") as i64)
        .collect();

    let firstseven: usize = input[0..7].parse().expect("Couldn't convert");

    numbers = numbers
        .into_iter()
        .rev()
        .cycle()
        .take(10000 * input.len() - firstseven)
        .collect();

    for _ in 0..cycles {
        numbers = numbers
            .iter()
            .scan(0i64, |total, i| {
                *total += *i;
                Some(total.abs() % 10)
            })
            .collect();
    }

    println!("{:?}", numbers);
    println!(
        "{:?}",
        numbers
            .iter()
            .skip(firstseven - 10)
            .take(8 + 10)
            .collect::<Vec<&i64>>()
    );

    numbers
        .iter()
        .rev()
        .take(8)
        .map(|i| format!("{}", i))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day16::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 100);
        assert_eq!(result, "62938399");
        println!("challenge 16.2: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func("03036732577212944063491565474664", 100),
            "84462026"
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func("02935109699940807407585447034323", 100),
            "78725270"
        );
    }

    #[test]
    fn test_main_3() {
        assert_eq!(
            main_func("03081770884921959731165446850517", 100),
            "53553731"
        );
    }
}
