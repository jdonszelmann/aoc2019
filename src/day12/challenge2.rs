use crate::day12::challenge1::{apply_timestep, Moon};
use num::Integer;
use std::collections::HashSet;

type Iterationstate = (i64, i64, i64, i64, i64, i64, i64, i64);

pub fn find_periods(mut moons: Vec<Moon>) -> u64 {
    let mut xperiods: HashSet<Iterationstate> = Default::default();
    let mut yperiods: HashSet<Iterationstate> = Default::default();
    let mut zperiods: HashSet<Iterationstate> = Default::default();

    let mut xperiod = 0;
    let mut yperiod = 0;
    let mut zperiod = 0;

    loop {
        apply_timestep(&mut moons);
        if !xperiods.insert((
            moons[0].position.0,
            moons[0].velocity.0,
            moons[1].position.0,
            moons[1].velocity.0,
            moons[2].position.0,
            moons[2].velocity.0,
            moons[3].position.0,
            moons[3].velocity.0,
        )) {
            xperiod = xperiods.len();
        }
        if !yperiods.insert((
            moons[0].position.1,
            moons[0].velocity.1,
            moons[1].position.1,
            moons[1].velocity.1,
            moons[2].position.1,
            moons[2].velocity.1,
            moons[3].position.1,
            moons[3].velocity.1,
        )) {
            yperiod = yperiods.len();
        }
        if !zperiods.insert((
            moons[0].position.2,
            moons[0].velocity.2,
            moons[1].position.2,
            moons[1].velocity.2,
            moons[2].position.2,
            moons[2].velocity.2,
            moons[3].position.2,
            moons[3].velocity.2,
        )) {
            zperiod = zperiods.len();
        }

        if xperiod != 0 && yperiod != 0 && zperiod != 0 {
            break;
        }
    }

    xperiod.lcm(&yperiod).lcm(&zperiod) as u64
}

#[cfg(test)]
mod test {
    use crate::day12::challenge1::Moon;
    use crate::day12::challenge2::find_periods;

    #[test]
    fn test_main_real() {
        let result = find_periods(vec![
            Moon::new((-6, -5, -8), 0),
            Moon::new((0, -3, -13), 1),
            Moon::new((-15, 10, -11), 2),
            Moon::new((-3, -8, 3), 3),
        ]);
        assert_eq!(result, 376203951569712);
        println!("challenge 12.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            find_periods(vec![
                Moon::new((-1, 0, 2), 0),
                Moon::new((2, -10, -7), 1),
                Moon::new((4, -8, 8), 2),
                Moon::new((3, 5, -1), 3),
            ]),
            2772
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            find_periods(vec![
                Moon::new((-8, -10, 0), 0),
                Moon::new((5, 5, 10), 1),
                Moon::new((2, -7, 3), 2),
                Moon::new((9, -8, -3), 3),
            ]),
            4686774924
        );
    }
}
