use std::ops;
use std::cmp::Ordering;
use advent_of_code::lcm;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut bodies = [
        Body::new(8, 0, 8),
        Body::new(0, -5, -10),
        Body::new(16, 10, -5),
        Body::new(19, -10, -7),
    ];
    for _ in 0..1_000 {
        step(&mut bodies);
    }
    let energy: i64 = bodies.iter().map(|e| e.get_energy()).sum();
    dbg!(energy);
}

fn part2() {
    let initial_x = vec![
        Body1d { position: 8, velocity: 0 },
        Body1d { position: 0, velocity: 0 },
        Body1d { position: 16, velocity: 0 },
        Body1d { position: 19, velocity: 0 },
    ];
    let initial_y = vec![
        Body1d { position: 0, velocity: 0 },
        Body1d { position: -5, velocity: 0 },
        Body1d { position: 10, velocity: 0 },
        Body1d { position: -10, velocity: 0 },
    ];
    let initial_z = vec![
        Body1d { position: 8, velocity: 0 },
        Body1d { position: -10, velocity: 0 },
        Body1d { position: -5, velocity: 0 },
        Body1d { position: -7, velocity: 0 },
    ];
    let x_period = dbg!(get_period(&initial_x));
    let y_period = dbg!(get_period(&initial_y));
    let z_period = dbg!(get_period(&initial_z));
    dbg!(lcm(lcm(x_period, y_period), z_period));
}

fn get_period(initial: &[Body1d]) -> i64 {
    // assumes that everything returns to its initial position
    // (which is true for the examples and my input which is good enough for me)
    let state: &mut [Body1d] = &mut initial.to_owned();
    let mut counter = 0;
    loop {
        step_1d(state);
        counter += 1;
        if state == initial {
            return counter
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Body {
    position: Vector,
    velocity: Vector,
}

impl Body {
    fn new(x: i64, y: i64, z: i64) -> Body {
        Body {
            position: Vector { x, y, z },
            velocity: Vector { x: 0, y: 0, z: 0 },
        }
    }

    fn apply_velocity(&mut self) {
        self.position += &self.velocity;
    }

    fn get_energy(&self) -> i64 {
        self.position.energy() * self.velocity.energy()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn get_gravity_adjustment(&self, other: &Vector) -> Vector {
        Vector {
            x: match self.x.cmp(&other.x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1
            },
            y: match self.y.cmp(&other.y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1
            },
            z: match self.z.cmp(&other.z) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1
            },
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

fn step(bodies: &mut [Body]) {
    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i != j {
                let other_pos = bodies[j].position.clone();
                let body1 = &mut bodies[i];
                body1.velocity += &body1.position.get_gravity_adjustment(&other_pos);
            }
        }
    }
    for body in bodies.iter_mut() {
        body.apply_velocity();
    }
}

#[derive(PartialEq, Clone)]
struct Body1d {
    position: i64,
    velocity: i64,
}

fn step_1d(bodies: &mut [Body1d]) {
    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i != j {
                let other_pos = bodies[j].position;
                bodies[i].velocity += match bodies[i].position.cmp(&other_pos) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1
                }
            }
        }
    }
    for body in bodies.iter_mut() {
        body.position += body.velocity;
    }
}

#[test]
fn test_example1() {
    let mut bodies = [
        Body::new(-1, 0, 2),
        Body::new(2, -10, -7),
        Body::new(4, -8, 8),
        Body::new(3, 5, -1),
    ];
    step(&mut bodies);
    assert_eq!(bodies[3].velocity, Vector { x: -1, y: -3, z: 1 });
    assert_eq!(bodies[3].position, Vector { x: 2, y: 2, z: 0 });
    step(&mut bodies);
    assert_eq!(bodies[3].velocity, Vector { x: -1, y: -6, z: 2 });
    assert_eq!(bodies[3].position, Vector { x: 1, y: -4, z: 2 });
    for _ in 0..8 {
        step(&mut bodies);
    }
    let energy: i64 = bodies.iter().map(|e| e.get_energy()).sum();
    assert_eq!(energy, 179);
}

#[test]
fn test_1d() {
    let mut bodies = [
        Body1d { position: -1, velocity: 0 },
        Body1d { position: 2, velocity: 0 },
        Body1d { position: 4, velocity: 0 },
        Body1d { position: 3, velocity: 0 },
    ];
    step_1d(&mut bodies);
    assert_eq!(bodies[3].velocity, -1);
    assert_eq!(bodies[3].position, 2);
    step_1d(&mut bodies);
    assert_eq!(bodies[3].velocity, -1);
    assert_eq!(bodies[3].position, 1);
}
