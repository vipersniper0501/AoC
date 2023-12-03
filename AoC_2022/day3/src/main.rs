use std::{fs, io::{self, BufRead}};


/// Find duplicate items in different halves of a rucksack
///
/// * `comp1`: First half of rucksack
/// * `comp2`: Second half of rucksack
fn find_duplicate(comp1: &str, comp2: &str) -> char {
    let mut invalid_item = ' ';
    let mut flag = false;
    for c in comp1.chars() {
        if flag {
            break;
        }
        for d in comp2.chars() {
            if c == d {
                flag = true;
                invalid_item = c;
                break;
            }
        }
    }

    return invalid_item;
}


fn find_badge(ruck1: &str, ruck2: &str, ruck3: &str) -> Option<u32> {

    for c in ruck1.chars() {
        for d in ruck2.chars() {
            if c == d {
                for e in ruck3.chars() {
                    if d == e {
                        let badge_value = e as u32;
                        if badge_value > 64 && badge_value < 91 {
                            return Some(badge_value - 64 + 26);
                        } else if badge_value > 96 && badge_value < 123 {
                            return Some(badge_value - 96);
                        }
                    }
                }
            }
        }
    }
    return None;
}



fn main() {

    let file = fs::File::open("data/input")
        .expect("No File Found");

    let reader = io::BufReader::new(file);

    let mut total = 0;
    let mut total_badge_values = 0;


    let mut ruck1 = String::new();
    let mut ruck2 = String::new();

    let mut rucksack_counter = 0;

    for line in reader.lines() {

        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => println!("{e}")
        }

        let first_half = &line_data[..line_data.len()/2];
        let second_half = &line_data[line_data.len()/2..];
        
        let invalid = find_duplicate(first_half, second_half);
        let mut invalid_as_digit = invalid as u32;


        if rucksack_counter == 0 {
            ruck1 = line_data;
            rucksack_counter += 1;
        } else if rucksack_counter == 1 {
            ruck2 = line_data;
            rucksack_counter += 1;
        } else if rucksack_counter == 2 {
            let common_badge_value = find_badge(ruck1.as_str(), ruck2.as_str(), &line_data)
                                .expect("No common badge was found");
            total_badge_values += common_badge_value;
            rucksack_counter = 0;
        }




        if invalid_as_digit > 64 && invalid_as_digit < 91 {
            invalid_as_digit = invalid_as_digit - 64 + 26;
        } else if invalid_as_digit > 96 && invalid_as_digit < 123 {
            invalid_as_digit = invalid_as_digit - 96;
        }


        total += invalid_as_digit;

    }

    println!("Sum of priorities of all rucksacks: {}", total);
    println!("Sum of badge values: {}", total_badge_values);

}
