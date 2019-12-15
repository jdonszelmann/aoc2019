use crate::day02::challenge1::CPU;
use crate::day11::challenge1::get_cpu;
use crate::day15::challenge1::Direction;
use crate::day15::challenge1::Direction::{East, North, South, West};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

fn main_func(input: &str) -> u64 {
    let mut cpu = get_cpu(input);

    let last_output = RefCell::new(1);
    let walls = RefCell::new(HashSet::new());
    let had = RefCell::new(HashSet::new());

    let x = RefCell::new(0);
    let y = RefCell::new(0);

    let lx = RefCell::new(0);
    let ly = RefCell::new(0);

    let firsttime = RefCell::new(true);

    let path: RefCell<Vec<Direction>> = RefCell::new(Vec::new());
    let queue = RefCell::new(VecDeque::new());

    queue.borrow_mut().extend(
        [
            West, West, South, South, West, West, North, North, North, North, North, North, North,
            North, West, West, South, South, West, West, North, North, West, West, West, West,
            West, West, North, North, North, North, West, West, South, South, West, West, West,
            West, South, South, South, South, East, East, East, East, East, East, East, East, East,
            East, South, South, West, West, South, South, East, East, East, East, East, East,
            South, South, South, South, South, South, West, West, South, South, South, South, East,
            East, East, East, North, North, North, North, East, East, North, North, East, East,
            South, South, East, East, North, North, East, East, North, North, East, East, North,
            North, West, West, West, West, North, North, East, East, East, East, North, North,
            East, East, North, North, West, West, West, West, South, South, West, West, West, West,
            North, North, West, West, North, North, East, East, North, North, North, North, West,
            West, North, North, East, East, East, East, South, South, East, East, North, North,
            North, North, North, North, North, North, West, West, South, South, South, South, West,
            West, West, West, North, North, West, West, West, West, South, South, South, South,
            East, East, South, South, West, West, West, West, North, North, North, North, West,
            West, North, North, North, North, West, West, South, South, South, South, South, South,
            West, West, West, West, West, West, South, South, South, South, West, West, North,
            North, North, North, North, North, East, East, North, North, East, East, East, East,
            North, North, West, West, West, West, West, West, South, South,
        ]
        .iter(),
    );

    let length = RefCell::new(0);

    let cb = |_icpu: &mut CPU, value: isize| {
        *last_output.borrow_mut() = value;
    };

    let icb = |icpu: &mut CPU, _value: isize| -> isize {
        if path.borrow().len() > *length.borrow() {
            *length.borrow_mut() = path.borrow().len();
        }

        match *last_output.borrow() {
            0 => {
                walls.borrow_mut().insert((*x.borrow(), *y.borrow()));
                had.borrow_mut()
                    .insert((*lx.borrow_mut(), *ly.borrow_mut()));

                *x.borrow_mut() = *lx.borrow();
                *y.borrow_mut() = *ly.borrow();
                path.borrow_mut().pop();
                queue.borrow_mut().pop_front();
            }
            i if i >= 1 && i <= 3 => {
                if i == 2 && *firsttime.borrow() {
                    //                    print_map(&walls, &x, &y, &had, &path);
                    //                    println!("x = {}, y = {}", *x.borrow(), *y.borrow());
                    //                    println!("path = {:?}", *path.borrow());

                    path.borrow_mut().clear();
                    queue.borrow_mut().clear();
                    had.borrow_mut().clear();
                    walls.borrow_mut().clear();
                    *x.borrow_mut() = 0;
                    *y.borrow_mut() = 0;
                    *lx.borrow_mut() = 0;
                    *ly.borrow_mut() = 0;
                    *length.borrow_mut() = 0;

                    *firsttime.borrow_mut() = false;
                }

                *lx.borrow_mut() = *x.borrow();
                *ly.borrow_mut() = *y.borrow();

                if !had.borrow().contains(&(*x.borrow(), *y.borrow())) {
                    queue.borrow_mut().extend(path.borrow_mut().iter().cloned());

                    queue.borrow_mut().push_back(South);
                    queue.borrow_mut().push_back(North);

                    queue.borrow_mut().push_back(East);
                    queue.borrow_mut().push_back(West);

                    queue.borrow_mut().push_back(North);
                    queue.borrow_mut().push_back(South);

                    queue.borrow_mut().push_back(West);
                    queue.borrow_mut().push_back(East);

                    queue
                        .borrow_mut()
                        .extend(path.borrow_mut().iter().cloned().map(|i| i.reverse()).rev());
                }
                had.borrow_mut()
                    .insert((*lx.borrow_mut(), *ly.borrow_mut()));
            }
            i => unreachable!(i),
        }

        let next_dir = match queue.borrow_mut().pop_front() {
            Some(i) => i,
            None => {
                icpu.stop();
                return 1;
            }
        };
        path.borrow_mut().push(next_dir);

        let pathlen = path.borrow().len();
        if pathlen > 1 && path.borrow()[pathlen - 1] == path.borrow()[pathlen - 2].reverse() {
            path.borrow_mut().pop();
            path.borrow_mut().pop();
        }

        match next_dir {
            Direction::North => {
                *y.borrow_mut() += 1;
                1
            }
            Direction::East => {
                *x.borrow_mut() += 1;
                4
            }
            Direction::South => {
                *y.borrow_mut() -= 1;
                2
            }
            Direction::West => {
                *x.borrow_mut() -= 1;
                3
            }
        }
    };

    cpu.input_cb = &icb;
    cpu.output_cb = &cb;
    cpu.run();

    let l = *length.borrow();
    l as u64 - 1
}

#[cfg(test)]
mod test {
    use crate::day15::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 376);
        println!("challenge 15.2: {}", result);
    }
}
