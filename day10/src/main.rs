use std::{fs, io::{self, BufRead}, collections::HashSet};

#[derive(Default)]
struct Day10 {
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


        }
    }

    fn part1(&self) {
        println!("Part1: Unsolved");
    }

    fn part2(&self) {
        println!("Part2: Unsolved");
    }
}


fn main() {
    let mut day = Day10::new();
    day.parse();
    day.part1();
    day.part2();
}
