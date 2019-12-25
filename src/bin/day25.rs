use std::fs;
use advent_of_code::intcode::IntCodeCpu;
#[allow(unused_imports)]
use std::io::stdin;

fn main() {
    let input = fs::read_to_string("./input/day25.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);
    part1(&mut cpu.clone());
}

fn run_until_command(cpu: &mut IntCodeCpu) {
    while let Some(out) = cpu.read_ascii_line() {
        println!("{}", out);
        if &out == "Command?" {
            break;
        }
    }
}

fn take_all_items(cpu: &mut IntCodeCpu) {
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("take tambourine\n");
    run_until_command(cpu);
    cpu.input_ascii("east\n");
    run_until_command(cpu);
    cpu.input_ascii("take astrolabe\n");
    run_until_command(cpu);
    cpu.input_ascii("east\n");
    run_until_command(cpu);
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("take klein bottle\n");
    run_until_command(cpu);
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("take easter egg\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
    cpu.input_ascii("west\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
    cpu.input_ascii("take shell\n");
    run_until_command(cpu);
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("west\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
    cpu.input_ascii("take hypercube\n");
    run_until_command(cpu);
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("west\n");
    run_until_command(cpu);
    cpu.input_ascii("take dark matter\n");
    run_until_command(cpu);
    cpu.input_ascii("west\n");
    run_until_command(cpu);
    cpu.input_ascii("north\n");
    run_until_command(cpu);
    cpu.input_ascii("west\n");
    run_until_command(cpu);
    cpu.input_ascii("take coin\n");
    run_until_command(cpu);
    cpu.input_ascii("south\n");
    run_until_command(cpu);
}

fn part1(cpu: &mut IntCodeCpu) {
    take_all_items(cpu);
    let items = [
        "hypercube",
        "coin",
        "klein bottle",
        "shell",
        "easter egg",
        "astrolabe",
        "tambourine",
        "dark matter"
    ];
    for combination in 0..(1 << items.len()) {
        let mut cpu = cpu.clone();
        for (i, item) in items.iter().enumerate() {
            if combination & (1 << i) == 0 {
                cpu.input_ascii("drop ");
                cpu.input_ascii(item);
                cpu.input_ascii("\n");
                run_until_command(&mut cpu);
            }
        }
        cpu.input_ascii("south\n");
        run_until_command(&mut cpu);
        while let Some(out) = cpu.read_ascii_line() {
            println!("{}", out);
            if out.contains("keypad") {
                return;
            }
        }
    }
    // interactive mode
    /*
    loop {
        run_until_command(cpu);
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).unwrap();
        cpu.input_ascii(&cmd);
    }
    */
}
