use std::{fs, io::{self, BufRead}};
use rayon::prelude::*;

#[derive(Debug)]
struct Seed {
    value: i64,
}

#[derive(Debug)]
struct Mapping {
    destination: i64,
    source: i64,
    range: i64
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

fn parse(seed_range: bool) -> Almanac{
    let file = fs::File::open("data/input")
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

            if seed_range == true {
                for i in (0..seeds.len()).step_by(2) {
                    let start = seeds[i].parse::<i64>()
                                        .expect("Failed to parse i64");
                    let range = seeds[i + 1].parse::<i64>()
                                            .expect("Failed to parse i64");

                    println!("Start: {}, Range: {}", start, range);
                    for x in start..(start + range) {
                        almanac.seeds.push(
                            Seed { value: x});
                    }
                    
                }
            } else {
                for i in 0..seeds.len() {
                    almanac.seeds.push(Seed { value: seeds[i].parse::<i64>()
                                               .expect("Failed to parse i64")})
                }
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
                current_mapping = v as i64;
                continue;
            },
            None => ()
        };

        let create_mapping = |s: &str| -> Mapping {
            let line_data = s.split(" ");
            let mut data: Vec<i64> = Vec::new();
            for d in line_data {
                data.push(d.parse::<i64>().expect("Failed to parse i64"));
            }
            Mapping { destination: data[0],
                      source: data[1], 
                      range: data[2]}
            
        };

        match current_mapping {
            1 => {
                almanac.seed_to_soil.push(create_mapping(&line_data));
            },
            2 => {
                almanac.soil_to_fertilizer.push(create_mapping(&line_data));
            },
            3 => {
                almanac.fertilizer_to_water.push(create_mapping(&line_data));
            },
            4 => {
                almanac.water_to_light.push(create_mapping(&line_data));
            },
            5 => {
                almanac.light_to_temperature.push(create_mapping(&line_data));
            },
            6 => {
                almanac.temperature_to_humidity.push(create_mapping(&line_data));
            },
            7 => {
                almanac.humidity_to_location.push(create_mapping(&line_data));
            },
            _ => ()
        }
    }

    return almanac;
}

fn calculate_seeds(almanac: &mut Almanac) -> i64 {
    println!("Calculating seeds value...");

    almanac.seeds.par_iter_mut().for_each(|s| {
    // for s in &mut almanac.seeds {
        // println!("Seed: {}", s.value);
        
        // Seed to soil
        for m in &almanac.seed_to_soil {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }

        // println!("Soil: {}", s.value);

        // Soil to fertilizer
        for m in &almanac.soil_to_fertilizer {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }

        // println!("Fertilizer: {}", s.value);

        // Fertilizer to water
        for m in &almanac.fertilizer_to_water {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }

        // println!("Water: {}", s.value);

        // Water to light
        for m in &almanac.water_to_light {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }

        // println!("Light: {}", s.value);

        // Light to temperature
        for m in &almanac.light_to_temperature {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }
        
        // println!("Temperature: {}", s.value);

        // Temperature to humidity
        for m in &almanac.temperature_to_humidity {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }

        // println!("Humidity: {}", s.value);

        // Humidity to location
        for m in &almanac.humidity_to_location {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.value = m.destination + offset;
                break;
            }
        }
        // println!("Location: {}\n\n", s.value);
    });

    println!("Finding lowest location...");

    almanac.seeds.par_iter().map(|s| s.value).min().unwrap_or(0)
}

fn part1(almanac: &mut Almanac) {
    let lowest_loc: i64 = calculate_seeds(almanac);

    println!("Part 1: Lowest location = {}\n", lowest_loc);
}

fn part2(almanac: &mut Almanac) {
    let lowest_loc: i64 = calculate_seeds(almanac);

    println!("Part 2: Lowest location = {}\n", lowest_loc);
}

fn main() {
    let mut almanac = parse(false);
    part1(&mut almanac);
    let mut almanac = parse(true);
    // println!("{:#?}", almanac);
    part2(&mut almanac);
    
}
