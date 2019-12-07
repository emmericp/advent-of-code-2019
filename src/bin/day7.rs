use std::fs;
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let input = fs::read_to_string("./input/day7.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    dbg!(part1(&cpu));
    dbg!(part2(&cpu));
}

fn part1(cpu: &IntCodeCpu) -> i64 {
    let mut phases = [0, 1, 2, 3, 4];
    let permutations = permutohedron::Heap::new(&mut phases);
    let mut thrust = vec![];
    for phase in permutations {
        let mut out = 0;
        for phase_setting in &phase {
            let mut amp = cpu.clone();
            amp.input.push_back(*phase_setting);
            amp.input.push_back(out);
            amp.run();
            out = amp.output.unwrap();
        }
        thrust.push(out);
    }
    thrust.into_iter().max().unwrap()
}

fn part2(cpu: &IntCodeCpu) -> i64 {
    let mut phases = [5, 6, 7, 8, 9];
    let permutations = permutohedron::Heap::new(&mut phases);
    let mut thrust = vec![];
    for phase in permutations {
        let mut amps: Vec<IntCodeCpu> = phase.iter().map(|phase_setting| {
            let mut amp = cpu.clone();
            amp.input.push_back(*phase_setting);
            amp
        }).collect();
        let mut out = 0;
        while amps.first().unwrap().running {
            for amp in amps.iter_mut() {
                amp.input.push_back(out);
                if let Some(output) = amp.run_until_out() {
                    out = output
                } else {
                    break;
                }
            }
        }
        thrust.push(out);
    }
    thrust.into_iter().max().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&IntCodeCpu::from_code("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")),
        43_210
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&IntCodeCpu::from_code("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                                      27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")),
        139_629_729
    );
}
