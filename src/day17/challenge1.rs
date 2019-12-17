use crate::day02::challenge1::CPU;
use crate::day11::challenge1::get_cpu;
use std::cell::{RefCell};

fn find_intersections(values: Vec<Vec<char>>) -> u64 {
    let mut sum = 0;

    for (y, column) in values.iter().enumerate() {
        for (x, value) in column.iter().enumerate() {
            let above = if y > 0 {
                values.get(y-1)
            } else {
                None
            };
            let below = values.get(y+1);
            let right = column.get(x+1);
            let left = if x > 0 {
                column.get(x-1)
            } else {
                None
            };

            if let (Some(above), Some(below), Some(right), Some(left)) = (above,below,right, left) {
                if value == &'#' && above.get(x) == Some(&'#') && below.get(x) == Some(&'#') && right == &'#' && left == &'#' {
                    print!("{}", 'O');
                    sum += (x * y) as u64;
                } else {
                    print!("{}", value);
                }
            } else {
                print!("{}", value);
            }
        }
        println!()
    }

    sum
}

fn main_func(input: &str) -> u64 {
    let mut cpu = get_cpu(input);

    let res = RefCell::new(Vec::new());
    res.borrow_mut().push(vec![]);

    let x = RefCell::new(0);
    let y = RefCell::new(0);

    let cb = |_icpu: &mut CPU, value: isize| {
        *x.borrow_mut() += 1;
        if value == 10 {
            *y.borrow_mut() += 1;
            *x.borrow_mut() = 0;
            res.borrow_mut().push(vec![]);
        } else {
            res.borrow_mut()[*y.borrow()].push(value as u8 as char);
        }

        print!("{}", value as u8 as char);
    };

    cpu.output_cb = &cb;
    cpu.run();


    find_intersections(res.into_inner())
}

#[cfg(test)]
mod test {
    use crate::day17::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 7780);
        println!("challenge 17.1: {}", result);
    }
}
