use std::fs;
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let input = fs::read_to_string("./input/day19.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    part1(&cpu.clone());
    part2(&cpu.clone());
}

fn is_in_beam(cpu: &IntCodeCpu, x: i64, y: i64) -> bool {
    let mut cpu = cpu.clone();
    cpu.input.push_back(x);
    cpu.input.push_back(y);
    cpu.run();
    cpu.output.pop_front() == Some(1)
}


fn part1(cpu: &IntCodeCpu) {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if is_in_beam(cpu, x, y) {
                print!("#");
                count += 1;
            } else {
                print!(".");
            }
        }
        println!(" ");
    }
    dbg!(count);
}

fn part2(cpu: &IntCodeCpu) {
    let mut beam_start = 0;
    let mut y = 10; // beam has "holes" near the origin (which aren't handled below)
    loop {
        let mut found_beam = false;
        let mut x = beam_start;
        loop {
            if !is_in_beam(cpu, x, y) {
                if found_beam {
                    break;
                }
            } else {
                if !found_beam {
                    found_beam = true;
                    beam_start = x;
                }
                if is_in_beam(cpu, x + 99, y + 99)
                    && is_in_beam(cpu, x + 99, y)
                    && is_in_beam(cpu, x, y + 99)
                {
                    dbg!(x * 10_000 + y);
                    return;
                }
            }
            x += 1;
        }
        y += 1;
    }
}
