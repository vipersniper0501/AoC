use std::{fs, io::{self, BufRead}};


/// Calculates score for round of rock paper scissors
///
/// * `o`: oppenents move
/// * `y`: your move
fn calculate_round(o: &str, y: &str) -> i32 {

    let mut score = 0;

    match y {
        "X" => score = score + 1,
        "Y" => score = score + 2,
        "Z" => score = score + 3,
        _ => println!("Invalid input")
    }

    // Tie
    if o == "A" && y == "X" {
        score = score + 3;
    } else if o == "B" && y == "Y" {
        score = score + 3;
    } else if o == "C" && y == "Z" {
        score = score + 3;
    }

    // You win
    if y == "X" && o == "C" {
        score = score + 6;
    } else if y == "Y" && o == "A" {
        score = score + 6;
    } else if y == "Z" && o == "B" {
        score = score + 6;
    }


    return score;
}


fn main() {

    let file = fs::File::open("data/input").
        expect("No file found");

    let reader = io::BufReader::new(file);

    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;

    for line in reader.lines() {

        let mut line_data = String::new();

        match line {
            Ok(v) => line_data = v,
            Err(e) => println!("{e}")
        }


        let opp = line_data.split(" ").collect::<Vec<&str>>()[0].trim();
        // let you = line_data.split(" ").collect::<Vec<&str>>()[1].trim();
        let outcome = line_data.split(" ").collect::<Vec<&str>>()[1].trim();

        // Part 1
        total_score_part1 = total_score_part1 + calculate_round(opp, outcome);
    

        // Part 2
        // Lose
        if outcome == "X" {
            if opp == "A" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "Z");
            } else if opp == "B" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "X");
            } else if opp == "C" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "Y");
            }
        }
        
        // Tie
        if outcome == "Y" {
            if opp == "A" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "X");
            } else if opp == "B" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "Y");
            } else if opp == "C" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "Z");
            }
        }
        
        // Win
        if outcome == "Z" {
            if opp == "A" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "Y");
            } else if opp == "B" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "Z");
            } else if opp == "C" {
                total_score_part2 = total_score_part2 + calculate_round(opp, "X");
            }
        }

    }

    println!("Part 1 Total Score: {}", total_score_part1);
    println!("Part 2 Total Score: {}", total_score_part2);



}
