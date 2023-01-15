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
            // let (instr, para) = line_data.split(' ').collect();
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
        println!("Part1: Unsolved");
    }

    fn part2(&self) {
        println!("Part2: Unsolved");
    }
}


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

struct Computer {
    x_reg: i32,
    cycle: i32
}

impl Computer {
    fn incr_cycle(&mut self) {
        self.cycle += 1;
    }

    fn execute_instruction(&mut self) {

    }
}


fn main() {
    let mut day = Day10::new();
    day.parse();
    day.part1();
    day.part2();
}
