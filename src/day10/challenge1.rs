use std::collections::HashMap;

fn main_func(input: &str) -> u64 {
    let mut asteroids = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            if value == '#' {
                asteroids.push((x as f64 + 0.5, y as f64 + 0.5));
            }
        }
    }

    let mut highest = std::usize::MIN;
    for asteroid1 in &asteroids {
        let mut angles = vec![];
        for asteroid2 in &asteroids {
            let angle = (asteroid1.0 - asteroid2.0).atan2(asteroid1.1 - asteroid2.1);
            angles.push((angle, asteroid1));
        }

        // idk, this is terrible but it works
        let mut a = HashMap::new();
        a.extend(angles.iter().map(|i| ((i.0 * 1_000_000f64) as i64, i.1)));

        let length = a.len();

        if length > highest {
            highest = length;
        }
    }

    highest as u64
}

#[cfg(test)]
mod test {
    use crate::day10::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 284);
        println!("challenge 10.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(main_func(".#..#\n.....\n#####\n....#\n...##"), 8);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            ),
            33
        );
    }

    #[test]
    fn test_main_3() {
        assert_eq!(
            main_func(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            ),
            41
        );
    }

    #[test]
    fn test_main_4() {
        assert_eq!(
            main_func(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            ),
            210
        );
    }

    #[test]
    fn test_main_5() {
        assert_eq!(
            main_func(
                ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"
            ),
            30
        );
    }
}
