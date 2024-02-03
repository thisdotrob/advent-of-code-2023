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
    let input = fs::read_to_string("24.txt").unwrap();
    let test_area = TestArea {
        start_x: 200000000000000.0,
        end_x: 400000000000000.0,
        start_y: 200000000000000.0,
        end_y: 400000000000000.0,
    };
    println!("pt1: {}", pt1(&input, &test_area));
}

fn pt1(input: &str, test_area: &TestArea) -> usize {
    let hailstones = parse_input(input);

    let mut intersections = vec![];

    // TODO: calculate all the pairs, then map
    for i in 0..hailstones.len() {
        for j in 0..hailstones.len() {
            if i == j {
                continue
            } else {
                let (hailstone_a, line_equation_a) = &hailstones[i];
                let (hailstone_b, line_equation_b) = &hailstones[j];
                if line_equation_a.is_parallel_to(&line_equation_b) {
                    continue
                }
                let intersection = line_equation_a.intersects(&line_equation_b);
                // TODO: make within_test_area take a Point
                if !within_test_area(intersection.x, intersection.y, test_area){
                    continue
                }
                if !in_past(&intersection, &hailstone_a) && !in_past(&intersection, &hailstone_b) {
                    intersections.push((hailstone_a, hailstone_b, intersection));
                }
            }
        }
    }

    intersections.len() / 2 // loop above double counts each intersection
                            // TODO: add logic to not calculate for line_equation combinations
                            // already seen.
}

fn parse_input(input: &str) -> Vec<(HailStone, LineEquation)> {
    input.lines().map(|line| {
        let hailstone = parse_line(line);
        let line_equation = line_equation(&hailstone);
        (hailstone, line_equation)
    }).collect()
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

fn within_test_area(x: f64, y: f64, test_area: &TestArea) -> bool {
    x >= test_area.start_x && x <= test_area.end_x && y >= test_area.start_y && y <= test_area.end_y
}

// TODO: make this a constructor on LineEquation
fn line_equation(hailstone: &HailStone) -> LineEquation {
    let m = hailstone.y_velocity / hailstone.x_velocity;
    let y = hailstone.y; 
    let x = hailstone.x;
    let c = y - (m * x);
    LineEquation { m, c }
}

fn in_past(intersection: &Point, hailstone: &HailStone) -> bool {
    let mut in_past = false;

    if hailstone.x_velocity > 0.0 {
        if intersection.x < hailstone.x {
            in_past = true;
        }
    } else {
        if intersection.x > hailstone.x {
            in_past = true;
        }
    }

    if hailstone.y_velocity > 0.0 {
        if intersection.y < hailstone.y {
            in_past = true;
        }
    } else {
        if intersection.y > hailstone.y {
            in_past = true;
        }
    }

    in_past
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

#[derive(Debug, PartialEq)]
struct LineEquation {
    m: f64,
    c: f64,
}

impl LineEquation {
    fn intersects(&self, other: &Self) -> Point {
        // m1 * x + c1 = m2 * x + c2
        // (m1 * x) - (m2 * x) + c1 - c2 = 0
        // (m1 - m2) * x = c2 - c1
        // x = (c2 - c1) / (m1 - m2)
        let x = (other.c - self.c) / (self.m - other.m);
        let y = self.m * x + self.c;
        Point { x, y }
    }

    fn is_parallel_to(&self, other: &Self) -> bool {
        self.m == other.m
    }
}

#[cfg(test)]
mod day_24_pt1_tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("24_example.txt").unwrap();
        let expected = vec![
            &HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 },
            &HailStone { x: 18.0, y: 19.0, x_velocity: -1.0, y_velocity: -1.0 },
            &HailStone { x: 20.0, y: 25.0, x_velocity: -2.0, y_velocity: -2.0 },
            &HailStone { x: 12.0, y: 31.0, x_velocity: -1.0, y_velocity: -2.0 },
            &HailStone { x: 20.0, y: 19.0, x_velocity: 1.0, y_velocity: -5.0 },
        ];
        assert_eq!(expected, parse_input(&input).iter().map(|x| &x.0).collect::<Vec<_>>()); // TODO:
                                                                                           // run
                                                                                           // assertion
                                                                                           // over
                                                                                           // second
                                                                                           // element
                                                                                           // too
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
    fn test_line_equation() {
        let hailstone = HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 };
        let expected = LineEquation {
            m: -0.5,
            c: 22.5,
        };
        assert_eq!(expected, line_equation(&hailstone));
    }

    #[test]
    fn test_intersects() { // TODO: rename to intersection
        let hailstone_a = HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 };
        let hailstone_b = HailStone { x: 12.0, y: 31.0, x_velocity: -1.0, y_velocity: -2.0 };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        let expected_intersection = Point { x: 6.2, y: 19.4 };
        assert_eq!(expected_intersection, line_equation_a.intersects(&line_equation_b));
    }

    #[test]
    fn test_in_past() {
        let hailstone_a = HailStone { x: 19.0, y: 13.0, x_velocity: -2.0, y_velocity: 1.0 };
        let hailstone_b = HailStone { x: 20.0, y: 19.0, x_velocity: 1.0, y_velocity: -5.0 };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        let intersection = line_equation_a.intersects(&line_equation_b);
        assert!(in_past(&intersection, &hailstone_a));
        assert!(!in_past(&intersection, &hailstone_b));
    }

    #[test]
    fn test_is_parallel_to() {
        let hailstone_a = HailStone { x: 18.0, y: 19.0, x_velocity: -1.0, y_velocity: -1.0 };
        let hailstone_b = HailStone { x: 20.0, y: 25.0, x_velocity: -2.0, y_velocity: -2.0 };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        assert!(line_equation_a.is_parallel_to(&line_equation_b));

        // Hailstone A: 12, 31, 28 @ -1, -2, -1
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        let hailstone_a = HailStone { x: 12.0, y: 31.0, x_velocity: -1.0, y_velocity: -2.0 };
        let hailstone_b = HailStone { x: 20.0, y: 19.0, x_velocity: 1.0, y_velocity: -5.0 };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        assert!(!line_equation_a.is_parallel_to(&line_equation_b));
    }
}
