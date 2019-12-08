use std::io::Write;

fn render(input: &str, width: usize, height: usize) -> Vec<Vec<char>> {
    let mut chariter = input.chars();

    let mut layer = vec![vec!['2'; width]; height];

    loop {
        for i in 0..height {
            for j in 0..width {
                if let Some(curr) = chariter.next() {
                    if layer[i][j] == '2' {
                        layer[i][j] = curr;
                    }
                } else {
                    return layer;
                }
            }
        }
    }
}

fn main_func(input: &str, width: usize, height: usize) -> String {
    let rendered = render(input, width, height);
    let mut res = vec![];
    writeln!(&mut res).expect("Couldn't write");

    for row in rendered.iter().take(height) {
        for col in row.iter().take(width) {
            match col {
                '0' => write!(&mut res, " ").expect("Couldn't write"),
                '1' => write!(&mut res, "█").expect("Couldn't write"),
                '2' => write!(&mut res, "-").expect("Couldn't write"),
                _ => unreachable!(),
            }
        }
        writeln!(&mut res).expect("Couldn't write");
    }

    String::from_utf8_lossy(&res).into_owned()
}

#[cfg(test)]
mod test {
    use crate::day08::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 25, 6);
        assert_eq!(
            result,
            "\n\
             ███  █  █ ███  ████ █  █ \n\
             █  █ █  █ █  █ █    █  █ \n\
             █  █ ████ █  █ ███  █  █ \n\
             ███  █  █ ███  █    █  █ \n\
             █    █  █ █    █    █  █ \n\
             █    █  █ █    ████  ██  \n"
        );
        println!("challenge 8.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        let result = main_func("0222112222120000", 2, 2);
        assert_eq!(result, "\n █\n█ \n");
    }
}
