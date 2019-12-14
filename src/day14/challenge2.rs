use crate::day14::challenge1::{parse_resource, Reaction};
use std::collections::HashMap;

const BASE: &str = "ORE";
pub fn count(
    value: &str,
    amount: i64,
    tree: &HashMap<String, Reaction>,
    excess: &mut HashMap<String, i64>,
    max: i64,
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
        for i in 0..multiplier as usize {
            for (name, quantity) in &reaction.requires {
                acc += super::challenge1::count(name, *quantity, tree, excess);
            }

            if i % 10000 == 0 {
                println!("{}, {}, {}", i, acc, max);
            }
            if acc > max {
                return i as i64;
            }
        }
        acc
    } else {
        panic!("NOT FOUND")
    }
}

fn main_func(input: &str, total_ore_amount: i64) -> u64 {
    let mut tree = HashMap::new();

    for line in input.lines() {
        let sides: Vec<&str> = line.split_terminator("=>").collect();

        let result = parse_resource(sides[1]);

        tree.entry(result.0.clone())
            .and_modify(|i: &mut Reaction| i.quantity = result.1)
            .or_insert_with(|| Reaction::new(result.0.clone(), result.1));

        for resource in sides[0].split_terminator(',') {
            let resource = parse_resource(resource);
            tree.entry(result.0.clone())
                .or_insert_with(|| Reaction::new(result.0.clone(), 1))
                .push(resource.0, resource.1);
        }
    }

    let mut excess = HashMap::new();
    let minimumfuel = total_ore_amount / super::challenge1::count("FUEL", 1, &tree, &mut excess);

    println!("basefuel: {}", minimumfuel);

    excess = HashMap::new();
    count(
        "FUEL",
        minimumfuel * 2,
        &tree,
        &mut excess,
        total_ore_amount,
    ) as u64
}

#[cfg(test)]
mod test {
    use crate::day14::challenge2::main_func;

    #[test]
    #[ignore]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 1000000000000);
        assert_eq!(result, 3209254);
        println!("challenge 14.2: {}", result);
    }

    #[test]
    #[ignore]
    fn test_main_3() {
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
5 BHXH, 4 VRPVC => 5 LTCX",
                1000000000000
            ),
            460664
        );
    }
}
