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

    println!("pt2 example: {}", pt2(&input));

    let input = fs::read_to_string("24.txt").unwrap();

    let test_area = TestArea {
        start_x: 200000000000000.0,
        end_x: 400000000000000.0,
        start_y: 200000000000000.0,
        end_y: 400000000000000.0,
    };

    println!("pt1: {}", pt1(&input, &test_area));

    println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str, test_area: &TestArea) -> usize {
    let hailstones = parse_input(input);

    let mut intersections = vec![];

    // TODO: calculate all the pairs, then map
    for i in 0..hailstones.len() {
        for j in 0..hailstones.len() {
            if i == j {
                continue;
            } else {
                let (hailstone_a, line_equation_a) = &hailstones[i];
                let (hailstone_b, line_equation_b) = &hailstones[j];
                if line_equation_a.is_parallel_to(&line_equation_b) {
                    continue;
                }
                let intersection = line_equation_a.intersects(&line_equation_b);
                // TODO: make within_test_area take a Point
                if !within_test_area(intersection.x, intersection.y, test_area) {
                    continue;
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

fn pt2(input: &str) -> f64 {
    let hailstones: Vec<_> = input.lines().map(|line| parse_line(line)).take(4).collect();

    let start_hailstone = &hailstones[0];

    let cramers_rule_equations = hailstones[1..]
        .iter()
        .map(|hailstone| cramer_equations(start_hailstone, &hailstone));

    let cramers_matrix: Vec<_> = cramers_rule_equations
        .clone()
        .flatten()
        .map(|equation| equation.coefficients.as_vec())
        .collect();

    assert_eq!(6, cramers_matrix.len());
    assert_eq!(6, cramers_matrix[0].len());

    let cramers_constants: Vec<_> = cramers_rule_equations
        .flatten()
        .map(|equation| equation.rhs)
        .collect();

    let coefficient_determinant = determinant(&cramers_matrix);

    let rock_x_matrix = replace_with_constants(&cramers_matrix, &cramers_constants, 0);
    let rock_x_determinant = determinant(&rock_x_matrix);
    let rock_x = rock_x_determinant / coefficient_determinant;

    let rock_y_matrix = replace_with_constants(&cramers_matrix, &cramers_constants, 1);
    let rock_y_determinant = determinant(&rock_y_matrix);
    let rock_y = rock_y_determinant / coefficient_determinant;

    let rock_z_matrix = replace_with_constants(&cramers_matrix, &cramers_constants, 2);
    let rock_z_determinant = determinant(&rock_z_matrix);
    let rock_z = rock_z_determinant / coefficient_determinant;

    rock_x + rock_y + rock_z
}

fn determinant(matrix: &Vec<Vec<f64>>) -> f64 {
    let mut answer = 0.;
    for i in 0..matrix.len() {
        let element = matrix[i][0];
        let mut minor_matrix = matrix.clone(); // TODO can the clone() be avoided?
        minor_matrix.remove(i);
        for row in minor_matrix.iter_mut() {
            row.remove(0);
        }
        let minor = if minor_matrix.len() == 1 {
            minor_matrix[0][0]
        } else {
            determinant(&minor_matrix)
        };
        let place_value = if i % 2 == 0 { 1. } else { -1. };
        let cofactor = minor * place_value;
        let product = cofactor * element;
        answer += product
    }
    answer
}

fn cramer_equations(hailstone_a: &HailStone, hailstone_b: &HailStone) -> Vec<CramerEquation> {
    vec![
        CramerEquation {
            coefficients: CramerCoefficients {
                rock_x: hailstone_a.y_velocity - hailstone_b.y_velocity,
                rock_y: hailstone_b.x_velocity - hailstone_a.x_velocity,
                rock_z: 0.,
                rock_x_velocity: hailstone_b.y - hailstone_a.y,
                rock_y_velocity: hailstone_a.x - hailstone_b.x,
                rock_z_velocity: 0.,
            },
            rhs: (hailstone_a.x * hailstone_a.y_velocity)
                - (hailstone_a.y * hailstone_a.x_velocity)
                - (hailstone_b.x * hailstone_b.y_velocity)
                + (hailstone_b.y * hailstone_b.x_velocity),
        },
        CramerEquation {
            coefficients: CramerCoefficients {
                rock_x: hailstone_a.z_velocity - hailstone_b.z_velocity,
                rock_y: 0.,
                rock_z: hailstone_b.x_velocity - hailstone_a.x_velocity,
                rock_x_velocity: hailstone_b.z - hailstone_a.z,
                rock_y_velocity: 0.,
                rock_z_velocity: hailstone_a.x - hailstone_b.x,
            },
            rhs: (hailstone_a.x * hailstone_a.z_velocity)
                - (hailstone_a.z * hailstone_a.x_velocity)
                - (hailstone_b.x * hailstone_b.z_velocity)
                + (hailstone_b.z * hailstone_b.x_velocity),
        },
    ]
}

fn replace_with_constants(
    matrix: &Vec<Vec<f64>>,
    constants: &Vec<f64>,
    column_index: usize,
) -> Vec<Vec<f64>> {
    let mut matrix = matrix.clone(); // Can we avoid the clone?
    for i in 0..matrix.len() {
        matrix[i][column_index] = constants[i]
    }
    matrix
}

struct CramerCoefficients {
    rock_x: f64,
    rock_y: f64,
    rock_z: f64,
    rock_x_velocity: f64,
    rock_y_velocity: f64,
    rock_z_velocity: f64,
}

impl CramerCoefficients {
    fn as_vec(&self) -> Vec<f64> {
        vec![
            self.rock_x,
            self.rock_y,
            self.rock_z,
            self.rock_x_velocity,
            self.rock_y_velocity,
            self.rock_z_velocity,
        ]
    }
}

struct CramerEquation {
    coefficients: CramerCoefficients,
    rhs: f64, // TODO: rename to "constant"
}

fn parse_input(input: &str) -> Vec<(HailStone, LineEquation)> {
    input
        .lines()
        .map(|line| {
            let hailstone = parse_line(line);
            let line_equation = line_equation(&hailstone);
            (hailstone, line_equation)
        })
        .collect()
}

fn parse_line(line: &str) -> HailStone {
    let (position, velocity) = line.split_once(" @ ").unwrap();
    let mut position = position.split(", ");
    let mut velocity = velocity.split(", ");
    HailStone {
        x: position.next().unwrap().parse().unwrap(),
        y: position.next().unwrap().parse().unwrap(),
        z: position.next().unwrap().parse().unwrap(),
        x_velocity: velocity.next().unwrap().trim().parse().unwrap(),
        y_velocity: velocity.next().unwrap().trim().parse().unwrap(),
        z_velocity: velocity.next().unwrap().trim().parse().unwrap(),
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
struct HailStone {
    // TODO: rename to HailStone, or rename variables to hail_stone
    x: f64,
    y: f64,
    z: f64,
    x_velocity: f64,
    y_velocity: f64,
    z_velocity: f64,
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
            &HailStone {
                x: 19.0,
                y: 13.0,
                z: 30.,
                x_velocity: -2.0,
                y_velocity: 1.0,
                z_velocity: -2.,
            },
            &HailStone {
                x: 18.0,
                y: 19.0,
                z: 22.,
                x_velocity: -1.0,
                y_velocity: -1.0,
                z_velocity: -2.,
            },
            &HailStone {
                x: 20.0,
                y: 25.0,
                z: 34.,
                x_velocity: -2.0,
                y_velocity: -2.0,
                z_velocity: -4.,
            },
            &HailStone {
                x: 12.0,
                y: 31.0,
                z: 28.,
                x_velocity: -1.0,
                y_velocity: -2.0,
                z_velocity: -1.,
            },
            &HailStone {
                x: 20.0,
                y: 19.0,
                z: 15.,
                x_velocity: 1.0,
                y_velocity: -5.0,
                z_velocity: -3.,
            },
        ];
        assert_eq!(
            expected,
            parse_input(&input).iter().map(|x| &x.0).collect::<Vec<_>>()
        ); // TODO:
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
        let expected = HailStone {
            x: 19.0,
            y: 13.0,
            z: 30.,
            x_velocity: -2.0,
            y_velocity: 1.0,
            z_velocity: -2.,
        };
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_within_test_area() {
        let test_area = TestArea {
            start_x: 7.0,
            end_x: 27.0,
            start_y: 7.0,
            end_y: 27.0,
        };
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
        let hailstone = HailStone {
            x: 19.0,
            y: 13.0,
            z: 30.,
            x_velocity: -2.0,
            y_velocity: 1.0,
            z_velocity: -2.,
        };
        let expected = LineEquation { m: -0.5, c: 22.5 };
        assert_eq!(expected, line_equation(&hailstone));
    }

    #[test]
    fn test_intersects() {
        // TODO: rename to intersection
        let hailstone_a = HailStone {
            x: 19.0,
            y: 13.0,
            z: 30.,
            x_velocity: -2.0,
            y_velocity: 1.0,
            z_velocity: -2.,
        };
        let hailstone_b = HailStone {
            x: 12.0,
            y: 31.0,
            z: 28.,
            x_velocity: -1.0,
            y_velocity: -2.0,
            z_velocity: -1.,
        };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        let expected_intersection = Point { x: 6.2, y: 19.4 };
        assert_eq!(
            expected_intersection,
            line_equation_a.intersects(&line_equation_b)
        );
    }

    #[test]
    fn test_in_past() {
        let hailstone_a = HailStone {
            x: 19.0,
            y: 13.0,
            z: 30.,
            x_velocity: -2.0,
            y_velocity: 1.0,
            z_velocity: -2.,
        };
        let hailstone_b = HailStone {
            x: 20.0,
            y: 19.0,
            z: 15.,
            x_velocity: 1.0,
            y_velocity: -5.0,
            z_velocity: -3.,
        };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        let intersection = line_equation_a.intersects(&line_equation_b);
        assert!(in_past(&intersection, &hailstone_a));
        assert!(!in_past(&intersection, &hailstone_b));
    }

    #[test]
    fn test_is_parallel_to() {
        let hailstone_a = HailStone {
            x: 18.0,
            y: 19.0,
            z: 22.,
            x_velocity: -1.0,
            y_velocity: -1.0,
            z_velocity: -2.,
        };
        let hailstone_b = HailStone {
            x: 20.0,
            y: 25.0,
            z: 34.,
            x_velocity: -2.0,
            y_velocity: -2.0,
            z_velocity: -4.,
        };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        assert!(line_equation_a.is_parallel_to(&line_equation_b));

        let hailstone_a = HailStone {
            x: 12.0,
            y: 31.0,
            z: 28.,
            x_velocity: -1.0,
            y_velocity: -2.0,
            z_velocity: -1.,
        };
        let hailstone_b = HailStone {
            x: 20.0,
            y: 19.0,
            z: 15.,
            x_velocity: 1.0,
            y_velocity: -5.0,
            z_velocity: -3.,
        };
        let line_equation_a = line_equation(&hailstone_a);
        let line_equation_b = line_equation(&hailstone_b);
        assert!(!line_equation_a.is_parallel_to(&line_equation_b));
    }
}

#[cfg(test)]
mod day_24_pt2_tests {
    use super::*;

    #[test]
    fn test_determinant_2x2() {
        let matrix = vec![vec![3., 9.], vec![3., -5.]];
        assert_eq!((3. * -5.) - (9. * 3.), determinant(&matrix));
    }

    #[test]
    fn test_determinant_3x3() {
        let matrix = vec![vec![3., 9., -8.], vec![2., -5., 1.], vec![-1., 12., -13.]];
        assert_eq!(232., determinant(&matrix));
    }

    #[test]
    fn test_determinant_6x6() {
        let matrix = vec![
            vec![3., 9., -8., 7., -3., 6.],
            vec![2., -5., 1., -2., 7., 4.],
            vec![-1., 12., -13., -4., 8., 11.],
            vec![3., 3., -6., 15., -9., 2.],
            vec![16., 5., 19., -3., 15., 2.],
            vec![1., 9., -3., -17., 18., 20.],
        ];
        assert_eq!(665160., determinant(&matrix));
    }

    #[test]
    fn test_replace_with_constants() {
        let matrix = vec![
            vec![2.0, 1.0, 0.0, 6.0, 1.0, 0.0],
            vec![-1.0, 0.0, 0.0, 1.0, 0.0, -8.0],
            vec![3.0, 0.0, 0.0, 12.0, -1.0, 0.0],
            vec![0.0, 0.0, -2.0, -1.0, 0.0, 4.0],
            vec![3.0, 1.0, 0.0, 18.0, 7.0, 0.0],
            vec![-1.0, 0.0, 1.0, 7.0, 0.0, -2.0],
        ];
        let constants = vec![44.0, -4.0, 35.0, -74.0, 38.0, 6.0];
        let column_index = 0;
        let matrix = replace_with_constants(&matrix, &constants, column_index);
        for (i, constant) in constants.iter().enumerate() {
            assert_eq!(*constant, matrix[i][column_index]);
        }
    }
}
