use std::collections::HashMap;
use std::fs;

pub fn run() {
    println!("pt1: {}", pt1());

    println!("pt2: {}", pt2());
}

fn pt1() -> usize {
    let contents = fs::read_to_string("5.txt").unwrap();
    let mut lines = contents.lines();

    let seed_line = lines.next().unwrap();
    let (_, seeds) = seed_line.split_once(":").unwrap();
    let seeds = seeds.trim();
    let seeds: Vec<_> = seeds
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

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

    let map_names = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

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

    lowest_location_number
}

fn pt2() -> usize {
    let contents = fs::read_to_string("5.txt").unwrap();
    let mut lines = contents.lines();

    let seed_line = lines.next().unwrap();
    let (_, seeds) = seed_line.split_once(":").unwrap();
    let seeds = seeds.trim();
    let seeds: Vec<_> = seeds
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let seeds = seeds.chunks_exact(2);
    let seeds = seeds.map(|s| -> [usize; 2] { s.try_into().unwrap() });
    let seeds = seeds.map(|mut s| {
        s[1] -= 1;
        s
    });

    let mut maps: HashMap<String, HashMap<String, Vec<MapRange>>> = HashMap::new();

    let mut source_k = String::from("");
    let mut dest_k = String::from("");
    let mut categories = vec![];

    categories.push("seed".to_string());

    for line in lines {
        if let Some(char) = line.chars().next() {
            if char.is_ascii_digit() {
                let mut line = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let dest_range_start = line.next().unwrap();
                let source_range_start = line.next().unwrap();
                let range_length = line.next().unwrap() - 1;
                let source_maps = maps
                    .entry(String::from(&source_k))
                    .or_insert(HashMap::new());
                let map = source_maps
                    .entry(String::from(&dest_k))
                    .or_insert(Vec::new());
                let map_range = MapRange(range_length, source_range_start, dest_range_start);
                map.push(map_range);
            } else {
                let mut split = line.split_whitespace();
                let mut split = split.next().unwrap().split("-");
                source_k = split.next().unwrap().to_string();
                split.next();
                dest_k = split.next().unwrap().to_string();
                categories.push(String::from(&dest_k));
            }
        }
    }

    for seed in seeds {
        let [mut start_seed, mut seed_range_length] = seed;

        while seed_range_length > 0 {
            let mut start_num = start_seed;
            let mut range_length = seed_range_length;
            for w in categories.windows(2) {
                let source_k = &w[0];
                let dest_k = &w[1];
                let map = maps.get(source_k).unwrap().get(dest_k).unwrap();
                let map_range = map.iter().find(|mr| {
                    let range_length = mr.0;
                    let source_range_start = mr.1;
                    start_num >= source_range_start
                        && start_num <= (source_range_start + range_length - 1)
                });

                if let Some(mr) = map_range {
                    let source_range_start = mr.1;
                    let offset = start_num - source_range_start;
                    let map_range_length = mr.0 - offset;
                    if map_range_length < range_length {
                        range_length = map_range_length
                    };

                    let dest_range_start = mr.2;
                    start_num = dest_range_start + offset;
                } else {
                    for mr in map {
                        let source_range_start = mr.1;
                        if source_range_start > start_num
                            && source_range_start <= start_num + range_length
                        {
                            // last number that can be directly mapped source > dest
                            range_length = source_range_start - start_num;
                        }
                    }
                }

                let seed_maps = maps.get_mut("seed").unwrap();
                let map = seed_maps.entry(dest_k.to_string()).or_insert(Vec::new());
                let map_range = MapRange(range_length, start_seed, start_num);
                map.push(map_range);
            }

            start_seed += range_length + 1;
            seed_range_length -= range_length;
        }
    }

    let seed_to_location_map = maps.get("seed").unwrap().get("location").unwrap();

    let lowest_location_map_range = seed_to_location_map.iter().min_by(|x, y| x.2.cmp(&y.2));

    lowest_location_map_range.unwrap().2
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
        }
        None => source_num,
    }
}

#[derive(Debug)]
struct MapRange(usize, usize, usize);
