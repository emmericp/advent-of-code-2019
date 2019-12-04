use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("./input/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result1 = 0i64;
    let mut result2 = 0i64;
    for line in reader.lines() {
        let mass = line.unwrap().parse::<i64>().unwrap();
        let fuel = fuel_for_mass(mass);
        result1 += fuel;
        result2 += fuel_for_fuel(fuel);
    }
    dbg!(result1);
    dbg!(result1 + result2);
}

fn fuel_for_mass(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

fn fuel_for_fuel(mass: i64) -> i64 {
    let fuel = fuel_for_mass(mass);
    if fuel == 0 {
        return 0;
    }
    fuel_for_fuel(fuel) + fuel
}

#[test]
fn test_part1() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(1969), 654);
}

#[test]
fn test_part2() {
    assert_eq!(fuel_for_fuel(2), 0);
    assert_eq!(fuel_for_fuel(654), 966 - 654);
}
