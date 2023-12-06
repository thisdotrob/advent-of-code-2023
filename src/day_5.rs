use std::fs;
use std::collections::HashMap;

pub fn run() {
    let contents = fs::read_to_string("5.txt").unwrap();
    let mut lines = contents.lines();

    let seed_line = lines.next().unwrap();
    let (_, seeds) = seed_line.split_once(":").unwrap();
    let seeds = seeds.trim();
    let seeds: Vec<_> = seeds.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

    let mut maps: HashMap<&str, Vec<MapRange>> = HashMap::new();

    let mut k = "";
    for line in lines {
        if let Some(char) = line.chars().next() {
            if char.is_ascii_digit() {
                let mut line = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let dest_range_start = line.next().unwrap();
                let source_range_start = line.next().unwrap();
                let range_length = line.next().unwrap();
                maps.entry(k).and_modify(|v| {
                    v.push(MapRange(dest_range_start, source_range_start, range_length));
                });
            } else {
                (k, _) = line.split_once(" ").unwrap();
                maps.insert(k, vec![]);
            }
        }
    }

    let map_names = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

    let mut lowest_location_number = usize::MAX;

    for &seed in seeds.iter() {
        let location = map_names.iter().fold(seed, |num, map_name| {
            let map = maps.get(map_name).unwrap();
            get_destination_num(num, map)
        });

        if location < lowest_location_number {
            lowest_location_number = location;
        }
    }

    println!("pt1: {}", lowest_location_number);
}

fn get_destination_num(source_num: usize, map: &Vec<MapRange>) -> usize {
    let map_range = map.iter().find(|m| {
        let source_range_start = m.1;
        let range_length = m.2;
        source_num >= source_range_start && source_num <= (source_range_start + range_length - 1)
    });

    match map_range {
        Some(range) => {
            let dest_range_start = range.0;
            let source_range_start = range.1;
            let offset = source_num - source_range_start;
            dest_range_start + offset
        },
        None => source_num,
    }
}

#[derive(Debug)]
struct MapRange (usize, usize, usize);
