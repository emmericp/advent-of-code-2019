use std::fs;
use advent_of_code::intcode::IntCodeCpu;
use std::collections::VecDeque;

fn main() {
    let input = fs::read_to_string("./input/day15.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    solve(&cpu.clone());
}

fn solve(cpu: &IntCodeCpu) {
    let mut todo = VecDeque::new();
    todo.push_back((cpu.clone(), 0, 0));
    while let Some((cpu, steps, came_from)) = todo.pop_front() {
        for direction in 1..=4 {
            if direction != came_from {
                let mut clone = cpu.clone();
                clone.input.push_back(direction);
                let result = clone.run_until_out().unwrap();
                if result == 1 {
                    let came_from = match direction {
                        1 => 2,
                        2 => 1,
                        3 => 4,
                        4 => 3,
                        _ => panic!("bad direction")
                    };
                    todo.push_back((clone, steps + 1, came_from));
                } else if result == 2 {
                    dbg!(steps + 1);
                    part2(&clone);
                    break;
                }
            }
        }
    }
}

fn part2(cpu: &IntCodeCpu) {
    let mut todo = VecDeque::new();
    todo.push_back((cpu.clone(), 0, 0));
    let mut max_steps = 0;
    while let Some((cpu, steps, came_from)) = todo.pop_front() {
        for direction in 1..=4 {
            if direction != came_from {
                let mut clone = cpu.clone();
                clone.input.push_back(direction);
                let result = clone.run_until_out().unwrap();
                if result == 1 {
                    let came_from = match direction {
                        1 => 2,
                        2 => 1,
                        3 => 4,
                        4 => 3,
                        _ => panic!("bad direction")
                    };
                    todo.push_back((clone, steps + 1, came_from));
                    if steps + 1 > max_steps {
                        max_steps = steps + 1;
                    }
                }
            }
        }
    }
    dbg!(max_steps);
}
