fn main_func(input: &str) -> u64 {
    let mut layers: Vec<(u64, u64, u64)> = vec![];

    let mut chariter = input.chars();

    loop {
        let mut zerodigits = 0;
        let mut onedigits = 0;
        let mut twodigits = 0;

        for _ in 0..25 * 6 {
            if let Some(curr) = chariter.next() {
                match curr {
                    '0' => zerodigits += 1,
                    '1' => onedigits += 1,
                    '2' => twodigits += 1,
                    _ => (),
                }
            } else {
                let minimum = layers.iter().min_by_key(|i| i.0).expect("no min");
                return minimum.1 * minimum.2;
            }
        }
        layers.push((zerodigits, onedigits, twodigits));
    }
}

#[cfg(test)]
mod test {
    use crate::day08::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 1452);
        println!("challenge 8.1: {}", result);
    }
}
