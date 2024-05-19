use std::{fs, io::{self, BufRead}};

#[derive(Debug)]
struct Race {
    time: i32,
    record: i32
}

fn parse(file_name: &str) -> Vec<Race> {
    let races: Vec<Race> = Vec::new();

    let file = fs::File::open(file_name)
        .expect("No file found");
    let reader = io::BufReader::new(file);

    let mut line = "";
    reader.read_line(line);

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        let values: Vec<&str> = line_data.split_whitespace().collect();

    }

    return races;
}

fn part1() {

}

fn part2() {

}


fn main() {
    parse("data/input");

}
