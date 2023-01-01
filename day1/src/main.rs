use std::{fs, io::{self, BufRead}};


/// Insertion Sort
///
/// * `list`: 
fn sort(list: &mut Vec<i32>) {
    

    for i in 1..list.len() {

        let mut j = i;
        while j > 0 && list[j] > list[j - 1] {
            list.swap(j, j - 1);
            j = j - 1;
        }
    }
    
}


fn main() {


    let mut max_calories = 0;

    let mut top_three: Vec<i32> = vec![0];

    let file = fs::File::open("data/input")
   .expect("No file found");

    let reader = io::BufReader::new(file);
    
    let mut current_calories = 0;

    for line in reader.lines() {
        
        let mut line_val = "".to_string();
        match line {
            Ok(val) => line_val = val,
            _error => println!("Error reading line"),
        }

        if !line_val.is_empty() {

            let line_val = line_val.parse::<i32>()
                .expect("Could not cast line value to int");

            current_calories = current_calories + line_val;


        } else { 
            if max_calories < current_calories {
                max_calories = current_calories;
            }
            top_three.insert(0, current_calories);
            current_calories = 0;
        }


    }

    sort(&mut top_three);


    println!("Maximum amount of calories carried by an elf is {max_calories}");
    println!("Top three calorie counts: \n1) {}\n2) {}\n3) {}", top_three[0],
    top_three[1], top_three[2]);
    println!("Total calories of top three elves: {}", top_three[0] + top_three[1] + top_three[2]);


}
