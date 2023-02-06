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
        
        let mut monkey_counter = 0;


        for line in reader.lines() {
            let mut line_data = String::new();
            match line {
                Ok(v) => line_data = v,
                Err(e) => println!("{e}")
            }

            // Parse data here
            // if line.contains("Monkey") {


            // }

        }
    }

    fn part1(&self) {
        println!("Part1: Not completed");
    }

    fn part2(&self) {
        println!("Part2: Not completed");
    }
}


// needs a parser
enum Instruction {
    ADD,
    MULTIPLY
}

// needs a parser
struct Operation {
    instruction: Instruction,
    val: i32
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
