use crate::day18::challenge1::Feature::{Door, Key};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Maze {
    pub maze: Vec<Vec<char>>,
    pub start: (usize, usize),
    pub keys: usize,
    pub doors: usize,
    pub length: u64,

    pub collected_keys: HashSet<char>,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let start = maze
            .iter()
            .map(|i| i.iter().position(|i| i == &'@'))
            .enumerate()
            .filter(|i| i.1.is_some())
            .map(|i| (i.0, i.1.unwrap()))
            .next()
            .expect("No @ found");

        let keys = maze
            .iter()
            .map(|i| i.iter().filter(|i| i.is_lowercase()).count())
            .sum();

        let doors = maze
            .iter()
            .map(|i| i.iter().filter(|i| i.is_uppercase()).count())
            .sum();

        Self {
            maze,
            start,
            keys,
            doors,
            length: 0,
            collected_keys: HashSet::new(),
        }
    }

    pub fn set_start(&mut self, (y, x): (usize, usize)) {
        self.maze[self.start.0][self.start.1] = '.';
        self.maze[y][x] = '@';
        self.start = (y, x);
    }

    pub fn options(&self, (y, x): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];

        if y > self.maze.len() || self.maze.is_empty() || x > self.maze[0].len() {
            return res;
        }

        if x > 0 && self.maze[y][x - 1] != '#' {
            res.push((y, x - 1));
        }

        if y > 0 && self.maze[y - 1][x] != '#' {
            res.push((y - 1, x));
        }

        if x < self.maze[0].len() - 1 && self.maze[y][x + 1] != '#' {
            res.push((y, x + 1));
        }

        if y < self.maze.len() - 1 && self.maze[y + 1][x] != '#' {
            res.push((y + 1, x));
        }

        res
    }

    pub fn collect(&mut self, (y, x): (usize, usize)) {
        if self.maze[y][x].is_uppercase() {
            self.maze[y][x] = '.';
            self.doors -= 1;
        } else if self.maze[y][x].is_lowercase() {
            self.collected_keys
                .insert(self.maze[y][x].to_ascii_uppercase());
            self.maze[y][x] = '.';
            self.keys -= 1;
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in &self.maze {
            for j in i {
                write!(f, "{}", j)?;
            }
            writeln!(f)?
        }

        write!(f, "keys: {}, doors: {}", self.keys, self.doors)
    }
}

#[derive(Debug)]
enum Feature {
    Door,
    Key,
}

fn floodfill(maze: &Maze) -> Vec<(Feature, u64, (usize, usize))> {
    let mut queue = VecDeque::new();
    queue.push_back((maze.start, 0));
    let mut had = HashMap::new();

    let mut features = vec![];

    while let Some(item) = queue.pop_front() {
        if let Some(distance) = had.get(&item.0) {
            if *distance <= item.1 {
                continue;
            }
        }

        had.insert(item.0, item.1);

        let curr = maze.maze[(item.0).0][(item.0).1];
        let newitem = item;

        if curr.is_alphabetic() && curr.is_uppercase() {
            features.push((Door, newitem.1, newitem.0));
        } else if curr.is_alphabetic() && curr.is_lowercase() {
            features.push((Key, newitem.1, newitem.0));
        }

        let options = maze.options(newitem.0);

        for option in options {
            let value = maze.maze[option.0][option.1];

            if value.is_uppercase() {
                if maze.collected_keys.contains(&value) {
                    features.push((Door, newitem.1 + 1, (option.0, option.1)));
                }
                continue;
            }

            queue.push_back((option, newitem.1 + 1))
        }
    }

    features
}

fn one_cycle(mut smallest: u64, current_maze: Maze) -> (u64, Vec<Maze>) {
    let mut possibilities = vec![];

    if current_maze.length > smallest {
        return (smallest, vec![]);
    }

    let mut possible_features = floodfill(&current_maze);

    possible_features.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    for p in possible_features {
        let mut newmaze = current_maze.clone();
        newmaze.collect(p.2);

        newmaze.set_start(p.2);
        newmaze.length += p.1 as u64;

        if newmaze.keys > 0 || newmaze.doors > 0 && newmaze.length < smallest {
            possibilities.push(newmaze)
        } else if newmaze.length < smallest {
            smallest = newmaze.length;
        }
    }

    (smallest, possibilities)
}

fn main_func(input: &str) -> u64 {
    let maze = Maze::new(input);

    println!("{}", maze);

    let mut possibilities = Vec::new();
    possibilities.push(maze);

    let mut tried = 0;

    let mut smallest = 4000;

    while let Some(current_maze) = possibilities.pop() {
        tried += 1;
        if tried % 10000 == 0 {
            println!("iteration {} with length {}", tried, smallest);
        }

        let (newsmallest, newmazes) = one_cycle(smallest, current_maze);

        if newsmallest < smallest {
            smallest = newsmallest;
        }
        possibilities.extend(newmazes.into_iter());
    }

    smallest
}

#[cfg(test)]
mod test {
    use crate::day18::challenge1::main_func;

    #[test]
    #[ignore]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        //        assert_eq!(result, value);
        println!("challenge 18.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func(
                "#########
#b.A.@.a#
#########"
            ),
            8
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func(
                "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
            ),
            86
        );
    }

    #[test]
    fn test_main_3() {
        assert_eq!(
            main_func(
                "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
"
            ),
            132
        );
    }

    #[test]
    #[ignore]
    fn test_main_4() {
        assert_eq!(
            main_func(
                "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
"
            ),
            136
        );
    }

    #[test]
    fn test_main_5() {
        assert_eq!(
            main_func(
                "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
"
            ),
            81
        );
    }
}
