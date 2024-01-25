use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("21_example.txt").unwrap();
    println!("pt1 example: {}", pt1::<11>(&input, 6));
    let input = fs::read_to_string("21.txt").unwrap();
    println!("pt1: {}", pt1::<131>(&input, 64));
}

fn pt1<const N: usize>(input: &str, steps_left: u64) -> usize {
    let garden: Garden<N> = Garden::from(input);

    let start_coord = start_coord(&garden);

    let mut seen: Seen = HashSet::new();

    let coords = reachable_coords(&garden, &start_coord, steps_left, &mut seen);

    coords.len()
}


fn pt2<const N: usize>(input: &str, steps_left: u64) -> usize {
    let garden: Garden<N> = Garden::from(input);

    let start_coord = start_coord(&garden);

    let mut seen: Seen = HashSet::new();

    let coords = reachable_coords_infinite_garden(&garden, &start_coord, steps_left, &mut seen);

    coords.len()
}

fn start_coord<const N: usize>(garden: &Garden<N>) -> Coord {
    for row_index in 0..N {
        let row = garden.map[row_index as usize];
        for col_index in 0..N {
            let char = row[col_index as usize];
            if char == 'S' {
                return Coord(
                    col_index.try_into().unwrap(),
                    row_index.try_into().unwrap(),
                );
            }
        }
    }
    panic!("Start coord not found");
}

fn reachable_coords<const N: usize>(
    garden: &Garden<N>,
    coord: &Coord,
    steps_left: StepCount,
    seen: &mut Seen,
) -> HashSet<Coord> {
    let mut result: HashSet<Coord> = HashSet::new();

    if seen.contains(&(*coord, steps_left)) {
        return result;
    }

    let symbol = garden.map[coord.1 as usize][coord.0 as usize];

    if symbol == '.' || symbol == 'S' {
        if steps_left == 0 {
            result.insert(*coord);
        } else {
            for coord in neighbour_coords::<N>(coord) {
                let reachable_coords = reachable_coords(garden, &coord, steps_left - 1, seen);

                result.extend(reachable_coords);
            }
        }
    }

    seen.insert((*coord, steps_left));

    result
}

fn neighbour_coords<const N: usize>(coord: &Coord) -> Vec<Coord> {
    let (x, y) = (coord.0, coord.1);

    let mut result = vec![];

    if x > 0 {
        result.push(Coord(x - 1, y));
    }

    if y > 0 {
        result.push(Coord(x, y - 1));
    }

    if x < (N - 1).try_into().unwrap() {
        result.push(Coord(x + 1, y));
    }

    if y < (N - 1).try_into().unwrap() {
        result.push(Coord(x, y + 1));
    }

    result
}

fn reachable_coords_infinite_garden<const N: usize>(
    garden: &Garden<N>,
    coord: &Coord,
    steps_left: StepCount,
    seen: &mut Seen,
) -> HashSet<Coord> {
    let mut result: HashSet<Coord> = HashSet::new();

    if seen.contains(&(*coord, steps_left)) {
        return result;
    }

    let x = if coord.0 < 0 {
        let xx = (coord.0.abs() as usize % N);
        if xx == 0 {
            0
        } else {
            N - xx
        }
    } else if coord.0 > 0 {
        coord.0 as usize % N
    } else {
        coord.0 as usize
    };

    let y = if coord.1 < 0 {
        let yy = (coord.1.abs() as usize % N);
        if yy == 0 {
            0
        } else {
            N - yy
        }
    } else if coord.1 > 0 {
        coord.1 as usize % N
    } else {
        coord.1 as usize
    };
    
    println!("{:?} coord", coord);
    println!("{:?} (x, y)", (x, y));

    let symbol = garden.map[y][x];

    if symbol == '.' || symbol == 'S' {
        if steps_left == 0 {
            result.insert(*coord);
        } else {
            for coord in neighbour_coords_infinite_garden::<N>(coord) {
                let reachable_coords = reachable_coords_infinite_garden(garden, &coord, steps_left - 1, seen);

                result.extend(reachable_coords);
            }
        }
    }

    seen.insert((*coord, steps_left));

    result
}

fn neighbour_coords_infinite_garden<const N: usize>(coord: &Coord) -> Vec<Coord> {
    let (x, y) = (coord.0, coord.1);

    let mut result = vec![];

    result.push(Coord(x - 1, y));
    result.push(Coord(x, y - 1));
    result.push(Coord(x + 1, y));
    result.push(Coord(x, y + 1));

    result
}

struct Garden<const N: usize> {
    map: [[char; N]; N],
}

impl<const N: usize> From<&str> for Garden<N> {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().map(|l| l.chars());

        Self {
            map: std::array::from_fn(|_| {
                let mut line = lines.next().unwrap();
                std::array::from_fn(|_| line.next().unwrap())
            }),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord(isize, isize);

type StepCount = u64;

type Seen = HashSet<(Coord, StepCount)>;

#[cfg(test)]
mod day_21_pt_2_tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = fs::read_to_string("21_example.txt").unwrap();
        let answer = pt2::<11>(&input, 6);
        assert_eq!(16, answer);
    }

    #[test]
    fn test_example_2() {
        let input = fs::read_to_string("21_example.txt").unwrap();
        let answer = pt2::<11>(&input, 10);
        assert_eq!(50, answer);
    }

    #[test]
    fn test_example_3() {
        let input = fs::read_to_string("21_example.txt").unwrap();
        let answer = pt2::<11>(&input, 50);
        assert_eq!(1594, answer);
    }

    #[test]
    fn test_example_4() {
        let input = fs::read_to_string("21_example.txt").unwrap();
        let answer = pt2::<11>(&input, 100);
        assert_eq!(6536, answer);
    }

    // #[test]
    // fn test_example_5() {
    //     let input = fs::read_to_string("21_example.txt").unwrap();
    //     let answer = pt2::<11>(&input, 500);
    //     assert_eq!(167004, answer);
    // }

    // #[test]
    // fn test_example_6() {
    //     let input = fs::read_to_string("21_example.txt").unwrap();
    //     let answer = pt2::<11>(&input, 1000);
    //     assert_eq!(668697, answer);
    // }

    // #[test]
    // fn test_example_7() {
    //     let input = fs::read_to_string("21_example.txt").unwrap();
    //     let answer = pt2::<11>(&input, 5000);
    //     assert_eq!(16733044, answer);
    // }
}
