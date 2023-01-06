use std::{fs, io::{self, BufRead}};

struct Day8 {
    map: Vec<Vec<i32>>,

}

impl Day8 {

    fn new() -> Self {
        Self {
            map: Vec::new(),
        }
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

            self.map.push(line_data.chars().map(|element| 
                    element.to_string().parse::<i32>().unwrap()).collect());
        }

        println!("{:?}", self.map);
        
    }

    fn part1(&self) {

        println!("Part1: Unsolved");
    }

    fn part2(&self) {

        println!("Part2: Unsolved");
    }
}


fn main() {
    let mut day = Day8::new();
    day.parse();
    day.part1();
    day.part2();
}
