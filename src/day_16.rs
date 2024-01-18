use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("16_example.txt").unwrap();
    println!("pt1 example: {}", pt1::<10>(&example_input));
    let input = fs::read_to_string("16.txt").unwrap();
    println!("pt1: {}", pt1::<110>(&input));
}

fn pt1<const N: usize>(input: &str) -> usize {
    let mut contraption: Contraption<N> = Contraption::from(input);

    let start_coords = (0, 0);
    contraption.start_beam(start_coords, Direction::Right);

    contraption.total_energized_tiles()
}

struct Contraption<const N: usize> {
    tiles: [[char; N]; N],
    energized_tiles: [[bool; N]; N],
}

impl<const N: usize> Contraption<N> {
    fn start_beam(&mut self, start_coords: (usize, usize), start_direction: Direction) {
        let (mut x, mut y) = start_coords;

        let mut direction = start_direction;

        loop {
            let tile = self.tiles[y][x];

            let energized = self.energized_tiles[y][x];

            if let ('-' | '|', true) = (tile, energized) {
                // means a loop is about to start and the beam will never finish travelling
                break;
            }

            self.energized_tiles[y][x] = true;

            direction = match (tile, &direction) {
                ('.', _) => direction,
                ('\\', Direction::Up) => Direction::Left,
                ('\\', Direction::Down) => Direction::Right,
                ('\\', Direction::Left) => Direction::Up,
                ('\\', Direction::Right) => Direction::Down,
                ('/', Direction::Up) => Direction::Right,
                ('/', Direction::Down) => Direction::Left,
                ('/', Direction::Left) => Direction::Down,
                ('/', Direction::Right) => Direction::Up,
                ('|', Direction::Down | Direction::Up) => direction,
                ('|', Direction::Left | Direction::Right) => {
                    if y < N - 1 {
                        self.start_beam((x, y + 1), Direction::Down);
                    }
                    Direction::Up
                }
                ('-', Direction::Right | Direction::Left) => direction,
                ('-', Direction::Up | Direction::Down) => {
                    if x < N - 1 {
                        self.start_beam((x + 1, y), Direction::Right);
                    }
                    Direction::Left
                }
                _ => panic!("invalid tile"),
            };

            match direction {
                Direction::Up => {
                    if y == 0 {
                        break;
                    } else {
                        y -= 1
                    }
                }
                Direction::Down => {
                    if y == N - 1 {
                        break;
                    } else {
                        y += 1
                    }
                }
                Direction::Left => {
                    if x == 0 {
                        break;
                    } else {
                        x -= 1
                    }
                }
                Direction::Right => {
                    if x == N - 1 {
                        break;
                    } else {
                        x += 1
                    }
                }
            }
        }
    }

    fn total_energized_tiles(&self) -> usize {
        self.energized_tiles
            .map(|row| row.into_iter().filter(|is_energized| *is_energized).count())
            .iter()
            .sum()
    }
}

impl<const N: usize> From<&str> for Contraption<N> {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().map(|l| l.chars());

        Self {
            energized_tiles: [[false; N]; N],
            tiles: std::array::from_fn(|_| {
                let mut line = lines.next().unwrap();
                std::array::from_fn(|_| line.next().unwrap())
            }),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
