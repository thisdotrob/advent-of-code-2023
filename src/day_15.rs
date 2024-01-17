use std::fs;

pub fn run() {
    let contents = fs::read_to_string("15.txt").unwrap();
    let contents = contents.trim_end();
    println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
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

#[derive(Clone, Copy)]
struct Lens<'a> {
    focal_length: usize,
    label: &'a str,
}

#[derive(Clone, Copy)]
struct Box<'a> {
    lenses: [Option<Lens<'a>>; 256],
}

impl<'a> Box<'_> {
    fn new() -> Box<'a> {
        Box { lenses: [None; 256] }
    }
}

struct Boxes<'a> {
    by_hash: [Box<'a>; 256],
}

impl<'a> Boxes<'_> {
    fn new() -> Boxes<'a> {
        Boxes {
            by_hash: [Box::new(); 256],
        }
    }
}

fn pt2(contents: &str) -> usize {
    let mut focusing_power = 0;

    let boxes = Boxes::new();

    focusing_power
}
