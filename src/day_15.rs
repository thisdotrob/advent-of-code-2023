use std::fs;

pub fn run() {
    let contents = fs::read_to_string("15.txt").unwrap();
    let contents = contents.trim_end();
    // println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
}

fn pt1(contents: &str) -> usize {
    let mut hash_sum = 0;

    let initialization_sequence = contents.split(",");

    for step in initialization_sequence {
        hash_sum += hash(step);
    }

    hash_sum
}

fn pt2(contents: &str) -> usize {
    let mut boxes = Boxes::new();

    let initialization_sequence = contents.split(",");

    for step in initialization_sequence {
        let (label, operator, focal_length) = parse_step(step);
        let box_hash = hash(&label);
        match operator {
            '-' => boxes.remove_lens(box_hash, label),
            '=' => {
                let lens = Lens::new(focal_length.unwrap(), label);
                boxes.add_lens(box_hash, lens);
            },
            _ => panic!("unexpected operator")
        }
    }

    boxes.focusing_power()
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

fn parse_step(step: &str) -> (&str, char, Option<usize>) {
    if let Some((label, focal_length)) = step.split_once("=") {
        (label, '=', Some(focal_length.parse().unwrap()))
    } else {
        let (label, _) = step.split_once("-").unwrap();
        (label, '-', None)
    }
}

#[derive(Clone, Copy, Debug)]
struct Lens<'a> {
    focal_length: usize,
    label: &'a str,
}

impl<'a> Lens<'a> {
    fn new(focal_length: usize, label: &'a str) -> Lens<'a> {
        Lens { focal_length, label }
    }
}

#[derive(Clone, Copy, Debug)]
struct Box<'a> {
    lenses: [Option<Lens<'a>>; 5],
}

impl<'a> Box<'a> {
    fn new() -> Box<'a> {
        Box { lenses: [None; 5] }
    }

    fn remove_lens(&mut self, label: &str) {
        let mut removed = false;

        for i in 0..self.lenses.len() {
            match self.lenses[i] {
                Some(lens) => {
                    if lens.label == label {
                        self.lenses[i] = None;
                        removed = true;
                    } else if removed {
                        self.lenses[i - 1] = Some(lens);
                        self.lenses[i] = None;
                    }
                },
                None => break,
            }
        }
    }

    fn add_lens(&mut self, new_lens: Lens<'a>) {
        for i in 0..self.lenses.len() {
            match self.lenses[i] {
                Some(lens) => {
                    if lens.label == new_lens.label {
                        self.lenses[i] = Some(new_lens);
                        break
                    }
                },
                None => {
                    self.lenses[i] = Some(new_lens);
                    break
                },
            }
        }
    }

    fn focusing_power(&self) -> usize {
        let mut sum = 0;
        for (slot_number, lens) in self.lenses.iter().enumerate() {
            match lens {
                Some(lens) => {
                    let slot_number = 1 + slot_number;
                    sum += slot_number * lens.focal_length;
                },
                None => break
            }
        }
        sum
    }
}

struct Boxes<'a> {
    by_hash: [Box<'a>; 256],
}

impl<'a> Boxes<'a> {
    fn new() -> Boxes<'a> {
        Boxes {
            by_hash: [Box::new(); 256],
        }
    }

    fn remove_lens(&mut self, box_hash: usize, label: &str) {
        let mut r#box = self.by_hash[box_hash];
        r#box.remove_lens(label);
        self.by_hash[box_hash] = r#box;
    }

    fn add_lens(&mut self, box_hash: usize, new_lens: Lens<'a>) {
        let mut r#box = self.by_hash[box_hash];
        r#box.add_lens(new_lens);
        self.by_hash[box_hash] = r#box;
    }
    
    fn focusing_power(&self) -> usize {
        let mut sum = 0;
        for (box_hash, r#box) in self.by_hash.iter().enumerate() {
            let box_focusing_power = r#box.focusing_power();
            if box_focusing_power > 0 {
                let box_number = 1 + box_hash;
                sum += box_number * box_focusing_power;
            }
        }
        sum
    }
}
