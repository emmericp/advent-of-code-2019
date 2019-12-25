use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCodeCpu {
    ip: usize,
    rbp: usize,
    pub running: bool,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
    pub memory: Vec<i64>,
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

enum Instruction {
    Add { src1: i64, src2: i64, dst: i64 },
    Mul { src1: i64, src2: i64, dst: i64 },
    In { dst: i64 },
    Out { src: i64 },
    JumpNotZero { cond: i64, target: i64 },
    JumpZero { cond: i64, target: i64 },
    LessThan { src1: i64, src2: i64, dst: i64 },
    Equals { src1: i64, src2: i64, dst: i64 },
    AdjustRbp { src: i64 },
    Halt,
}

impl IntCodeCpu {
    pub fn from_code(code: &str) -> IntCodeCpu {
        IntCodeCpu {
            ip: 0,
            rbp: 0,
            running: true,
            input: VecDeque::new(),
            output: VecDeque::new(),
            memory: code.split(',').map(|e| e.trim().parse::<i64>().unwrap()).collect(),
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }

    pub fn run_until_io(&mut self) {
        while self.running {
            match self.step() {
                Instruction::In { .. } => break,
                Instruction::Out { .. } => break,
                _ => {}
            }
        }
    }

    pub fn run_until_out(&mut self) -> Option<i64> {
        while self.running {
            self.step();
            if let Some(output) = self.output.pop_front() {
                return Some(output);
            }
        }
        None
    }

    pub fn input_ascii(&mut self, ascii: &str) {
        ascii.chars().for_each(|c| self.input.push_back(c as i64));
        while !self.input.is_empty() {
            self.run_until_io()
        }
    }

    pub fn read_ascii_line(&mut self) -> Option<String> {
        let mut result = String::new();
        loop {
            self.run_until_io();
            match self.output.pop_front() {
                None => return None,
                Some(c) => {
                    let c = c as u8 as char;
                    if c == '\n' {
                        break;
                    } else {
                        result.push(c);
                    }
                },
            }
        }
        Some(result)
    }

    fn fetch_and_resize_memory(&mut self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr]
    }

    fn store_and_resize_memory(&mut self, addr: usize, val: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = val;
    }

    fn fetch_dst_address(&self, mode: ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => immediate,
            ParameterMode::Immediate => panic!("dst operand cannot use immediate mode"),
            ParameterMode::Relative => self.rbp as i64 + immediate,
        }
    }

    fn fetch_operand(&mut self, mode: ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => self.fetch_and_resize_memory(immediate as usize),
            ParameterMode::Immediate => immediate,
            ParameterMode::Relative => self.fetch_and_resize_memory((self.rbp as i64 + immediate) as usize),
        }
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        let inst = self.memory[self.ip];
        let opcode = inst % 100;
        let mode1 = match inst / 100 % 10 {
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Position
        };
        let mode2 = match inst / 1_000 % 10 {
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Position
        };
        let mode3 = match inst / 10_000 % 10 {
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Position
        };
        match opcode {
            1 => Instruction::Add {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(mode3, self.memory[self.ip + 3]),
            },
            2 => Instruction::Mul {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(mode3, self.memory[self.ip + 3]),
            },
            3 => Instruction::In {
                dst: self.fetch_dst_address(mode1, self.memory[self.ip + 1]),
            },
            4 => Instruction::Out {
                src: self.fetch_operand(mode1, self.memory[self.ip + 1]),
            },
            5 => Instruction::JumpNotZero {
                cond: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                target: self.fetch_operand(mode2, self.memory[self.ip + 2]),
            },
            6 => Instruction::JumpZero {
                cond: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                target: self.fetch_operand(mode2, self.memory[self.ip + 2]),
            },
            7 => Instruction::LessThan {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(mode3, self.memory[self.ip + 3]),
            },
            8 => Instruction::Equals {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(mode3, self.memory[self.ip + 3]),
            },
            9 => Instruction::AdjustRbp {
                src: self.fetch_operand(mode1, self.memory[self.ip + 1])
            },
            99 => Instruction::Halt,
            _ => panic!("bad opcode {}", opcode),
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Add { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, src1 + src2);
                self.ip += 4;
            }
            Instruction::Mul { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, src1 * src2);
                self.ip += 4;
            }
            Instruction::In { dst } => {
                let src = self.input.pop_front().unwrap_or(-1);
                self.store_and_resize_memory(*dst as usize, src);
                self.ip += 2;
            }
            Instruction::Out { src } => {
                self.output.push_back(*src);
                self.ip += 2;
            }
            Instruction::JumpNotZero { cond, target } => {
                if *cond != 0 {
                    self.ip = *target as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::JumpZero { cond, target } => {
                if *cond == 0 {
                    self.ip = *target as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::LessThan { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, if *src1 < *src2 { 1 } else { 0 });
                self.ip += 4;
            }
            Instruction::Equals { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, if *src1 == *src2 { 1 } else { 0 });
                self.ip += 4;
            }
            Instruction::AdjustRbp { src } => {
                self.rbp = ((self.rbp as i64) + *src) as usize;
                self.ip += 2;
            }
            Instruction::Halt => {
                self.running = false;
            }
        }
    }

    fn step(&mut self) -> Instruction {
        let inst = self.fetch_and_decode();
        self.execute(&inst);
        inst
    }
}

