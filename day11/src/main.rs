use std::{fs, io::{self, BufRead}};

#[derive(Default)]
struct Day11 {
    monkeys: Vec<Monkey>
}

impl Day11 {

    fn new() -> Self {
        Self::default()
    }

    fn parse(&mut self) {
        let file = fs::File::open("data/input")
            .expect("No File Found");

        let reader = io::BufReader::new(file);
        
        let mut monkey_counter = -1;
        let mut items: Vec<i32> = Vec::new();
        let mut ops = Operation::new();

        for line in reader.lines() {
            let mut line_data = String::new();
            match line {
                Ok(v) => line_data = v,
                Err(e) => println!("{e}")
            }

            // Parse data here
            if line_data.contains("Monkey") {
                // create previous monkey here
                monkey_counter += 1;
            }


            if line_data.contains("Starting items:") {
                // split this off into seperate function
                items = Vec::new();
                let temp_items = line_data.split(": ").collect::<Vec<&str>>();
                for i in temp_items[1].split(", ").collect::<Vec<&str>>() {
                    items.push(i.parse().unwrap());
                }
            }


            if line_data.contains("Operation") {
                // split this off into seperate function
                ops = Operation::new();
                let temp: Vec<&str> = line_data
                    .split("= old ")
                    .collect::<Vec<&str>>();

                let temp: Vec<&str> = temp[1]
                    .split(' ')
                    .collect::<Vec<&str>>();

                let instr: Instruction = Instruction::parse(temp[0]);
                let mut value: i32 = 0;

                if temp[1] == "old" {
                    value = -1;
                }
                else {
                    value = temp[1].parse().unwrap();
                }

                ops.instruction = instr;
                ops.val = value;
            }

            println!("Current Monkey: id={}, items={:?}, ops={:#?}", monkey_counter, items, ops);
        }
        //create last monkey here.

    }

    fn part1(&self) {
        println!("Part1: Not completed");
    }

    fn part2(&self) {
        println!("Part2: Not completed");
    }
}


#[derive(Debug)]
enum Instruction {
    NONE,
    ADD,
    MULTIPLY
}

impl Instruction {
    fn parse(s: &str) -> Self {
        match s {
            "*" => return Self::MULTIPLY,
            "+" => return Self::ADD,
            _ => panic!("invalid instruction '{s}'")
        }
    }
}

#[derive(Debug)]
struct Operation {
    instruction: Instruction,
    val: i32
}

impl Operation {
    fn new() -> Self {
        Self {
            instruction: Instruction::NONE,
            val: 0
        }
    }
}


/// Structure representing a monkey
///
/// * `id`: An id for each monkey
/// * `items`: A vector of items that each monkey is holding
/// * `operation`: The operation to be completed
/// * `test_value`: Value to test which monkey the item to be thrown to
/// * `t_monkey`: The monkey's id that should be thrown to if the test value is true
/// * `f_monkey`: The monkey's id that should be thrown to if the test value is false
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    test_value: i32,
    t_monkey: i32,
    f_monkey: i32
}


impl Monkey {
    fn new(a_id: i32, a_items: Vec<i32>, a_operation: Operation, a_test_value: i32,
        a_t_monkey: i32, a_f_monkey: i32) -> Self {
        Self { 
            id: a_id, 
            items: a_items,
            operation: a_operation,
            test_value: a_test_value,
            t_monkey: a_t_monkey,
            f_monkey: a_f_monkey
        }
    }
}

fn main() {
    let mut day = Day11::new();
    day.parse();
    day.part1();
    day.part2();
}
