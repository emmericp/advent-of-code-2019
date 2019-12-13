use std::fs;
use advent_of_code::intcode::IntCodeCpu;
use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("./input/day13.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    part1(&mut cpu.clone());
    part2(&mut cpu.clone());
}

fn part1(cpu: &mut IntCodeCpu) {
    cpu.run();
    dbg!(cpu.output.iter().tuples().filter(|(_, _, tile)| **tile == 2).count());
}

fn part2(cpu: &mut IntCodeCpu) {
    cpu.memory[0] = 2;
    let mut paddle_pos = 0;
    let mut score = 0;
    loop {
        let x = cpu.run_until_out();
        if x.is_none() {
            break;
        }
        let x = x.unwrap();
        let y = cpu.run_until_out().unwrap();
        let tile = cpu.run_until_out().unwrap().into();
        match tile {
            TileType::Paddle => {
                paddle_pos = x;
            },
            TileType::Ball => {
                match x.cmp(&paddle_pos) {
                    Ordering::Less => cpu.input.push_back(-1),
                    Ordering::Equal => cpu.input.push_back(0),
                    Ordering::Greater => cpu.input.push_back(1),
                }
            },
            TileType::Unknown { raw } => {
                if x == -1 && y == 0 {
                    score = raw;
                }
            }
            _ => {}
        }
    }
    dbg!(score);
}

#[derive(Debug)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Unknown { raw: i64 },
}

impl Into<TileType> for i64 {
    fn into(self) -> TileType {
        match self {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => TileType::Unknown { raw: self },
        }
    }
}
