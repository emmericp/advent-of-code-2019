use std::fs;
use advent_of_code::intcode::IntCodeCpu;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = fs::read_to_string("./input/day21.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    // part 1 can be brute forced in a few seconds
    brute_force_search(&cpu, "WALK", &["A", "B", "C", "D", "T", "J"]);
    print_cpu_result(&cpu, "WALK", &["OR A T", "AND C T", "NOT T J", "AND D J"]);
    // part 2 can't be brute-forced in reasonable time :(
    print_cpu_result(&cpu, "RUN", &["OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "OR E T", "OR H T", "AND T J"]);
}

fn print_cpu_result(cpu: &IntCodeCpu, mode: &str, inst: &[&str]) {
    let mut cpu = cpu.clone();
    for inst in inst {
        cpu.input_ascii(inst);
        cpu.input_ascii("\n");
    }
    cpu.input_ascii(mode);
    cpu.input_ascii("\n");
    cpu.run();
    if let Some(result) = cpu.output.iter().find(|o| **o > 255) {
        dbg!(result);
    } else {
        for c in cpu.output.iter() {
            print!("{}", String::from_utf8(vec![*c as u8]).unwrap());
        }
    }
}

fn brute_force_search(cpu: &IntCodeCpu, mode: &str, reg1: &[&str]) {
    let operations = ["AND", "OR", "NOT"];
    let reg2 = ["T", "J"];
    let instructions: Vec<String> = operations
        .iter()
        .cartesian_product(
            reg1.iter()
        )
        .cartesian_product(
            reg2.iter()
        )
        .map(|((operation, reg1), reg2)| {
            let mut instruction = String::new();
            instruction.push_str(operation);
            instruction.push_str(" ");
            instruction.push_str(reg1);
            instruction.push_str(" ");
            instruction.push_str(reg2);
            instruction
        })
        .collect();
    let result = instructions
        .iter()
        .cartesian_product(
            instructions.iter()
        )
        .cartesian_product(
            instructions.iter()
        )
        .cartesian_product(
            instructions.iter()
        )
        .par_bridge()
        .find_any(|(((op1, op2), op3), op4)| {
            run_cpu(cpu, mode, op1, op2, op3, op4)
        }).unwrap();
    dbg!(result);
}

fn run_cpu(cpu: &IntCodeCpu, mode: &str, op1: &str, op2: &str, op3: &str, op4: &str) -> bool {
    let mut cpu = cpu.clone();
    cpu.input_ascii(op1);
    cpu.input_ascii("\n");
    cpu.input_ascii(op2);
    cpu.input_ascii("\n");
    cpu.input_ascii(op3);
    cpu.input_ascii("\n");
    cpu.input_ascii(op4);
    cpu.input_ascii("\n");
    cpu.input_ascii(mode);
    cpu.input_ascii("\n");
    cpu.run();
    if let Some(result) = cpu.output.iter().find(|o| **o > 255) {
        dbg!(result);
        true
    } else {
        false
    }
}
