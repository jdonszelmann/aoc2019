
pub fn fft(numbers: &[i64], pattern: &[i64]) -> i64 {
    numbers
        .iter()
        .zip(pattern.iter().cycle().skip(1))
        .map(|(i, j)| match j {
            0 => 0,
            1 => *i,
            -1 => -i,
            _ => unreachable!(),
        })
        .sum::<i64>()
        .abs()
        % 10
}

pub fn repeat(items: Vec<i64>, numrep: usize) -> Vec<i64> {
    items
        .into_iter()
        .flat_map(|i| vec![i].into_iter().cycle().take(numrep))
        .collect()
}

pub fn cycle(numbers: Vec<i64>) -> Vec<i64> {
    numbers
        .iter()
        .enumerate()
        .map(|(index, _)| fft(&numbers, &repeat(vec![0, 1, 0, -1], index + 1)))
        .collect::<Vec<i64>>()
}

fn main_func(input: &str, cycles: i64) -> String {
    let mut numbers: Vec<i64> = input
        .chars()
        .map(|i| char::to_digit(i, 10).expect("Couldn't convert") as i64)
        .collect();

    for _ in 0..cycles {
        numbers = cycle(numbers);
    }

    numbers.iter().take(8).map(|i| format!("{}", i)).collect()
}

#[cfg(test)]
mod test {
    use crate::day16::challenge1::{main_func, repeat};

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 100);
        assert_eq!(result, "30550349");
        println!("challenge 16.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            repeat(vec![0, 1, 0, -1], 3),
            vec![0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(main_func("12345678", 1), "48226158");
        assert_eq!(main_func("12345678", 2), "34040438");
        assert_eq!(main_func("12345678", 3), "03415518");
    }

    #[test]
    fn test_main_3() {
        assert_eq!(
            main_func("80871224585914546619083218645595", 100),
            "24176176"
        );
    }

    #[test]
    fn test_main_4() {
        assert_eq!(
            main_func("19617804207202209144916044189917", 100),
            "73745418"
        );
    }

    #[test]
    fn test_main_5() {
        assert_eq!(
            main_func("69317163492948606335995924319873", 100),
            "52432133"
        );
    }
}
