use std::{fs, io::{self, BufRead}};


fn parse() -> Vec<String> {

    let file = fs::File::open("data/input")
        .expect("No file foudn");
    let reader = io::BufReader::new(file);

    let mut data: Vec<String> = Vec::new();

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        data.push(line_data);
    }

    return data;
}

fn part1(input: Vec<String>) {

    let mut calibration_sum = 0;

    for line in input {
        let mut line_data: String = line.clone();
        line_data.retain(|c| c.is_numeric());
        let first_number = match line_data.chars().next() {
            Some(c) => c.to_string(),
            None => {
                eprintln!("Failed to get character");
                return ();
            }
        };
        let first_number: i32 = match first_number.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to parse into i32");
                return ();
            }
        };

        let second_number: String = line_data.chars().last().expect("Does not exist").to_string();
        let second_number: i32 = match second_number.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to parse into i32");
                return ();
            }
        };

        let number = first_number * 10 + second_number;
        calibration_sum += number;
    }

    println!("Part 1: Sum of calibration values = {}", calibration_sum);
}
#[derive(Debug)]
struct Character {
    c: char,
    loc: i32,
}

fn part2(input: Vec<String>) {
    let mut calibration_sum = 0;

    for line in input {
        let mut line_data: String = line.clone();
        
        let mut newdata: Vec<Character> = Vec::new();

        let tmp_linedata = line_data.clone();
        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }

        
        // One
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("one", "1");
        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }

        // Two
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("two", "2");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }
        
        // Three
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("three", "3");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }


        // Four
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("four", "4");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }
        
        // Five 
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("five", "5");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }
        
        // Six
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("six", "6");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }

        // Seven
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("seven", "7");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }

        // Eight
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("eight", "8");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }

        // Nine
        let mut tmp_linedata = line_data.clone();
        tmp_linedata = tmp_linedata.replace("nine", "9");

        for (i, character) in tmp_linedata.chars().enumerate() {
            if character.is_numeric() {
                newdata.push(Character { c: character, loc: i as i32 });
            }
        }

        newdata.sort_by_key(|ch| ch.loc);

        line_data.clear();
        for ch in newdata.iter() {
            line_data.push(ch.c);
        }

        line_data.retain(|c| c.is_numeric());

        let first_number = match line_data.chars().next() {
            Some(c) => c.to_string(),
            None => {
                eprintln!("Failed to get character");
                return ();
            }
        };
        let first_number: i32 = match first_number.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to parse into i32");
                return ();
            }
        };

        let second_number: String = line_data.chars().last().expect("Does not exist").to_string();
        let second_number: i32 = match second_number.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to parse into i32");
                return ();
            }
        };

        let number = first_number * 10 + second_number;
        calibration_sum += number;
    }

    println!("Part 2: Corrected sum of calibration values = {}", calibration_sum);

}


fn main() {
    let data = parse();
    part1(data.clone());
    part2(data.clone());
}
