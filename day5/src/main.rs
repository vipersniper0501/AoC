use std::{fs, io::{self, BufRead}};



fn load_crates(mut ship: Vec<Vec<String>>, layer: &str) -> Vec<Vec<String>> {

    for i in 0..layer.len()-1 {
        if layer.as_bytes()[i] as char == '[' {
            let sublayer_string = layer.to_string(); 
            let sublayer_str = &sublayer_string[i..i+3];
            ship[i/4].insert(0, sublayer_str.to_string());
        }
    }

    return ship;
}

fn crate_mover_9001(mut ship: Vec<Vec<String>>, quantity: i32, start: i32, end: i32) -> Vec<Vec<String>> {

    let start_stack = start as usize;
    let end_stack = end as usize;

    let mut temp_cargo_stack: Vec<String> = vec![];

    for _ in 0..quantity {
        let cargo_crate = ship[start_stack - 1].pop().expect("No crates left");
        temp_cargo_stack.insert(0, cargo_crate);
    }

    ship[end_stack - 1].append(&mut temp_cargo_stack);


    return ship;
}


fn crate_mover_9000(mut ship: Vec<Vec<String>>, quantity: i32, start: i32, end: i32) -> Vec<Vec<String>> {

    let start_stack = start as usize;
    let end_stack = end as usize;


    for _ in 0..quantity {
        let cargo_crate = ship[start_stack - 1].pop().expect("No crates left");
        ship[end_stack - 1].push(cargo_crate);
    }

    // println!("Moved {} from {} to {}: \n{:#?}", quantity, start, end, ship);

    return ship;
}


fn main() {
    let file = fs::File::open("data/input")
        .expect("No file found");
    let reader = io::BufReader::new(file);


    let mut ship: Vec<Vec<String>> = vec![vec![]; 9];
    let mut ship2: Vec<Vec<String>> = vec![vec![]; 9];


    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => println!("{e}")
        }


        if line_data.contains("[") {
            ship = load_crates(ship, &line_data);
            ship2 = load_crates(ship2, &line_data);
        }
        if line_data.contains("move") {
            let data: Vec<&str> = line_data.split(" ").collect();
            let quantity = data[1].parse::<i32>().expect("Could not parse data");
            let start = data[3].parse::<i32>().expect("Could not parse data");
            let end = data[5].parse::<i32>().expect("Could not parse data");

            ship = crate_mover_9000(ship, quantity, start, end);
            ship2 = crate_mover_9001(ship2, quantity, start, end);
        }


    }
    print!("Cargo organization using the Crate Mover 9000: ");
    for i in 0..ship.len() {
        match ship[i].pop() {
            Some(x) => print!("{}", x),
            None => ()
        }
    }
    println!("");
    print!("Cargo organization using teh Crate Mover 9001: ");
    for i in 0..ship2.len() {
        match ship2[i].pop() {
            Some(x) => print!("{}", x),
            None => ()
        }
    }

}
