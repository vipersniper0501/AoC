use std::{fs, io::{self, BufRead}};


fn parse() -> Vec<Vec<char>> {

    let file = fs::File::open("data/input")
        .expect("No file found");
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        data.push(line_data.chars().collect());
    }

    return data;
}

fn find_connected_digits(row: i32, 
                         col: i32, 
                         data: &mut Box<Vec<Vec<char>>>,
                         is_symbol: &dyn Fn(&char) -> bool) -> i32 {
    let mut digit = "".to_string();
    let mut new_c = col;


    // Walk it back
    while (data[row as usize][new_c as usize] != '.'
        && !is_symbol(&data[row as usize][new_c as usize])) && (new_c - 1) >= 0 {
        new_c = new_c - 1;
        // println!("Walking it back: {}", data[row as usize][new_c as usize]);
    }
    if data[row as usize][new_c as usize] == '.' || is_symbol(&data[row as usize][new_c as usize]) {
        new_c = new_c + 1;
    }

    // Walk it forward
    while (new_c ) < data[0].len() as i32 && data[row as usize][new_c as usize] != '.'
           && !is_symbol(&data[row as usize][new_c as usize]) {
        // println!("Walking it forward: {}", data[row as usize][new_c as usize]);
        digit.push(data[row as usize][new_c as usize]);
        data[row as usize][new_c as usize] = '.';
        new_c = new_c + 1;
    }

    
    // println!("{}", digit);
    return digit.parse::<i32>().expect("Failed to parse i32");
}

fn check_for_parts(row: i32, col: i32, data: &mut Box<Vec<Vec<char>>>) -> i32 {
    let mut found_parts_sum = 0;
    let symbols: Vec<char> = "`~!@#$%^&*()_-+=[]{};:'\",<>/?\\".chars().collect();
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    // Helper function to check if a position is valid in the matrix
    let is_valid_position = |r: i32, c: i32, d: &mut Box<Vec<Vec<char>>>| 
                                             r >= 0 
                                             && (r as usize) < d.len()
                                             && c >= 0 
                                             && (c as usize) < d[0].len();

    // Helper function to check if a character is a symbol
    let is_symbol = |ch: &char| symbols.contains(&ch);

    // Closure to check if a character is a digit
    let is_digit = |ch: char| ch.is_digit(10);

    if !is_symbol(&data[row as usize][col as usize]) {
        return 0;
    }
    
    for &(dr, dc) in &directions {
        let new_r = row + dr;
        let new_c = col + dc;
        if !is_valid_position(new_r, new_c, data) {
            continue;
        }
        let value = data[new_r as usize][new_c as usize];
        if is_digit(value) == true {
            found_parts_sum += find_connected_digits(new_r, new_c, data, &is_symbol);
        }
    }

    return found_parts_sum;
}


fn part1(input: Vec<Vec<char>>) {

    let mut sum_of_parts: i32 = 0;
    let mut data: Box<Vec<Vec<char>>> = Box::new(input.clone());

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            sum_of_parts += check_for_parts(x as i32, y as i32, &mut data);
        }
    }

    println!("Part 1: Sum of parts = {}", sum_of_parts);
}

fn part2() {

    println!("Part 2: ");
}


fn main() {
    let data = parse();

    // println!("{:#?}", data);

    part1(data.clone());
    // part2(data.clone());
}
