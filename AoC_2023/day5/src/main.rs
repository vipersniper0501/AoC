use std::{fs, io::{self, BufRead}};

#[derive(Debug)]
struct Seed {
    value: i32,
    soil: i32,
    fertilizer: i32,
    water: i32,
    light: i32,
    temperature: i32,
    humidity: i32,
    location: i32
}

#[derive(Debug)]
struct Mapping {
    destination: i32,
    source: i32,
    range: i32
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>
}

fn parse() -> Almanac{
    let file = fs::File::open("data/test")
        .expect("No file found");
    let reader = io::BufReader::new(file);

    let mut almanac = Almanac::default();

    let mut current_mapping = 0;

    let maps = ["seed-to-soil map:", "soil-to-fertilizer map:", 
                    "fertilizer-to-water map:", "water-to-light map:",
                    "light-to-temperature map:", "temperature-to-humidity map:",
                    "humidity-to-location map:"];

    for line in reader.lines() {
        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => eprintln!("{e}")
        }

        if line_data.is_empty() {
            current_mapping = 0;
            continue;
        }

        if line_data.contains("seeds") {
            let line_data = match line_data.strip_prefix("seeds: ") {
                Some(v) => v,
                None => ""
            };
            let seeds = line_data.split(" ").filter(|&x| !x.is_empty())
                                 .collect::<Vec<&str>>();
            for s in seeds {
                almanac.seeds.push(Seed { value: s.parse::<i32>()
                                                   .expect("Failed to parse i32"),
                                          soil: 0,
                                          fertilizer: 0,
                                          water: 0,
                                          light: 0,
                                          temperature: 0,
                                          humidity: 0,
                                          location: 0 })
            }
        }

        let slow_search = |s: &str| -> Option<usize> {
            for i in 0..maps.len() {
                if maps[i] == s {
                    return Some(i + 1);
                }
            }

            return None;
        };
        
        match slow_search(&line_data) {
            Some(v) => {
                current_mapping = v as i32;
                continue;
            },
            None => ()
        };

        let create_mapping = |s: &str| -> Mapping {
            let line_data = s.split(" ").collect::<Vec<&str>>();
            let mut data: Vec<i32> = Vec::new();
            for d in line_data {
                data.push(d.parse::<i32>().expect("Failed to parse i32"));
            }
            Mapping { destination: data[0],
                      source: data[1], 
                      range: data[2]}
            
        };

        match current_mapping {
            1 => {
                // println!("Seed to soil mapping: {}", line_data);
                almanac.seed_to_soil.push(create_mapping(&line_data));
            },
            2 => {
                // println!("Soil to fertilizer mapping: {}", line_data);
                almanac.soil_to_fertilizer.push(create_mapping(&line_data));
            },
            3 => {
                // println!("Fertilizer to water mapping: {}", line_data);
                almanac.fertilizer_to_water.push(create_mapping(&line_data));
            },
            4 => {
                // println!("Water to light mapping: {}", line_data);
                almanac.water_to_light.push(create_mapping(&line_data));
            },
            5 => {
                // println!("Light to temperature mapping: {}", line_data);
                almanac.light_to_temperature.push(create_mapping(&line_data));
            },
            6 => {
                // println!("Temperature to humidity mapping: {}", line_data);
                almanac.temperature_to_humidity.push(create_mapping(&line_data));
            },
            7 => {
                // println!("Humidity to location mapping: {}", line_data);
                almanac.humidity_to_location.push(create_mapping(&line_data));
            },
            _ => ()
        }
    }

    return almanac;
}

fn part1(almanac: &Almanac) {

}

fn part2(almanac: &Almanac) {

}

fn main() {
    let almanac = parse();
    println!("{:#?}", almanac);
    part1(&almanac);
    part2(&almanac);
    
}
