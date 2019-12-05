use crate::day4::challenge1::is_sorted;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn criteria(i: &u64) -> bool {
    let chars: Vec<char> = format!("{}", i).chars().collect();
    if !is_sorted(&chars) {
        return false;
    }

    for i in &chars {
        if chars.iter().filter(|&n| n == i).count() == 2 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::day4::challenge1::count_options;
    use crate::day4::challenge2::criteria;

    #[test]
    fn test_main_real() {
        let result = count_options(248345, 746315, criteria);
        assert_eq!(result, 660);
        println!("challenge 4.2: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(criteria(&112233), true);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(criteria(&123444), false);
    }

    #[test]
    fn test_main_3() {
        assert_eq!(criteria(&111122), true);
    }
}
