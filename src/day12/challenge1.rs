use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Moon {
    pub position: (i64, i64, i64),
    pub velocity: (i64, i64, i64),
    index: u64,
}

impl Moon {
    pub fn new(position: (i64, i64, i64), index: u64) -> Self {
        Self {
            position,
            velocity: (0, 0, 0),
            index,
        }
    }

    fn apply_gravity_one_moon(&mut self, moon: &mut Moon) {
        match moon.position.0.cmp(&self.position.0) {
            Ordering::Greater => {
                self.velocity.0 += 1;
                moon.velocity.0 -= 1;
            }
            Ordering::Less => {
                self.velocity.0 -= 1;
                moon.velocity.0 += 1;
            }
            _ => (),
        };

        match moon.position.1.cmp(&self.position.1) {
            Ordering::Greater => {
                self.velocity.1 += 1;
                moon.velocity.1 -= 1;
            }
            Ordering::Less => {
                self.velocity.1 -= 1;
                moon.velocity.1 += 1;
            }
            _ => (),
        };

        match moon.position.2.cmp(&self.position.2) {
            Ordering::Greater => {
                self.velocity.2 += 1;
                moon.velocity.2 -= 1;
            }
            Ordering::Less => {
                self.velocity.2 -= 1;
                moon.velocity.2 += 1;
            }
            _ => (),
        };
    }

    pub fn update_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    pub fn energy(&self) -> u64 {
        let kinetic = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        let potential = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();

        (kinetic * potential) as u64
    }
}

fn calculate_energy(moons: Vec<Moon>) -> u64 {
    moons.into_iter().map(|m| m.energy()).sum()
}

pub fn apply_timestep(moons: &mut Vec<Moon>) {
    let (first, rest) = moons.split_at_mut(1);
    let (second, rest) = rest.split_at_mut(1);
    let (third, rest) = rest.split_at_mut(1);
    let (fourth, _) = rest.split_at_mut(1);

    first[0].apply_gravity_one_moon(&mut second[0]);
    first[0].apply_gravity_one_moon(&mut third[0]);
    first[0].apply_gravity_one_moon(&mut fourth[0]);

    second[0].apply_gravity_one_moon(&mut third[0]);
    second[0].apply_gravity_one_moon(&mut fourth[0]);

    third[0].apply_gravity_one_moon(&mut fourth[0]);

    first[0].update_position();
    second[0].update_position();
    third[0].update_position();
    fourth[0].update_position();
}

fn main_func(mut moons: Vec<Moon>, steps: u64) -> u64 {
    for _ in 0..steps {
        apply_timestep(&mut moons);
    }

    calculate_energy(moons)
}

#[cfg(test)]
mod test {
    use crate::day12::challenge1::{main_func, Moon};

    #[test]
    fn test_main_real() {
        let result = main_func(
            vec![
                Moon::new((-6, -5, -8), 0),
                Moon::new((0, -3, -13), 1),
                Moon::new((-15, 10, -11), 2),
                Moon::new((-3, -8, 3), 3),
            ],
            1000,
        );
        assert_eq!(result, 5937);
        println!("challenge 12.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func(
                vec![
                    Moon::new((-1, 0, 2), 0),
                    Moon::new((2, -10, -7), 1),
                    Moon::new((4, -8, 8), 2),
                    Moon::new((3, 5, -1), 3),
                ],
                10
            ),
            179
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func(
                vec![
                    Moon::new((-8, -10, 0), 0),
                    Moon::new((5, 5, 10), 1),
                    Moon::new((2, -7, 3), 2),
                    Moon::new((9, -8, -3), 3),
                ],
                100
            ),
            1940
        );
    }
}
