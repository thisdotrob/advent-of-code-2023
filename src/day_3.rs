use std::collections::HashSet;
use std::env;
use std::fs;

pub fn run() {
    let contents = fs::read_to_string("3.txt").unwrap().replace("\n", "");

    let contents_chars: Vec<char> = contents.chars().collect();

    assert_eq!(contents.len(), contents_chars.len());

    let pt_1 = sum_of_part_numbers(contents_chars);

    println!("pt1: {}", pt_1);
}

fn sum_of_part_numbers(contents_chars: Vec<char>) -> u32 {
    let line_length = env::var("LINE_LENGTH").unwrap_or(String::from("140"));
    let line_length: usize = line_length.parse().unwrap();

    println!("ll: {}", line_length);

    let mut part_number_indexes = HashSet::new();

    let mut sum_of_part_numbers = 0;

    for i in 0..contents_chars.len() {
        if !part_number_indexes.contains(&i) && is_part_number(i, &contents_chars, line_length) {
            part_number_indexes.insert(i);

            let mut part_number_chars = vec![contents_chars[i]];

            let mut j = i - 1;

            loop {
                let char = contents_chars[j];
                println!("A {}", char);

                if !char.is_numeric() {
                    break;
                } else {
                    part_number_indexes.insert(j);
                    part_number_chars.push(char);
                }

                if j % line_length == 0 {
                    break;
                } else {
                    j -= 1
                }
            }

            part_number_chars.reverse();

            j = i + 1;

            loop {
                let char = contents_chars[j];
                println!("B {}", char);

                if !char.is_numeric() {
                    break;
                } else {
                    part_number_indexes.insert(j);
                    part_number_chars.push(char);
                }

                if j >= (line_length - 1) && (j - (line_length - 1)) % line_length == 0 {
                    break;
                } else {
                    j += 1
                }
            }

            let part_number_str: String = part_number_chars.iter().collect();
            let part_number_int = part_number_str.parse::<u32>().unwrap();
            println!("C {}", part_number_int);
            sum_of_part_numbers += part_number_int;
        };
    }

    sum_of_part_numbers
}

fn is_part_number(i: usize, chars: &Vec<char>, line_length: usize) -> bool {
    let length = chars.len();

    let is_first_in_row = i % line_length == 0;

    let is_last_in_row = if i >= (line_length - 1) {
        (i - (line_length - 1)) % line_length == 0
    } else {
        false
    };

    let char = chars[i];

    println!("{:?}", char);

    if !char.is_numeric() {
        return false;
    }

    let i = i as i32;
    let line_length = line_length as i32;

    println!("i: {}", i);

    let mut adjacent_indexes: Vec<i32> = vec![
        i - 1,  // left
        i + 1,  // right
        i - line_length, // above
        i + line_length, // below
        i - (line_length + 1), // diagonal above left
        i - (line_length - 1),  // diagonal above right
        i + (line_length - 1),  // diagonal below left
        i + (line_length + 1), // diagonal below right
    ];

    println!("{:?}", adjacent_indexes);

    if !is_first_in_row {
        adjacent_indexes.push(i - (line_length + 1)); // diagonal above left
        adjacent_indexes.push(i + (line_length - 1)); // diagonal below left
    }

    if !is_last_in_row {
        adjacent_indexes.push(i - (line_length - 1)); // diagonal above right
        adjacent_indexes.push(i + (line_length + 1)); // diagonal below right
    }

    // remove non-existant adjacent indexes
    adjacent_indexes.retain(|i| *i >= 0);
    adjacent_indexes.retain(|i| *i < length as i32);

    for i in adjacent_indexes {
        let char = chars[i as usize];
        if char != '.' && char.is_ascii_punctuation() {
            return true;
        }
    }

    false
}