#[test]
fn test_step_add_mul() {
    let mut cpu = IntCodeCpu::from_code("1,4,5,6,10,20,0");
    cpu.step();
    assert_eq!(cpu.ip, 4);
    assert_eq!(cpu.memory, vec![1, 4, 5, 6, 10, 20, 30]);
    assert!(cpu.running);
    cpu.ip = 0;
    cpu.memory[0] = 2;
    cpu.step();
    assert_eq!(cpu.ip, 4);
    assert_eq!(cpu.memory, vec![2, 4, 5, 6, 10, 20, 200]);
    assert!(cpu.running);
}

#[test]
fn test_run() {
    let mut cpu = IntCodeCpu::from_code("1,9,10,3,2,3,11,0,99,30,40,50");
    cpu.run();
    assert!(!cpu.running);
    assert_eq!(cpu.memory, vec![3500, 9, 10, 70,
                                2, 3, 11, 0,
                                99,
                                30, 40, 50])
}

#[test]
fn test_io() {
    let mut cpu = IntCodeCpu::from_code("3,0,3,1,4,0,4,1,99");
    cpu.input.push_back(1234);
    cpu.input.push_back(5678);
    cpu.run();
    assert_eq!(cpu.output.pop_front(), Some(1234));
    assert_eq!(cpu.output.pop_front(), Some(5678));
}

#[test]
fn test_parameter_modes() {
    let mut cpu = IntCodeCpu::from_code("1002,4,3,4,33");
    cpu.run();
    assert_eq!(cpu.memory[4], 99);
    cpu = IntCodeCpu::from_code("1101,100,-1,4,0");
    cpu.run();
    assert_eq!(cpu.memory[4], 99);
}

#[test]
fn test_conditions() {
    fn helper(code: &str, true_example: i64, false_example: i64) {
        let mut cpu = IntCodeCpu::from_code(code);
        cpu.input.push_back(true_example);
        cpu.run();
        assert_eq!(cpu.output.pop_front(), Some(1));

        let mut cpu = IntCodeCpu::from_code(code);
        cpu.input.push_back(false_example);
        cpu.run();
        assert_eq!(cpu.output.pop_front(), Some(0));
    }

    helper("3,9,8,9,10,9,4,9,99,-1,8", 8, 7);
    helper("3,3,1108,-1,8,3,4,3,99", 8, 7);
    helper("3,9,7,9,10,9,4,9,99,-1,8", 7, 8);
    helper("3,3,1107,-1,8,3,4,3,99", 7, 8);
}


#[test]
fn test_resizing() {
    // quine from day 9
    let mut cpu = IntCodeCpu::from_code("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    cpu.run();
    assert_eq!(cpu.output.len(), 16);
    assert_eq!(cpu.output.pop_front(), Some(109));
    assert_eq!(cpu.output.pop_front(), Some(1));
}

#[test]
fn test_large_numbers() {
    let mut cpu = IntCodeCpu::from_code("1102,34915192,34915192,7,4,7,99,0");
    cpu.run();
    assert_eq!(cpu.output.pop_front(), Some(34_915_192 * 34_915_192));
}

