use std::fs;

pub fn run() {
    let input = fs::read_to_string("24_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
    // println!("pt2 example: {}", pt2(&input));
    // let input = fs::read_to_string("23.txt").unwrap();
    // println!("pt1: {}", pt1(&input));
    // println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<HailStone> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> HailStone {
    let (position, velocity) = line.split_once(" @ ").unwrap();
    let mut position = position.split(", ");
    let mut velocity = velocity.split(", ");
    HailStone {
        x: position.next().unwrap().parse().unwrap(),
        y: position.next().unwrap().parse().unwrap(),
        x_velocity: velocity.next().unwrap().trim().parse().unwrap(),
        y_velocity: velocity.next().unwrap().trim().parse().unwrap(),
    }
}

#[derive(Debug, PartialEq)]
struct HailStone {
    x: i64,
    y: i64,
    x_velocity: i64,
    y_velocity: i64,
}

#[cfg(test)]
mod day_24_pt1_tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("24_example.txt").unwrap();
        let expected = vec![
            HailStone { x: 19, y: 13, x_velocity: -2, y_velocity: 1 },
            HailStone { x: 18, y: 19, x_velocity: -1, y_velocity: -1 },
            HailStone { x: 20, y: 25, x_velocity: -2, y_velocity: -2 },
            HailStone { x: 12, y: 31, x_velocity: -1, y_velocity: -2 },
            HailStone { x: 20, y: 19, x_velocity: 1, y_velocity: -5 },
        ];
        assert_eq!(expected, parse_input(&input));
    }

    #[test]
    fn test_parse_line() {
        let line = "19, 13, 30 @ -2,  1, -2";
        let expected = HailStone { x: 19, y: 13, x_velocity: -2, y_velocity: 1 };
        assert_eq!(expected, parse_line(line));
    }
}
