use std::collections::HashSet;

fn manhattan(coord: &(i32, i32)) -> i32 {
    coord.0.abs() + coord.1.abs()
}

fn find_distance(input: &str) -> i32 {
    let mut paths = vec![];

    for (cycle, path) in input.lines().enumerate() {
        let mut x = 0;
        let mut y = 0;
        paths.push(HashSet::new());
        for dir in path.split_terminator(',') {
            match dir.chars().nth(0) {
                Some('R') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y));
                        y += 1;
                    }
                }
                Some('L') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y));
                        y -= 1;
                    }
                }
                Some('U') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y));
                        x += 1;
                    }
                }
                Some('D') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y));
                        x -= 1;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    paths[0]
        .intersection(&paths[1])
        .map(manhattan)
        .filter(|i| *i != 0)
        .min()
        .expect("no minimum")
}

#[cfg(test)]
mod test {
    use crate::day3::challenge1::find_distance;

    #[test]
    fn test_main_1() {
        assert_eq!(
            find_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
    }

    #[test]
    fn test_main_3() {
        assert_eq!(find_distance("R8,U5,L5,D3,\nU7,R6,D4,L4"), 6);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = find_distance(input);
        assert_eq!(result, 721);
        println!("challenge 3.1: {}", result);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            find_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}
