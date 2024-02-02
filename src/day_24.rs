use std::collections::HashSet;
use std::cmp::{min, max};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("24_example.txt").unwrap();
    let test_area = TestArea {
        start_x: 7.0,
        end_x: 27.0,
        start_y: 7.0,
        end_y: 27.0,
    };
    println!("pt1 example: {}", pt1(&input, &test_area));
    // println!("pt2 example: {}", pt2(&input));
    // let input = fs::read_to_string("23.txt").unwrap();
    // println!("pt1: {}", pt1(&input));
    // println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str, test_area: &TestArea) -> usize {
    let hailstones = parse_input(input);

    let trajectories: Vec<Vec<Point>> = hailstones.iter().map(|hailstone| trajectory(hailstone, test_area)).collect();

    let mut intersections = HashSet::new();

    // TODO: calculate all the pairs, then map
    for i in 0..trajectories.len() {
        for j in 0..trajectories.len() {
            if i == j {
                continue
            } else {
                // println!("A: {:?}, B: {:?}", trajectories[i], trajectories[j]);
                if intersect(&trajectories[i], &trajectories[j]) {
                    // println!("YES");
                    intersections.insert((min(i, j), max(i, j)));
                } else {
                    // println!("NO");
                }
                // println!();
            }
        }
    }

    intersections.len()
}

fn parse_input(input: &str) -> Vec<HailStone> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> HailStone {
    let (position, velocity) = line.split_once(" @ ").unwrap();
    let mut position = position.split(", ");
    let mut velocity = velocity.split(", ");
    let x_velocity: f64 = velocity.next().unwrap().trim().parse().unwrap();
    let y_velocity: f64 = velocity.next().unwrap().trim().parse().unwrap();
    HailStone {
        x: position.next().unwrap().parse().unwrap(),
        y: position.next().unwrap().parse().unwrap(),
        x_velocity: round_to_decimal_places(x_velocity / 1000.0, 1000000000),
        y_velocity: round_to_decimal_places(y_velocity / 1000.0, 1000000000),
    }
}

fn within_test_area(x: f64, y: f64, test_area: &TestArea) -> bool {
    x >= test_area.start_x && x <= test_area.end_x && y >= test_area.start_y && y <= test_area.end_y
}

fn trajectory(hailstone: &HailStone, test_area: &TestArea) -> Vec<Point> {
    let mut points = vec![];

    let HailStone { mut x, mut y, x_velocity, y_velocity } = hailstone;

    while within_test_area(x, y, test_area) {
        points.push(Point { x, y });
        x += x_velocity;
        y += y_velocity;
    }

    points
}

fn intersect(trajectory_a: &Vec<Point>, trajectory_b: &Vec<Point>) -> bool {
    trajectory_a.iter().any(|point| trajectory_b.contains(&point))
}

fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (value * multiplier).floor() / multiplier
}

#[derive(Debug, PartialEq)]
struct HailStone { // TODO: rename to HailStone, or rename variables to hail_stone
    x: f64,
    y: f64,
    x_velocity: f64,
    y_velocity: f64,
}

struct TestArea {
    start_x: f64,
    end_x: f64,
    start_y: f64,
    end_y: f64,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[cfg(test)]
mod day_24_pt1_tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("24_example.txt").unwrap();
        let expected = vec![
            HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 },
            HailStone { x: 18.0, y: 19.0, x_velocity: -1.0, y_velocity: -1.0 },
            HailStone { x: 20.0, y: 25.0, x_velocity: -2.0, y_velocity: -2.0 },
            HailStone { x: 12.0, y: 31.0, x_velocity: -1.0, y_velocity: -2.0 },
            HailStone { x: 20.0, y: 19.0, x_velocity: 1.0, y_velocity: -5.0 },
        ];
        assert_eq!(expected, parse_input(&input));
    }

    #[test]
    fn test_parse_line() {
        let line = "19, 13, 30 @ -2,  1, -2";
        let expected = HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 };
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_within_test_area() {
        let test_area = TestArea { start_x: 7.0, end_x: 27.0, start_y: 7.0, end_y: 27.0 };
        assert!(within_test_area(7.0, 7.0, &test_area));
        assert!(within_test_area(27.0, 27.0, &test_area));
        assert!(within_test_area(7.0, 27.0, &test_area));
        assert!(within_test_area(27.0, 7.0, &test_area));
        assert!(within_test_area(21.0, 21.0, &test_area));
        assert!(!within_test_area(6.0, 6.0, &test_area));
        assert!(!within_test_area(6.0, 21.0, &test_area));
        assert!(!within_test_area(21.0, 6.0, &test_area));
        assert!(!within_test_area(28.0, 28.0, &test_area));
        assert!(!within_test_area(28.0, 21.0, &test_area));
        assert!(!within_test_area(21.0, 28.0, &test_area));
    }

    #[test]
    fn test_trajectory() {
        let hailstone = HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 };
        let test_area = TestArea { start_x: 7.0, end_x: 27.0, start_y: 7.0, end_y: 27.0 };
        let expected = vec![
            Point { x: 19.0, y: 13.0},
            Point { x: 17.0, y: 14.0},
            Point { x: 15.0, y: 15.0},
            Point { x: 13.0, y: 16.0},
            Point { x: 11.0, y: 17.0},
            Point { x: 9.0, y: 18.0},
            Point { x: 7.0, y: 19.0},
        ];
        assert_eq!(expected, trajectory(&hailstone, &test_area));
    }

    #[test]
    fn test_intersect() {
        let trajectory_a = vec![Point { x: 1.0, y: 1.0}];
        let trajectory_b = vec![Point { x: 1.0, y: 1.0}];
        assert!(intersect(&trajectory_a, &trajectory_b));

        let trajectory_a = vec![Point { x: 2.0, y: 2.0}, Point { x: 1.0, y: 1.0}];
        let trajectory_b = vec![Point { x: 1.0, y: 1.0}];
        assert!(intersect(&trajectory_a, &trajectory_b));

        assert!(!intersect(&vec![], &vec![]));

        let trajectory_a = vec![Point { x: 1.0, y: 2.0}];
        let trajectory_b = vec![Point { x: 1.0, y: 1.0}];
        assert!(!intersect(&trajectory_a, &trajectory_b));

        let trajectory_a = vec![Point { x: 2.0, y: 1.0}];
        let trajectory_b = vec![Point { x: 1.0, y: 1.0}];
        assert!(!intersect(&trajectory_a, &trajectory_b));

        let trajectory_a = vec![Point { x: 2.0, y: 2.0}];
        let trajectory_b = vec![Point { x: 1.0, y: 1.0}];
        assert!(!intersect(&trajectory_a, &trajectory_b));
    }
}
