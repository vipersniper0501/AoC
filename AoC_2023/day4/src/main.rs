use std::{fs, io::{self, BufRead}};

#[derive(Debug)]
struct Card {
    id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    matches: Vec<i32>
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
        let new_card: Card = Card {id: current_id, numbers: n, winning_numbers: wn, matches: Vec::new() };
        cards.push(new_card);
    }

    return cards;
}


fn part1(cards: Vec<Card>) {

}

fn part2() {

}

fn main() {
    let cards = parse();
    println!("{:#?}", cards);
    part1(cards);
}
