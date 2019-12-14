use crate::day02::challenge1::CPU;
use crate::day11::challenge1::get_cpu;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tty_read::{ReaderOptions, TermReader};

fn print_board(board: &RefCell<[[char; 44]; 20]>, score: &RefCell<isize>) {
    println!("\x1b[2J\r");
    for col in board.borrow().iter() {
        for value in col.iter() {
            print!("{}", value);
            //            if value == &'⃝' {
            //                print!(" ");
            //            }
        }
        println!("\r")
    }

    println!("\r\nScore: {}", *score.borrow());
}

lazy_static! {
    static ref OPTIONS: ReaderOptions = ReaderOptions::default();
}

pub fn arkanoid() {
    main_func(include_str!("input"), true, true);
}

pub fn main_func(input: &str, printing: bool, manual: bool) -> u64 {
    let mut cpu = get_cpu(input);
    cpu.set_program_byte(0, 2);

    let (sender, receiver) = mpsc::channel();
    if manual {
        thread::spawn(move || {
            let reader = TermReader::open_stdin(&OPTIONS).expect("failed to open stdin reader");
            loop {
                let byte = match reader.read_byte() {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                match sender.send(byte) {
                    Ok(_) => (),
                    Err(_) => continue,
                };
            }
        });
    }

    let counter = RefCell::new(0);
    let x = RefCell::new(0);
    let y = RefCell::new(0);

    let board: RefCell<[[char; 44]; 20]> = RefCell::new([[' '; 44]; 20]);

    let scoreseq = RefCell::new(false);
    let score = RefCell::new(0);
    let ballx = RefCell::new(0);
    let pedalx = RefCell::new(0);
    let should_print = RefCell::new(printing);

    let cb = |_: &mut CPU, value: isize| {
        *counter.borrow_mut() += 1;

        match (*counter.borrow() - 1) % 3 {
            0 => {
                if value == -1 {
                    *scoreseq.borrow_mut() = true
                } else {
                    *x.borrow_mut() = value;
                }
            }
            1 => {
                if *scoreseq.borrow() && value != 0 {
                    unreachable!()
                } else {
                    *y.borrow_mut() = value;
                }
            }
            2 => {
                if *scoreseq.borrow() {
                    *score.borrow_mut() = value;
                    *scoreseq.borrow_mut() = false;
                } else {
                    let character = match value {
                        0 => ' ',
                        1 => '█',
                        2 => '#',
                        3 => {
                            *pedalx.borrow_mut() = *x.borrow();
                            '—'
                        }
                        4 => {
                            *ballx.borrow_mut() = *x.borrow();
                            '⃝'
                        }
                        _ => unreachable!(),
                    };

                    board.borrow_mut()[*y.borrow() as usize][*x.borrow() as usize] = character;
                }
            }
            _ => unreachable!(),
        }

        if *x.borrow() == 43 && *y.borrow() == 19 && *should_print.borrow() {
            print_board(&board, &score)
        }
    };

    let icb = |_: &mut CPU, _: isize| -> isize {
        if *should_print.borrow() {
            print_board(&board, &score);
        }

        if manual {
            match receiver.recv_timeout(Duration::from_millis(300)) {
                Err(_) => 0,
                Ok(c) => match c as char {
                    'A' | 'a' => -1,
                    'D' | 'd' => 1,
                    _ => 0,
                },
            }
        } else if ballx > pedalx {
            1
        } else if ballx < pedalx {
            -1
        } else {
            0
        }
    };

    cpu.output_cb = &cb;
    cpu.input_cb = &icb;

    cpu.run();

    if *should_print.borrow() {
        print_board(&board, &score);
    }

    let borrowedscore = score.borrow();
    *borrowedscore as u64
}
#[cfg(test)]
mod test {
    use crate::day13::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, false, false);
        assert_eq!(result, 16539);
        println!("challenge 13.2: {}", result);
    }

    #[test]
    #[ignore]
    fn test_main_print() {
        let input = include_str!("input");
        let result = main_func(input, true, false);
        assert_eq!(result, 16539);
        println!("challenge 13.2: {}", result);
    }
}
