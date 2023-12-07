use std::fs;
use std::ops::RangeInclusive;

pub fn run() {
    println!("pt1: {:?}", pt1("6.txt"));
    println!("pt2: {:?}", pt2("6.txt"));
}

fn pt1(filename: &str) -> usize {
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();
    let race_time_line = lines.next().unwrap();
    let race_record_distance_line = lines.next().unwrap();

    let race_times: Vec<_> = race_time_line.split_whitespace().collect();
    let race_record_distances: Vec<_> = race_record_distance_line.split_whitespace().collect();

    let mut record_beating_button_time_counts = vec![];

    for i in 1..race_times.len() {
        let race_time: u64 = race_times[i].parse().unwrap();
        let race_record_distance: u64 = race_record_distances[i].parse().unwrap();
        let times = record_beating_button_times(race_time, race_record_distance);
        record_beating_button_time_counts.push(times.count());
    }

    let answer: usize = record_beating_button_time_counts.iter().product();

    answer
}

fn pt2(filename: &str) -> usize {
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();

    let race_time_line = lines.next().unwrap();
    let (_, race_time) = race_time_line.split_once(":").unwrap();
    let race_time: String = race_time.split_whitespace().collect();
    let race_time: u64 = race_time.parse().unwrap();

    let race_record_distance_line = lines.next().unwrap();
    let (_, race_record_distance) = race_record_distance_line.split_once(":").unwrap();
    let race_record_distance: String = race_record_distance.split_whitespace().collect();
    let race_record_distance: u64 = race_record_distance.parse().unwrap();

    let times = record_beating_button_times(race_time, race_record_distance);

    times.count()
}

fn record_beating_button_times(race_time: u64, race_record_distance: u64) -> RangeInclusive<u64> {
    let mut button_time = 0;
    let mut distance = 0;

    while distance <= race_record_distance {
        button_time += 1;
        let race_time_left = race_time - button_time;
        distance = button_time * race_time_left;
    }

    let first_record_beating_button_time = button_time;

    button_time = race_time;
    let mut distance = 0;

    while distance <= race_record_distance {
        button_time -= 1;
        let race_time_left = race_time - button_time;
        distance = button_time * race_time_left;
    }

    let last_record_beating_button_time = button_time;

    first_record_beating_button_time..=last_record_beating_button_time
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(288, pt1("6_example.txt"));
    }
}

#[cfg(test)]
mod pt2_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(71503, pt2("6_example.txt"));
    }
}
