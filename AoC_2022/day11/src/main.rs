use std::{fs, io::{self, BufRead}};

pub mod monkey;

use monkey::*;

#[derive(Default)]
struct Day11 {
    monkeys: Vec<Monkey>
}

impl Day11 {

    fn new() -> Self {
        Self::default()
    }

    fn monkey_throw(&mut self, src_monkey: &mut monkey::Monkey,
        dest_monkey_id: i32) {
        let item = src_monkey.items.remove(0);
        self.monkeys[dest_monkey_id as usize].items.push(item);

    }

    fn parse(&mut self) {
        let file = fs::File::open("data/input")
            .expect("No File Found");

        let reader = io::BufReader::new(file);
        
        let mut monkey_counter = -1;
        let mut a_items: Vec<i32> = Vec::new();
        let mut ops = Operation::new();
        let mut divisible_by = 1;
        let mut true_monkey = monkey_counter;
        let mut false_monkey = monkey_counter;

        for line in reader.lines() {
            let mut line_data = String::new();
            match line {
                Ok(v) => line_data = v,
                Err(e) => println!("{e}")
            }

            // Parse data here
            if line_data.contains("Monkey") {
                // create previous monkey here
                if monkey_counter >= 0 {
                    let new_monkey: Monkey = Monkey::new(monkey_counter,
                        a_items,
                        ops,
                        divisible_by,
                        true_monkey,
                        false_monkey
                        );
                    self.monkeys.push(new_monkey);
                }
                // Now increment monkey and reset values
                monkey_counter += 1;
                a_items = Vec::new();
                ops = Operation::new();
                divisible_by = 1;
                true_monkey = monkey_counter;
                false_monkey = monkey_counter;
            }


            if line_data.contains("Starting items:") {
                // split this off into seperate function
                a_items = Vec::new();
                let temp_items = line_data.split(": ").collect::<Vec<&str>>();
                for i in temp_items[1].split(", ").collect::<Vec<&str>>() {
                    a_items.push(i.parse().unwrap());
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
                let mut _value: i32 = 0;

                if temp[1] == "old" {
                    _value = -1;
                }
                else {
                    _value = temp[1].parse().unwrap();
                }

                ops.instruction = instr;
                ops.val = _value;
            }

            if line_data.contains("Test") {
                divisible_by = line_data.split(" by ").collect::<Vec<&str>>()[1]
                    .parse().unwrap();
            }

            if line_data.contains("true") {
                true_monkey = line_data
                    .split(" monkey ")
                    .collect::<Vec<&str>>()[1]
                    .parse().unwrap();

            }

            if line_data.contains("false") {
                false_monkey = line_data
                    .split(" monkey ")
                    .collect::<Vec<&str>>()[1]
                    .parse().unwrap();
            }

        }
        //create last monkey here.
        let new_monkey: Monkey = Monkey::new(monkey_counter,
            a_items,
            ops,
            divisible_by,
            true_monkey,
            false_monkey
            );
        self.monkeys.push(new_monkey);

        // for m in &self.monkeys {
            // println!("{:?}", m);
        // }

    }

    fn part1(&mut self) {
        println!("Part1: Not completed");
        println!("{:?}", self.monkeys[0]);
        println!("{:?}", self.monkeys[1]);
        let inspection = self.monkeys[0].inspect();
        let mut m = self.monkeys.get_mut(0).unwrap();
        self.monkey_throw(&mut m, inspection);
        // self.monkeys[0].throw(self.monkeys[0].inspect(), &mut self.monkeys);

    }

    fn part2(&self) {
        println!("Part2: Not completed");
    }
}





fn main() {
    let mut day = Day11::new();
    day.parse();
    day.part1();
    day.part2();
}
