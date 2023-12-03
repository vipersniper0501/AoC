use std::{fs, io::{self, BufRead}};

#[derive(Clone, Debug, Default)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32
}

#[derive(Clone, Debug)]
struct Game {
    id: i32,
    rounds: Vec<Cubes>
}

impl Game {
    fn parse(s: &str) -> Self {
        let s2: Vec<&str> = s.split(':').collect();
        let mut extracted_id: i32 = 0;
        match s2[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>() {
            Ok(v) => extracted_id = v,
            Err(_e) => eprintln!("Error parsing int")
        };

        let mut g: Vec<Cubes> = Vec::new();

        let rounds: Vec<&str> = s2[1].split(';').collect();

        for r in rounds {
            let r: Vec<&str> = r.split(',').collect();

            let mut cubes: Cubes = Cubes::default();
            for c in r {
                let mut c = c.to_string();

                match &mut c {
                    c if c.contains("red") => {
                        c.retain(|c| c.is_numeric());
                        match c.parse::<i32>() {
                            Ok(v) => cubes.red = v,
                            Err(_e) => eprintln!("Failed to parse int")
                        }
                    },
                    c if c.contains("green") => {
                        c.retain(|c| c.is_numeric());
                        match c.parse::<i32>() {
                            Ok(v) => cubes.green = v,
                            Err(_e) => eprintln!("Failed to parse int")
                        }
                    }
                    c if c.contains("blue") => {
                        c.retain(|c| c.is_numeric());
                        match c.parse::<i32>() {
                            Ok(v) => cubes.blue = v,
                            Err(_e) => eprintln!("Failed to parse int")
                        }
                    }
                    _ => eprintln!("invalid option")
                }
            }
            g.push(cubes);
        }
        
        return Self { id: extracted_id, rounds: g }
    }
}

fn parse() -> Vec<Game> {

    let file = fs::File::open("data/input")
        .expect("No file foudn");
    let reader = io::BufReader::new(file);

    let mut data: Vec<Game> = Vec::new();

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        let game = Game::parse(&line_data);

        data.push(game);
    }

    return data;
}

fn part1(input: Vec<Game>) {

    let mut sum_of_ids = 0;

    for game in input {
        let mut possible_flag = true;
        for r in game.rounds {
            if r.red > 12 {
                possible_flag = false;
                break;
            }
            if r.green > 13 {
                possible_flag = false;
                break;
            }
            if r.blue > 14 {
                possible_flag = false;
                break;
            }
        }

        if possible_flag {
            sum_of_ids += game.id;
        }
    }

    println!("Part 1: Sum of ids = {}", sum_of_ids);

}

fn part2(input: Vec<Game>) {

    let mut sum_of_powers = 0;

    for game in input {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for r in game.rounds {
            if r.red > min_red {
                min_red = r.red;
            }
            if r.green > min_green {
                min_green = r.green;
            }
            if r.blue > min_blue {
                min_blue = r.blue;
            }
        }
        sum_of_powers += min_red * min_green * min_blue;
    }

    println!("Part 2: Sum of powers = {}", sum_of_powers);
}


fn main() {
    let data = parse();

    println!("{:#?}", data);

    part1(data.clone());
    part2(data.clone());
}
