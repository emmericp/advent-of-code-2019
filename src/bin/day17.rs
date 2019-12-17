use std::fs;
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let input = fs::read_to_string("./input/day17.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    part1(&mut cpu.clone());
    part2(&mut cpu.clone());
}

fn part1(cpu: &mut IntCodeCpu) {
    let mut maze: Vec<Vec<bool>> = vec![];
    let mut line = vec![];
    let mut crossings = vec![];
    while let Some(output) = cpu.run_until_out() {
        print!("{}", String::from_utf8(vec![output as u8]).unwrap());
        match output as u8 {
            b'.' => {
                line.push(false);
            }
            b'#' | b'^' => {
                line.push(true);
                if line.len() >= 2 && maze.len() >= 3 {
                    let prev_line = &maze[maze.len() - 2];
                    let prev_prev_line = &maze[maze.len() - 3];
                    if prev_line[line.len() - 2]
                        && prev_line[line.len() - 1]
                        && *prev_line.get(line.len()).unwrap_or(&false)
                        && prev_prev_line[line.len() - 1] {
                        crossings.push((line.len() - 1, maze.len() - 2))
                    }
                }
            }
            b'\n' => {
                maze.push(line);
                line = vec![];
            }
            _ => panic!("unknown char {}", output)
        }
    }
    dbg!(crossings.iter().map(|(x, y)| x * y).sum::<usize>());
}

fn input_ascii(cpu: &mut IntCodeCpu, ascii: &str) {
    ascii.chars().for_each(|c| cpu.input.push_back(c as i64));
}

fn part2(cpu: &mut IntCodeCpu) {
    // solved on paper
    cpu.memory[0] = 2;
    input_ascii(cpu, "A,B,A,B,A,C,B,C,A,C\n");
    input_ascii(cpu, "L,6,R,12,L,6\n");
    input_ascii(cpu, "R,12,L,10,L,4,L,6\n");
    input_ascii(cpu, "L,10,L,10,L,4,L,6\n");
    input_ascii(cpu, "n\n");
    cpu.run();
    dbg!(&cpu.output.pop_back());
}
