use std::{fs, io::{self, BufRead}};

#[derive(Debug)]
struct Race {
    time: i64,
    record: i64 
}

fn parse(file_name: &str, kerning: bool) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let file = fs::File::open(file_name)
        .expect("No file found");
    let reader = io::BufReader::new(file);

    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    let mut distances_flag: bool = false;

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        let mut values: Vec<&str> = line_data.split_whitespace().collect();
        values.remove(0);

        let mut int_values = Vec::new();
        if kerning {
            let value_str: String = values.join("");
            let value =  match value_str.parse::<i64>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Parse Error: {e}");
                    0
                }
            };
            int_values.push(value);
        } else {

            int_values = match values.iter().map(|s| s.parse::<i64>()).collect() {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to parse value {e}");
                    vec![]
                }

            };
        }

        if distances_flag == false {
            times = int_values;
            distances_flag = true;
        } else {
            distances = int_values;
        }
    }

    for i in 0..times.len() {
        let race: Race = Race { time: times[i], record: distances[i] };
        races.push(race);
    }

    return races;
}

fn part1(races: &Vec<Race>) {
    let mut margin_of_error: i64 = 1;

    for race in races {
        let time: i64 = race.time;
        let record: i64 = race.record;

        let mut possible_times: i64 = 0;

        for time_held in 1..time {
            let remaining_time = time - time_held;

            let distance = time_held * remaining_time;

            if distance > record {
                possible_times += 1;
            }
        }

        margin_of_error *= possible_times;

    }

    println!("Part 1: Margin of Error = {margin_of_error}");
}

fn part2(races: &Vec<Race>) {
    let race = match races.get(0) {
        Some(v) => v,
        None => {
            panic!("Missing Race!");
        }
    };

    let time: i64 = race.time;
    let record: i64 = race.record;

    let mut possible_times: i64 = 0;

    for time_held in 1..time {
        let remaining_time = time - time_held;

        let distance = time_held * remaining_time;

        if distance > record {
            possible_times += 1;
        }
    }
    println!("Part 2: Number of ways to win = {possible_times}");
}


fn main() {
    let races: Vec<Race> = parse("data/input", false);
    part1(&races);

    let races: Vec<Race> = parse("data/input", true);
    part2(&races);
}


