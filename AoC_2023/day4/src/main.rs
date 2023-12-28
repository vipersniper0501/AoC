use std::{fs, io::{self, BufRead}};

#[derive(Debug, Clone)]
struct Card {
    _id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    matches: Vec<i32>,
    copies: i32,
}

fn parse() -> Vec<Card> {

    let mut cards: Vec<Card> = Vec::new();
    let mut current_id = 0;

    let file = fs::File::open("data/input")
        .expect("No file found");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        // Remove 'Card #:' prefix
        line_data = line_data.split(":").collect::<Vec<&str>>()[1].to_string();
        line_data = line_data.trim().to_string();
        

        // Split data into winning_numbers and numbers 
        let split_numbers = line_data.split("|").collect::<Vec<&str>>();

        let wn_str = split_numbers[0].split(" ").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
        let n_str = split_numbers[1].split(" ").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

        // Convert list of &str into list of i32
        let mut wn: Vec<i32> = Vec::new();
        for x in wn_str {
             wn.push(x.parse::<i32>().expect("Failed to parse i32"));
        }

        // Convert list of &str into list of i32
        let mut n: Vec<i32> = Vec::new();
        for x in n_str {
            n.push(x.parse::<i32>().expect("Failed to parse i32"));
        }

        current_id += 1;
        let new_card: Card = Card {_id: current_id,
                                   numbers: n, 
                                   winning_numbers: wn, 
                                   matches: Vec::new(),
                                   copies: 1};
        cards.push(new_card);
    }

    return cards;
}

fn find_matches(cards: &mut Vec<Card>) {

    for c in cards {
        for n in &c.numbers {
            if c.winning_numbers.contains(&n) {
                c.matches.push(*n);
            }
        }
    }

}

fn calculate_total_points(cards: &Vec<Card>) -> i32 {
    let mut total_points = 0;

    for c in cards {
        let number_of_matches: u32 = c.matches.len() as u32;
        if number_of_matches == 0 {
            continue;
        }
        total_points += 2_i32.pow(number_of_matches - 1);
    }

    return total_points;
}

fn calculate_number_of_scratchcards(cards: &mut Vec<Card>) -> i32 {
    let mut number_of_scratchcards: i32 = 0;

    for i in 0..cards.len() {
        for x in 0..cards[i].matches.len() {
            cards[i + x + 1].copies += cards[i].copies;
        }
    }

    for c in cards {
        number_of_scratchcards += c.copies;
    }

    return number_of_scratchcards;
}

fn part1(cards: &Vec<Card>) {

    let total_points: i32 = calculate_total_points(cards);
    println!("Part1: Total points = {}", total_points);

}

fn part2(cards: &mut Vec<Card>) {

    let total_number_of_scratchcards = calculate_number_of_scratchcards(cards);
    println!("Part2: Total number of scratchcards = {}", total_number_of_scratchcards);
}

fn main() {
    let mut cards = parse();
    find_matches(&mut cards);
    part1(&cards);
    part2(&mut cards);
}
