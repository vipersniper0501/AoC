use std::{fs, io::{self, BufRead}};
use std::collections::HashSet;

fn find_start_of_packet_marker(stream: &String, size: usize) -> Option<i32> {
    for marker in size..stream.len() {
        let possible_marker_string = &stream[marker-size..marker];
        let possible_marker_set: HashSet<char> = possible_marker_string.chars().collect();

        if possible_marker_string.len() == possible_marker_set.len() {
            return Some(marker as i32);
        }
    }
    return None;


}


fn main() {
    let file = fs::File::open("data/input")
        .expect("File not found");

    let reader = io::BufReader::new(file);


    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => println!("{e}")
        }

        match find_start_of_packet_marker(&line_data, 4) {
            Some(v) => println!("(Part 1)First marker in stream appears after the {} character arrives.", v),
            None => println!("There is no marker in this data stream.")
        }

        match find_start_of_packet_marker(&line_data, 14) {
            Some(v) => println!("(Part 2)First marker in stream appears after the {} character arrives.", v),
            None => println!("There is no marker in this data stream.")
        }



    }
}
