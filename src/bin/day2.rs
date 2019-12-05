use std::fs::File;
use std::io::{BufReader, BufRead};
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let file = File::open("./input/day2.txt").unwrap();
    let mut code = String::new();
    BufReader::new(file).read_line(&mut code).ok();
    let cpu = IntCodeCpu::from_code(&code);
    for noun in 0..99 {
        for verb in 0..99 {
            let mut copy = cpu.clone();
            copy.memory[1] = noun;
            copy.memory[2] = verb;
            copy.run();
            if copy.memory[0] == 19_690_720 {
                dbg!(noun * 100 + verb);
            }
        }
    }
}
