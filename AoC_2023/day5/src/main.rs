use std::{fs, io::{self, BufRead}};

#[derive(Debug)]
struct Seed {
    value: i64,
    soil: i64,
    fertilizer: i64,
    water: i64,
    light: i64,
    temperature: i64,
    humidity: i64,
    location: i64
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

fn parse() -> Almanac{
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
            for s in seeds {
                almanac.seeds.push(Seed { value: s.parse::<i64>()
                                                   .expect("Failed to parse i64"),
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
                current_mapping = v as i64;
                continue;
            },
            None => ()
        };

        let create_mapping = |s: &str| -> Mapping {
            let line_data = s.split(" ").collect::<Vec<&str>>();
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

    for s in &mut almanac.seeds {
        
        // Seed to soil
        for m in &almanac.seed_to_soil {
            if s.value >= m.source && s.value < m.source + m.range {
                let offset: i64 = s.value - m.source;
                s.soil = m.destination + offset;
            }
        }
        if s.soil == 0 {
            s.soil = s.value;
        }

        // Soil to fertilizer
        for m in &almanac.soil_to_fertilizer {
            if s.soil >= m.source && s.soil < m.source + m.range {
                let offset: i64 = s.soil - m.source;
                s.fertilizer = m.destination + offset;
            }
        }
        if s.fertilizer == 0 {
            s.fertilizer = s.soil;
        }

        // Fertilizer to water
        for m in &almanac.fertilizer_to_water {
            if s.fertilizer >= m.source && s.fertilizer < m.source + m.range {
                let offset: i64 = s.fertilizer - m.source;
                s.water = m.destination + offset;
            }
        }
        if s.water == 0 {
            s.water = s.fertilizer;
        }

        // Water to light
        for m in &almanac.water_to_light {
            if s.water >= m.source && s.water < m.source + m.range {
                let offset: i64 = s.water - m.source;
                s.light = m.destination + offset;
            }
        }
        if s.light == 0 {
            s.light = s.water;
        }

        // Light to temperature
        for m in &almanac.light_to_temperature {
            if s.light >= m.source && s.light < m.source + m.range {
                let offset: i64 = s.light - m.source;
                s.temperature = m.destination + offset;
            }
        }
        if s.temperature == 0 {
            s.temperature = s.light;
        }

        // Temperature to humidity
        for m in &almanac.temperature_to_humidity {
            if s.temperature >= m.source && s.temperature < m.source + m.range {
                let offset: i64 = s.temperature - m.source;
                s.humidity = m.destination + offset;
            }
        }
        if s.humidity == 0 {
            s.humidity = s.temperature;
        }

        // Humidity to location
        for m in &almanac.humidity_to_location {
            if s.humidity >= m.source && s.humidity < m.source + m.range {
                let offset: i64 = s.humidity - m.source;
                s.location = m.destination + offset;
            }
        }
        if s.location == 0 {
            s.location = s.humidity;
        }
    }

    let mut lowest_loc: i64 = almanac.seeds[0].location;
    for s in &almanac.seeds {
        if s.location < lowest_loc {
            lowest_loc = s.location;
        }
    }
    
    return lowest_loc;
}

fn part1(almanac: &mut Almanac) {
    let lowest_loc: i64 = calculate_seeds(almanac);

    println!("Part 1: Lowest location = {}", lowest_loc);
}

fn part2(almanac: &Almanac) {

    println!("Part 2: Not complete");
}

fn main() {
    let mut almanac = parse();
    // println!("{:#?}", almanac);
    part1(&mut almanac);
    part2(&almanac);
    
}
