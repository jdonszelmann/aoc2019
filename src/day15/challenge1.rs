use crate::day02::challenge1::CPU;
use crate::day11::challenge1::get_cpu;
use crate::day15::challenge1::Direction::{East, North, South, West};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

pub fn print_map(
    walls: &RefCell<HashSet<(isize, isize)>>,
    x: &RefCell<isize>,
    y: &RefCell<isize>,
    had: &RefCell<HashSet<(isize, isize)>>,
    path: &RefCell<Vec<Direction>>,
) {
    let mut cx = 0;
    let mut cy = 0;
    let mut pathcoords = HashSet::new();
    for dir in path.borrow().iter() {
        match dir {
            North => cy += 1,
            East => cx += 1,
            South => cy -= 1,
            West => cx -= 1,
        }
        assert!(pathcoords.insert((cx, cy)));
    }

    for i in (-20..60).rev() {
        for j in -30..80 {
            if j == *x.borrow() && i == *y.borrow() {
                print!("D");
            } else if walls.borrow().contains(&(j, i)) {
                print!("#")
            } else if pathcoords.contains(&(j, i)) {
                print!("X")
            } else if had.borrow().contains(&(j, i)) {
                print!(".")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main_func(input: &str) -> u64 {
    let mut cpu = get_cpu(input);

    let last_output = RefCell::new(1);
    let walls = RefCell::new(HashSet::new());
    let had = RefCell::new(HashSet::new());

    let x = RefCell::new(0);
    let y = RefCell::new(0);

    let lx = RefCell::new(0);
    let ly = RefCell::new(0);

    let path: RefCell<Vec<Direction>> = RefCell::new(Vec::new());
    let queue = RefCell::new(VecDeque::new());

    let length = RefCell::new(0);

    let cb = |_icpu: &mut CPU, value: isize| {
        *last_output.borrow_mut() = value;
    };

    let icb = |icpu: &mut CPU, _value: isize| -> isize {
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
            1 => {
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
            2 => {
                //                print_map(&walls, &x, &y, &had, &path);
                //                println!("x = {}, y = {}", *x.borrow(), *y.borrow());
                //                println!("path = {:?}", *path.borrow());

                *length.borrow_mut() = path.borrow().len();
                icpu.stop();

                return 1;
            }
            i => unreachable!(i),
        }

        let next_dir = queue.borrow_mut().pop_front().expect("Empty");
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
    l as u64
}

#[cfg(test)]
mod test {
    use crate::day15::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 246);
        println!("challenge 15.1: {}", result);
    }
}
