use crate::day6::challenge1::build_orbittree;

fn main_func(input: &str) -> u64 {
    let tree = build_orbittree(input);

    tree.leaf_path("YOU", "SAN").expect("Should exist")
}

#[cfg(test)]
mod test {
    use crate::day6::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 484);
        println!("challenge 6.2: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
            4
        );
    }
}
