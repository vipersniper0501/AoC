use std::{fs, io::{BufRead, self}};

struct Range(i32, i32);

/// Checks the range of two elves and returns whether or not one completely overlaps
/// the other's boundaries.
///
/// * `elf1`: Range of assignment
/// * `elf2`: Range of assignment
fn check_range(elf1: &Range, elf2: &Range) -> bool{

    if elf2.0 >= elf1.0 && elf2.1 <= elf1.1 {
        return true;
    }
    if elf1.0 >= elf2.0 && elf1.1 <= elf2.1 {
        return true;
    }

    return false;
}


/// Checks the range of two elves and returns whether or not one overlaps
/// the other's boundaries at all.
///
/// * `elf1`: Range of assignment
/// * `elf2`: Range of assignment
fn check_all_range(elf1: &Range, elf2: &Range) -> bool{

    if elf1.0 >= elf2.0 && elf1.0 <= elf2.1 {
        return true;
    }
    if elf1.1 >= elf2.0 && elf1.1 <= elf2.1 {
        return true;
    }
    if elf2.0 >= elf1.0 && elf2.0 <= elf1.1 {
        return true;
    }
    if elf2.1 >= elf1.0 && elf2.1 <= elf1.1 {
        return true;
    }



    return false;
}

fn main() {

    let file = fs::File::open("data/input")
        .expect("No file found");

    let reader = io::BufReader::new(file);


    let mut pair_count = 0;
    let mut overlap_count = 0;

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => println!("{e}")
        }

        let elf_data: Vec<&str> = line_data.split(",").collect();
        let elf1: Vec<&str> = elf_data[0].split("-").collect();
        let elf2: Vec<&str> = elf_data[1].split("-").collect();


        let elf1_val1 = elf1[0].parse::<i32>()
            .expect("Could not parse");

        let elf1_val2 = elf1[1].parse::<i32>()
            .expect("Could not parse");

        let elf2_val1 = elf2[0].parse::<i32>()
            .expect("Could not parse");

        let elf2_val2 = elf2[1].parse::<i32>()
            .expect("Could not parse");


        let elf1_data = Range(elf1_val1, elf1_val2);

        let elf2_data = Range(elf2_val1, elf2_val2);


        if check_range(&elf1_data, &elf2_data) {
            pair_count += 1;
        }
        if check_all_range(&elf1_data, &elf2_data) {
            overlap_count += 1;
        }
    }

    println!("Assignment pairs that overlap completely: {}", pair_count);
    println!("Assignment pairs that overlap at all: {}", overlap_count);
}
