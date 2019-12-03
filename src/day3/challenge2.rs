
use std::collections::{HashSet, HashMap};

fn manhattan(coord: &(i32, i32)) -> i32{
    (coord.0.abs() + coord.1.abs()).abs()
}

fn find_distance(input: &str) -> i32 {

    let mut paths = vec![];
    let mut lengths = vec![];

    let mut cycle = 0;

    for path in input.split_terminator("\n") {
        let mut x = 0;
        let mut y = 0;
        let mut d = 0;
        paths.push(HashSet::new());
        lengths.push(HashMap::new());
        for dir in path.split_terminator(",") {
            match dir.chars().nth(0){
                Some('R') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y));
                        if !lengths[cycle].contains_key(&(x,y)){
                            lengths[cycle].insert((x, y), d);
                        }
                        y+=1;
                        d+=1;
                    }
                },
                Some('L') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y) );
                        if !lengths[cycle].contains_key(&(x,y)){
                            lengths[cycle].insert((x, y), d);
                        }

                        y-=1;
                        d+=1;
                    }
                },
                Some('U') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y) );
                        if !lengths[cycle].contains_key(&(x,y)){
                            lengths[cycle].insert((x, y), d);
                        }
                        x+=1;
                        d+=1;
                    }
                },
                Some('D') => {
                    for _ in 0..dir[1..].parse().expect("noint") {
                        paths[cycle].insert((x, y) );
                        if !lengths[cycle].contains_key(&(x,y)){
                            lengths[cycle].insert((x, y), d);
                        }
                        x-=1;
                        d+=1;
                    }
                },
                _ => unimplemented!(),
            }
        }
        cycle += 1;
    }

    let mut mindistance = std::i32::MAX;
    for i in paths[0].intersection(&paths[1]){
        let d1 = lengths[0].get(&i).expect("must be there");
        let d2 = lengths[1].get(&i).expect("must be there");
        if d1 + d2 < mindistance && i != &(0,0) {
            mindistance = d1 + d2;
        }
    }

    mindistance
}

#[cfg(test)]
mod test {
    use crate::day3::challenge2::find_distance;

//    #[test]
//    fn test_main_1() {
//        assert_eq!(find_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72
//U62,R66,U55,R34,D71,R55,D58,R83"), 159);
//    }
//
    #[test]
    fn test_main_3() {
        assert_eq!(find_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"), 610);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = find_distance(input);
        assert_eq!(result, 7388);
        println!("challenge 3.2: {}", result);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(find_distance("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }
}
