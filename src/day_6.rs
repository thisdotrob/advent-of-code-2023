use std::fs;

pub fn run() {
    println!("pt1: {:?}", pt1("6.txt"));
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
        let race_time: i32 = race_times[i].parse().unwrap();
        let race_record_distance: i32 = race_record_distances[i].parse().unwrap();

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

        let record_beating_button_times = first_record_beating_button_time..=last_record_beating_button_time;

        record_beating_button_time_counts.push(record_beating_button_times.count());
    }

    let answer: usize = record_beating_button_time_counts.iter().product();

    answer
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(288, pt1("6_example.txt"));
    }
}
