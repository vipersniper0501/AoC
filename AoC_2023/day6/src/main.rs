use std::{fs, io::{self, BufRead}};

#[derive(Debug)]
struct Race {
    time: i32,
    record: i32
}

fn parse(file_name: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let file = fs::File::open(file_name)
        .expect("No file found");
    let reader = io::BufReader::new(file);

    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();

    let mut distances_flag: bool = false;

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        let mut values: Vec<&str> = line_data.split_whitespace().collect();
        values.remove(0);

        let int_values: Vec<i32> = match values.iter().map(|s| s.parse::<i32>()).collect() {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to parse value {e}");
                vec![]
            }

        };

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
    let mut margin_of_error: i32 = 1;

    for race in races {
        let time: i32 = race.time;
        let record: i32 = race.record;

        let mut possible_times: i32 = 0;

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

fn part2() {

}


fn main() {
    let races: Vec<Race> = parse("data/input");
    part1(&races);

}
