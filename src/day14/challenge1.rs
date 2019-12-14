use std::collections::HashMap;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Reaction {
    pub name: String,
    pub quantity: i64,
    pub requires: Vec<(String, i64)>,
}

impl Reaction {
    pub fn new(name: String, quantity: i64) -> Self {
        Self {
            name,
            quantity,
            requires: vec![],
        }
    }

    pub fn push(&mut self, value: String, amount: i64) {
        self.requires.push((value, amount))
    }
}

pub fn parse_resource(string: &str) -> (String, i64) {
    let sides: Vec<&str> = string.trim().split_terminator(" ").collect();
    let quantity: i64 = sides[0].parse().expect("Couldn't parse int");
    let name = sides[1].into();

    (name, quantity)
}

const BASE: &str = "ORE";
pub fn count(
    value: &str,
    amount: i64,
    tree: &HashMap<String, Reaction>,
    excess: &mut HashMap<String, i64>,
) -> i64 {
    if value == BASE {
        return amount;
    }

    if let Some(reaction) = tree.get(value) {
        let mut multiplier = ((amount as f64) / (reaction.quantity as f64)).ceil() as i64;

        if reaction.quantity * multiplier > amount {
            let val = excess.entry(reaction.name.clone()).or_insert(0);
            *val += reaction.quantity * multiplier - amount;
            while *val >= reaction.quantity {
                *val -= reaction.quantity;
                multiplier -= 1;
            }
        }

        let mut acc = 0;
        for _ in 0..multiplier as usize {
            for (name, quantity) in &reaction.requires {
                acc += count(name, *quantity, tree, excess);
            }
        }
        acc
    } else {
        panic!("NOT FOUND")
    }
}

fn main_func(input: &str) -> i64 {
    let mut tree = HashMap::new();

    for line in input.lines() {
        let sides: Vec<&str> = line.split_terminator("=>").collect();

        let result = parse_resource(sides[1]);

        tree.entry(result.0.clone())
            .and_modify(|i: &mut Reaction| i.quantity = result.1)
            .or_insert(Reaction::new(result.0.clone(), result.1));

        for resource in sides[0].split_terminator(",") {
            let resource = parse_resource(resource);
            tree.entry(result.0.clone())
                .or_insert(Reaction::new(result.0.clone(), 1))
                .push(resource.0, resource.1);
        }
    }

    let mut excess = HashMap::new();
    count("FUEL", 1, &tree, &mut excess)
}

#[cfg(test)]
mod test {
    use crate::day14::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 628586);
        println!("challenge 14.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func(
                "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            ),
            31
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func(
                "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
            ),
            165
        );
    }

    #[test]
    fn test_main_3() {
        assert_eq!(
            main_func(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            ),
            13312
        );
    }

    #[test]
    fn test_main_4() {
        assert_eq!(
            main_func(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
            ),
            180697
        );
    }

    #[test]
    fn test_main_5() {
        assert_eq!(
            main_func(
                "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
            ),
            2210736
        );
    }

    #[test]
    fn test_main_6() {
        assert_eq!(main_func("10 ORE => 1 FUEL"), 10);
    }

    #[test]
    fn test_main_7() {
        assert_eq!(
            main_func(
                "10 ORE => 1 A
1 A => 1 FUEL"
            ),
            10
        );
    }

    #[test]
    fn test_main_8() {
        assert_eq!(
            main_func(
                "10 ORE => 1 A
2 A => 1 FUEL"
            ),
            20
        );
    }

    #[test]
    fn test_main_9() {
        assert_eq!(
            main_func(
                "10 ORE => 2 A
2 A => 1 FUEL"
            ),
            10
        );
    }

    #[test]
    fn test_main_10() {
        assert_eq!(
            main_func(
                "10 ORE => 2 A
3 A => 1 FUEL"
            ),
            20
        );
    }

    #[test]
    fn test_main_11() {
        assert_eq!(
            main_func(
                "10 ORE => 2 A
1 A => 1 FUEL"
            ),
            10
        );
    }

    #[test]
    fn test_main_12() {
        assert_eq!(
            main_func(
                "10 ORE => 2 A
5 A => 3 B
2 A, 2 B => 2 C
3 A, 3 C => 5 D
4 D => 1 FUEL"
            ),
            90
        );
    }
}
