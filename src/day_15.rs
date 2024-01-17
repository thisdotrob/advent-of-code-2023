use std::fs;

pub fn run() {
    let contents = fs::read_to_string("15.txt").unwrap();
    let contents = contents.trim_end();
    println!("pt1: {}", pt1(&contents));
}

fn pt1(contents: &str) -> usize {
    let mut hash_sum = 0;

    let initialization_sequence = contents.split(",");

    for step in initialization_sequence {
        hash_sum += dbg!(hash(step));
    }

    hash_sum
}

fn hash(step: &str) -> usize {
    let mut value = 0_usize;

    for ascii_code in step.as_bytes() {
        value += *ascii_code as usize;
        value *= 17;
        value %= 256;
    }

    value
}
