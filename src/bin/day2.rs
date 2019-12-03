use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("./input/day2.txt").unwrap();
    let mut code = String::new();
    BufReader::new(file).read_line(&mut code).ok();
    let memory: Vec<i64> = code.split(",").map(|e| e.trim().parse::<i64>().unwrap()).collect();
    for noun in 0..99 {
        for verb in 0..99 {
            let mut copy = memory.clone();
            copy[1] = noun;
            copy[2] = verb;
            let mut state = IntCodeState {
                ip: 0,
                memory: &mut copy,
            };
            state.run();
            if copy[0] == 19690720 {
                dbg!(noun * 100 + verb);
            }
        }
    }
}

struct IntCodeState<'a> {
    ip: usize,
    memory: &'a mut Vec<i64>,
}

impl IntCodeState<'_> {
    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        let opcode = self.memory.get(self.ip);
        let imm1 = self.memory[self.ip + 1];
        let imm2 = self.memory[self.ip + 2];
        let imm3 = self.memory[self.ip + 3];
        match opcode {
            Some(1) => {
                self.memory[imm3 as usize] = self.memory[imm1 as usize] + self.memory[imm2 as usize];
                self.ip += 4;
                true
            }
            Some(2) => {
                self.memory[imm3 as usize] = self.memory[imm1 as usize] * self.memory[imm2 as usize];
                self.ip += 4;
                true
            }
            Some(99) => {
                false
            }
            _ => panic!("bad opcode")
        }
    }
}

#[test]
fn test_step() {
    let mut state = IntCodeState {
        ip: 0,
        memory: &mut vec![1, 4, 5, 6, 10, 20, 0],
    };
    assert!(state.step());
    assert_eq!(state.ip, 4);
    assert_eq!(state.memory, vec![1, 4, 5, 6, 10, 20, 30]);
    state.ip = 0;
    state.memory[0] = 2;
    assert!(state.step());
    assert_eq!(state.ip, 4);
    assert_eq!(state.memory, vec![2, 4, 5, 6, 10, 20, 200]);
}

#[test]
fn test_run() {
    let mut state = IntCodeState {
        ip: 0,
        memory: &mut vec![1, 9, 10, 3,
                          2, 3, 11, 0,
                          99,
                          30, 40, 50],
    };
    state.run();
    assert_eq!(state.memory, vec![3500, 9, 10, 70,
                                  2, 3, 11, 0,
                                  99,
                                  30, 40, 50])
}
