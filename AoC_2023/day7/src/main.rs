use std::{fs, io::{self, BufRead}};

fn parse(file_name: &str) {

    let file = fs::File::open(file_name).expect("No file found");
    let reader = io::BufReader::new(file);
    
    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }
    }

}

fn part1() {

}

fn part2() {

}


fn main() {
    println!("Hello, world!");
}
