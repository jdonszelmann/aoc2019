use crate::day02::challenge1::{ADD, CPU, MUL, STOP};
use crate::day05::challenge1::{INP, OUT};
use crate::day05::challenge2::{BF, BT, EQ, LT};
use crate::day09::challenge1::SETREL;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn print_board(board: &HashSet<(i64, i64)>) {
    for i in (-5..2).rev() {
        for j in 1..40 {
            if board.contains(&(j, i)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn get_cpu(program: &str) -> CPU {
    let mut cpu = CPU::from(program);

    cpu.add_instruction(1, 4, ADD);
    cpu.add_instruction(2, 4, MUL);
    cpu.add_instruction(99, 1, STOP);
    cpu.add_instruction(3, 2, INP);
    cpu.add_instruction(4, 2, OUT);
    cpu.add_instruction(5, 3, BT);
    cpu.add_instruction(6, 3, BF);
    cpu.add_instruction(7, 4, LT);
    cpu.add_instruction(8, 4, EQ);
    cpu.add_instruction(9, 2, SETREL);

    cpu
}

pub fn main_func(input: &str, startwhite: bool) -> u64 {
    let mut cpu = get_cpu(input);

    let mut white_panels: HashSet<(i64, i64)> = HashSet::new();
    let mut painted_once: HashSet<(i64, i64)> = HashSet::new();

    let direction = Rc::new(RefCell::new(0));
    let color = Rc::new(RefCell::new(false));
    let second = Rc::new(RefCell::new(false));

    let cbdir = direction.clone();
    let cbcol = color.clone();
    let cbsecond = second.clone();

    let cb = |icpu: &mut CPU, value: isize| {
        let second = *cbsecond.borrow();

        if second {
            match value {
                0 => {
                    *cbdir.borrow_mut() -= 1;
                    if *cbdir.borrow() < 0 {
                        *cbdir.borrow_mut() = 3;
                    };
                }
                1 => {
                    *cbdir.borrow_mut() += 1;

                    if *cbdir.borrow() > 3 {
                        *cbdir.borrow_mut() = 0;
                    };
                }
                _ => unimplemented!(),
            }
        } else {
            *cbcol.borrow_mut() = value != 0;
        }

        *cbsecond.borrow_mut() = !second;
        icpu.pause()
    };

    cpu.output_cb = &cb;

    let mut x = 0;
    let mut y = 0;

    if startwhite {
        white_panels.insert((0, 0));
    }

    while !cpu.is_really_stopped() {
        let is_white;
        if white_panels.contains(&(x, y)) {
            is_white = true;
        } else {
            is_white = false
        }

        cpu.add_to_input(is_white as isize);
        cpu.resume();
        cpu.resume();

        if *color.borrow() {
            white_panels.insert((x, y));
        } else if white_panels.contains(&(x, y)) {
            white_panels.remove(&(x, y));
        }

        painted_once.insert((x, y));

        match *direction.borrow() {
            0 => y += 1,
            1 => x += 1,
            2 => y -= 1,
            3 => x -= 1,
            _ => unimplemented!(),
        }
    }

    if startwhite {
        print_board(&white_panels);
    }
    painted_once.len() as u64
}

#[cfg(test)]
mod test {
    use crate::day11::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, false);
        assert_eq!(result, 2319);
        println!("challenge 11.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        let input = include_str!("inputvictor");
        let result = main_func(input, false);
        assert_eq!(result, 2093);
    }

    #[test]
    fn test_main_2() {
        let input = include_str!("inputnoah");
        let result = main_func(input, false);
        assert_eq!(result, 2173);
    }
}
