use std::{fs, io::{self, BufRead}};

#[derive(Default)]
struct Day10 {
    commands: Vec<(Instruction, i32)>,
}

impl Day10 {

    fn new() -> Self {
        Self::default()
    }

    fn parse(&mut self) {
        let file = fs::File::open("data/input")
            .expect("No File Found");

        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            let mut line_data = String::new();
            match line {
                Ok(v) => line_data = v,
                Err(e) => println!("{e}")
            }
            if line_data.contains(' ') {
                let (instr, para) = line_data.split_once(' ').unwrap();
                let instr = Instruction::parse(instr);
                if instr == Instruction::ADDX {
                    let para: i32 = para.parse().unwrap();
                    self.commands.push((instr, para));
                }
            } else {
                let instr = Instruction::parse(line_data.as_str());
                if instr == Instruction::NOOP {
                    self.commands.push((instr, 0));
                }
            }
        }
    }

    fn part1(&self) {
        let mut comp = Computer::new();
        for (instr, val) in &self.commands {
            comp.execute_instruction(instr, *val)
        }
        println!("Part1: Signal Strength: {}", comp.signal_strengths);
    }

    fn part2(&self) {
        println!("Part2: Unsolved");
    }
}


/// Different types of Instructions possible for CRT
///
/// * `NOOP`: No operation instruction. Runs for one cycle and does nothing.
/// * `ADDX`: Add instruction. Runs for two cycles. First cycle does nothing.
///           At the end of the second cycle it adds a value (can be negative or
///           positive) to the X register of the computer.
#[derive(Debug,PartialEq)]
enum Instruction {
    NOOP,
    ADDX
}

impl Instruction {
    fn parse(s: &str) -> Self {
        match s {
            "noop" => Self::NOOP,
            "addx" => Self::ADDX,
            _ => panic!("invalid instruction '{s}'")
        }
    }
}

/// Structure that represents a computer
///
/// * `x_reg`: X Register
/// * `cycle`: Clock Circuit Cycle Counter
/// * `signal_strengths`: Cumulative signal strength
struct Computer {
    x_reg: i32,
    cycle: i32,
    signal_strengths: i32
}

impl Computer {

    fn new() -> Self {
        Self {
            x_reg: 1,
            cycle: 0,
            signal_strengths: 0
        }
    }

    /// Increments the computers clock circuit (cycle counter)
    fn incr_cycle(&mut self) {
        self.cycle += 1;
    }

    /// Executes the instruction provided by the CRT program
    ///
    /// * `instr`: Instruction to completed
    /// * `val`: Value associated with the instruction
    fn execute_instruction(&mut self, instr: &Instruction, val: i32) {
        match instr {
            Instruction::NOOP => {
                self.incr_cycle();
                self.inspect_values();
            },
            Instruction::ADDX => {
                self.incr_cycle();
                self.inspect_values();
                self.incr_cycle();
                self.inspect_values();
                self.x_reg += val;
            }
        }
    }

    /// Inspect values to determine signal strength
    fn inspect_values(&mut self) {
        if self.cycle == 20 {
            self.signal_strengths += self.x_reg * self.cycle;
        } else if (self.cycle - 20) % 40 == 0 {
            self.signal_strengths += self.x_reg * self.cycle;
        }
    }
}


fn main() {
    let mut day = Day10::new();
    day.parse();
    day.part1();
    day.part2();
}
