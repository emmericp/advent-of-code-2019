use std::fs;
use advent_of_code::intcode::IntCodeCpu;

fn main() {
    let input = fs::read_to_string("./input/day23.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    solve(&cpu);
}

fn solve(cpu: &IntCodeCpu) {
    let mut cpus = (0..50).map(|i| {
        let mut cpu = cpu.clone();
        cpu.input.push_back(i);
        cpu
    }).collect::<Vec<IntCodeCpu>>();
    let mut nat = (-1, -1);
    let mut nat_last_y = 0;
    let mut is_first_nat_packet = true;
    let mut did_send = [false; 50];
    let mut did_receive = [false; 50];
    loop {
        for i in 0..50 {
            cpus[i].run_until_io();
            if let Some(dst) = cpus[i].output.pop_front() {
                did_send[i] = true;
                let x = cpus[i].run_until_out().unwrap();
                let y = cpus[i].run_until_out().unwrap();
                if dst == 255 {
                    nat = (x, y);
                    if is_first_nat_packet {
                        dbg!(y); // part 1
                        is_first_nat_packet = false;
                    }
                } else {
                    cpus[dst as usize].input.push_back(x);
                    cpus[dst as usize].input.push_back(y);
                }
            } else if !cpus[i].input.is_empty() {
                did_receive[i] = true;
                cpus[i].run_until_io();
            }
        }
        if did_send.iter().all(|e| !*e) && did_receive.iter().all(|e| !*e) {
            cpus[0].input.push_back(nat.0);
            cpus[0].input.push_back(nat.1);
            if nat.1 == nat_last_y {
                dbg!(nat_last_y); // part 2
                return;
            } else {
                nat_last_y = nat.1;
            }
        }
        did_send.iter_mut().for_each(|e| *e = false);
        did_receive.iter_mut().for_each(|e| *e = false);
    }
}
