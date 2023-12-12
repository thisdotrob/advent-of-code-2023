use std::collections::HashMap;
use std::{fs, str::Lines};

pub fn run() {
    println!("pt1: {:?}", pt1("8.txt"));
    println!("pt2: {:?}", pt2("8.txt"));
}

fn pt2(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();
    let mut instructions = parse_instructions(lines.next().unwrap());
    lines.next();

    let mut tree: HashMap<String, [String; 2]> = HashMap::new();

    let mut start_labels = vec![];

    for line in lines {
        let label = &line[..3];
        if label.chars().last() == Some('A') {
            start_labels.push(String::from(label));
        }
        let left = String::from(&line[7..10]);
        let right = String::from(&line[12..15]);
        tree.insert(String::from(label), [left, right]);
    }

    let mut steps = 0;

    for start_label in &start_labels {
        let mut label = start_label;

        let mut step = 0;

        loop {
            step += 1;

            let instruction = instructions.next().unwrap();

            let [left, right] = tree.get(label).unwrap();

            label = if instruction == 'L' { left } else { right };

            if label.chars().last() == Some('Z') {
                if steps == 0 {
                    steps = step;
                } else {
                    steps = lcm(steps, step);
                }
                break;
            }
        }
    }

    steps
}

fn pt1(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();
    let mut instructions = parse_instructions(lines.next().unwrap());
    lines.next();
    let tree = construct_tree(lines);
    let mut label = 0;
    let mut steps = 0;
    loop {
        steps += 1;

        let instruction = instructions.next().unwrap();

        let [left, right] = tree[label];

        label = if instruction == 'L' { left } else { right };

        if label == 26425 {
            break;
        }
    }
    steps
}

struct Instructions {
    sequence: Vec<char>,
    current_index: usize,
}

impl Iterator for Instructions {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = Some(self.sequence[self.current_index]);

        self.current_index = if self.current_index == self.sequence.len() - 1 {
            0
        } else {
            self.current_index + 1
        };

        instruction
    }
}

fn parse_instructions(instructions_line: &str) -> Instructions {
    let sequence: Vec<_> = instructions_line.chars().collect();
    Instructions {
        sequence,
        current_index: 0,
    } // TODO initialise current_index internally
}

fn construct_tree(lines: Lines) -> [[usize; 2]; 26426] {
    let mut tree = [[0; 2]; 26426];
    for line in lines {
        let (label, left, right) = parse_line(line);
        tree[label] = [left, right];
    }
    tree
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    // TODO rename to parse_node
    let label = node_index(&line[..3]);
    let left = node_index(&line[7..10]);
    let right = node_index(&line[12..15]);
    (label, left, right)
}

fn node_index(label: &str) -> usize {
    let mut chars = label.chars();
    let mut index: usize = 0;
    for _ in 0..3 {
        index <<= 5;
        let int: usize = char_to_index_int(chars.next().unwrap());
        index |= int;
    }
    index
}

fn char_to_index_int(char: char) -> usize {
    char as usize - 65
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn test_first_example_input() {
        assert_eq!(2, pt1("8_example_1.txt"));
    }

    #[test]
    fn test_second_example_input() {
        assert_eq!(6, pt1("8_example_2.txt"));
    }
}

#[cfg(test)]
mod node_label_to_int_tests {
    use super::*;

    #[test]
    fn test_int_value_of_z_char() {
        let char = 'Z';
        assert_eq!(90, char as u8);
    }

    #[test]
    fn test_int_value_of_a_char() {
        let char = 'A';
        assert_eq!(65, char as u8);
    }
    #[test]
    fn test_aaa_is_0b00000_00000_00000() {
        assert_eq!(0b00000_00000_00000, node_index("AAA"));
    }

    #[test]
    fn test_aza_is_0b00000_11001_00000() {
        assert_eq!(0b00000_11001_00000, node_index("AZA"));
    }

    #[test]
    fn test_zzz_is_0b11001_11001_11001() {
        assert_eq!(0b11001_11001_11001, node_index("ZZZ"));
    }
}

#[cfg(test)]
mod parse_line_tests {
    // TODO rename to parse_node_tests
    use super::*;

    #[test]
    fn parses_first_example_input_line_1() {
        assert_eq!(
            (
                0b00000_00000_00000,
                0b00001_00001_00001,
                0b00010_00010_00010
            ),
            parse_line("AAA = (BBB, CCC)")
        );
    }
}

#[cfg(test)]
mod parse_instructions_tests {
    use super::*;

    #[test]
    fn returns_looping_instructions() {
        let mut instructions = parse_instructions("RL");
        for _ in 0..100 {
            assert_eq!('R', instructions.next().unwrap());
            assert_eq!('L', instructions.next().unwrap());
        }
    }
}

#[cfg(test)]
mod construct_tree_tests {
    use super::*;

    #[test]
    fn single_line_creates_single_element_tree() {
        let contents = "AAA = (BBB, CCC)\n";
        let tree = construct_tree(contents.lines());
        assert_eq!(tree[0], [0b00001_00001_00001, 0b00010_00010_00010]);
    }

    #[test]
    fn removing_first_two_lines_then_creating_tree_does_not_panic() {
        let contents = "RL\n\nAAA = (BBB, CCC)\n";
        let mut lines = contents.lines();
        let _instructions = lines.next();
        let _blank_line = lines.next();
        let tree = construct_tree(lines);
        assert_eq!(tree[0], [0b00001_00001_00001, 0b00010_00010_00010]);
    }
}
