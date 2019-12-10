use std::collections::{HashMap, HashSet};

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2f64) + (a.0 - b.0).powf(2f64)).sqrt()
}

fn do_rotation(
    _asteroids: Vec<(f64, f64)>,
    location: (f64, f64),
    angles: Vec<(f64, (f64, f64))>,
) -> u64 {
    let mut coordinates_per_angle = HashMap::new();
    for i in angles {
        if i.1 != location {
            coordinates_per_angle
                .entry((i.0 * 1_000_000f64) as i64)
                .and_modify(|val: &mut Vec<(f64, f64)>| val.push(i.1))
                .or_insert_with(|| vec![i.1]);
        }
    }

    let mut coordinates_per_angle_vec: Vec<(f64, Vec<(f64, f64)>)> = vec![];
    coordinates_per_angle_vec.extend(
        coordinates_per_angle
            .into_iter()
            .map(|i| (i.0 as f64 / 1_000_000f64, i.1)),
    );

    coordinates_per_angle_vec.sort_by(|i, j| {
        i.0.partial_cmp(&j.0)
            .expect("This can't go wrong")
            .reverse()
    });

    for i in &mut coordinates_per_angle_vec {
        i.1.sort_by(|i, j| {
            distance(*i, location)
                .partial_cmp(&distance(*j, location))
                .expect("This can't go wrong")
        });
    }

    let mut start_index = 0;
    for (index, i) in coordinates_per_angle_vec.iter().enumerate() {
        if i.0 <= 0f64 {
            start_index = index;
            break;
        }
    }

    let mut removed = 0;
    let mut removedcoord = (0.0, 0.0);

    while removed < 200 {
        for i in start_index..coordinates_per_angle_vec.len() {
            removedcoord = coordinates_per_angle_vec
                .get_mut(i)
                .expect("iob")
                .1
                .pop()
                .expect("empty");
            removed += 1;

            if removed == 200 {
                break;
            }
        }
        for i in 0..start_index {
            removedcoord = coordinates_per_angle_vec
                .get_mut(i)
                .expect("iob")
                .1
                .pop()
                .expect("empty");

            removed += 1;
            if removed == 200 {
                break;
            }
        }
    }

    (removedcoord.0 - 0.5) as u64 * 100 + (removedcoord.1 - 0.5) as u64
}

fn main_func(input: &str) -> u64 {
    let mut asteroids = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            if value == '#' {
                asteroids.push((x as f64 + 0.5, y as f64 + 0.5));
            }
        }
    }

    let mut highest = std::usize::MIN;
    let mut highestcoord = (0.0, 0.0);
    let mut highestcoordangles = vec![];

    for asteroid1 in &asteroids {
        let mut angles: Vec<(f64, (f64, f64))> = vec![];
        for asteroid2 in &asteroids {
            let angle = (asteroid1.0 - asteroid2.0).atan2(asteroid1.1 - asteroid2.1);
            angles.push((angle, *asteroid2));
        }

        // idk, this is terrible but it works
        let mut a: HashSet<i64> = HashSet::new();
        a.extend(angles.iter().map(|i| (i.0 * 1_000_000f64) as i64));

        let length = a.len();

        if length >= highest {
            highest = length;
            highestcoord = *asteroid1;
            highestcoordangles = angles;
        }
    }

    do_rotation(asteroids, highestcoord, highestcoordangles)
}

#[cfg(test)]
mod test {
    use crate::day10::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 404);
        println!("challenge 10.2: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            ),
            802
        );
    }
}
