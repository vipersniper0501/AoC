use std::{fs, io::{self, BufRead}};
use rayon::prelude::*;

/*
 * Idea for making it use less memory: After creating a list of "Seeds" immediately
 * find its lowest value and add it to a seperate list. Then clear the almanac's 
 * list of Seeds and add the next range of seeds to the almanac and repeat. 
 * When done with each range and found its lowest, find the lowest of the lowest
 * and return that. This way we only have one range's worth of memory loaded at 
 * a time.
 */

// #[derive(Debug)]
// struct Seed {
    // value: i64,
// }

type Seed = i64;

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

fn add_seeds_to_almanac(almanac: &mut Almanac, start: i64, range: i64) {
    for x in start..(start + range) {
        // almanac.seeds.push(
            // Seed { value: x});
        almanac.seeds.push(x);
    }

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
                    add_seeds_to_almanac(&mut almanac, start, range);
                    
                }
            } else {
                for i in 0..seeds.len() {
                    // almanac.seeds.push(Seed { value: seeds[i].parse::<i64>()
                                               // .expect("Failed to parse i64")})
                    almanac.seeds.push(seeds[i].parse::<i64>()
                                               .expect("Failed to parse i64"))
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

    let maps = [&almanac.seed_to_soil, &almanac.soil_to_fertilizer,
                &almanac.fertilizer_to_water, &almanac.water_to_light,
                &almanac.light_to_temperature, &almanac.temperature_to_humidity,
                &almanac.humidity_to_location];

    almanac.seeds.par_iter_mut().for_each(|s| {

        for i in 0..maps.len() {
            for m in maps[i] {
                // if s.value >= m.source && s.value < m.source + m.range {
                    // let offset: i64 = s.value - m.source;
                    // s.value = m.destination + offset;
                    // break;
                // }
                if *s >= m.source && *s < m.source + m.range {
                    let offset: i64 = *s - m.source;
                    *s = m.destination + offset;
                    break;
                }
            }
        }
    });

    println!("Finding lowest location...");

    // almanac.seeds.par_iter().map(|s| s.value).min().unwrap_or(0)
    almanac.seeds.par_iter().map(|s| *s).min().unwrap_or(0)
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
