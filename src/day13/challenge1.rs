use crate::day02::challenge1::CPU;
use crate::day11::challenge1::get_cpu;
use std::cell::RefCell;
use std::collections::HashSet;

fn main_func(input: &str) -> u64 {
    let mut cpu = get_cpu(input);

    let counter = RefCell::new(0);
    let x = RefCell::new(0);
    let y = RefCell::new(0);
    let blocks = RefCell::new(HashSet::new());

    let cb = |_icpu: &mut CPU, value: isize| {
        *counter.borrow_mut() += 1;

        match (*counter.borrow() - 1) % 3 {
            0 => {
                *x.borrow_mut() = value;
            }
            1 => {
                *y.borrow_mut() = value;
            }
            2 => {
                if value == 2 {
                    blocks.borrow_mut().insert((*x.borrow(), *y.borrow()));
                }
            }
            _ => unreachable!(),
        }
    };

    cpu.output_cb = &cb;

    cpu.run();

    let borrowedblocks = blocks.borrow();
    borrowedblocks.len() as u64
}

#[cfg(test)]
mod test {
    use crate::day13::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 333);
        println!("challenge 13.1: {}", result);
    }
}
