pub fn is_sorted<T>(vec: &[T]) -> bool
where
    T: PartialOrd,
{
    for i in vec.windows(2) {
        if i[0] > i[1] {
            return false;
        }
    }

    true
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn criteria(i: &u64) -> bool {
    let chars: Vec<char> = format!("{}", i).chars().collect();
    if !is_sorted(&chars) {
        return false;
    }

    for i in &chars {
        if chars.iter().filter(|&n| n == i).count() > 1 {
            return true;
        }
    }

    false
}

pub fn count_options(low: u64, high: u64, func: impl FnMut(&u64) -> bool) -> u64 {
    (low..high).filter(func).count() as u64
}

#[cfg(test)]
pub mod test {
    use crate::day4::challenge1::{count_options, criteria, is_sorted};

    #[test]
    pub fn test_main_real() {
        let result = count_options(248345, 746315, criteria);
        assert_eq!(result, 1019);
        println!("challenge 4.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(criteria(&111111), true);
    }

    #[test]
    fn test_main_2() {
        assert_eq!(criteria(&223450), false);
    }

    #[test]
    fn test_main_3() {
        assert_eq!(criteria(&123789), false);
    }

    #[test]
    fn test_main_4() {
        assert_eq!(is_sorted(&vec![1, 2, 3]), true);
    }

    #[test]
    fn test_main_5() {
        assert_eq!(is_sorted(&vec![3, 2, 1]), false);
    }

    #[test]
    fn test_main_6() {
        assert_eq!(is_sorted(&vec![1, 3, 2, 5]), false);
    }

    #[test]
    fn test_main_7() {
        assert_eq!(is_sorted(&vec![1, 1, 2]), true);
    }
}
