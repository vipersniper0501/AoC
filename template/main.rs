use std::{fs, io::{self, BufRead}};

#[derive(Default)]
struct DayX {
}

impl DayX {

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

            // parse data here

        }
    }

    fn part1(&self) {
        println!("Part1: Not completed");
    }

    fn part2(&self) {
        println!("Part2: Not completed");
    }
}


fn main() {
    let mut day = DayX::new();
    day.parse();
    day.part1();
    day.part2();
}
