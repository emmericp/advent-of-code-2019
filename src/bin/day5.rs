use std::fs::File;
use std::io::{BufReader, BufRead};
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let file = File::open("./input/day5.txt").unwrap();
    let mut code = String::new();
    BufReader::new(file).read_line(&mut code).ok();
    let mut cpu = IntCodeCpu::from_code(&code);
    cpu.input = Some(5);
    cpu.run();
    dbg!(cpu.output);
}
