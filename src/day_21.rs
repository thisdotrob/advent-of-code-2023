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

fn start_coord<const N: usize>(garden: &Garden<N>) -> Coord {
    for row_index in 0..N {
        let row = garden.map[row_index];
        for col_index in 0..N {
            let char = row[col_index];
            if char == 'S' {
                return Coord(col_index, row_index);
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

    let symbol = garden.map[coord.1][coord.0];

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

    if x < N - 1 {
        result.push(Coord(x + 1, y));
    }

    if y < N - 1 {
        result.push(Coord(x, y + 1));
    }
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
struct Coord(usize, usize);

type StepCount = u64;

type Seen = HashSet<(Coord, StepCount)>;
