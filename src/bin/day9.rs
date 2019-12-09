use std::fs;
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let input = fs::read_to_string("./input/day9.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    part1(&mut cpu.clone());
    part2(&mut cpu.clone());
}

fn part1(cpu: &mut IntCodeCpu) {
    cpu.input.push_back(1);
    cpu.run();
    dbg!(cpu.output.pop_front());
}


fn part2(cpu: &mut IntCodeCpu) {
    cpu.input.push_back(2);
    cpu.run();
    dbg!(cpu.output.pop_front());
}
