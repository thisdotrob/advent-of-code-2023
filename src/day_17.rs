use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("17_example.txt").unwrap();
    println!("pt1 example: {}", pt1::<13>(&example_input));
    let input = fs::read_to_string("17.txt").unwrap();
    println!("pt1: {}", pt1::<141>(&input));
}

fn pt1<const N: usize>(input: &str) -> usize {
    let heat_loss_map: HeatLossMap<N> = HeatLossMap::from(input);

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(PathState {
        heat_loss: 0,
        location: (0, 0),
        direction: Direction::Right,
        steps_in_direction: 1,
    });

    heap.push(PathState {
        heat_loss: 0,
        location: (0, 0),
        direction: Direction::Down,
        steps_in_direction: 1,
    });

    let mut minimum_heat_loss = usize::MAX;

    while let Some(state) = heap.pop() {
        if seen.contains(&(state.location, state.direction, state.steps_in_direction)) {
            continue;
        };

        if state.location == (N - 1, N - 1) {
            if state.heat_loss < minimum_heat_loss {
                minimum_heat_loss = state.heat_loss;
            }
            continue;
        }

        let (mut x, mut y) = state.location;

        match state.direction {
            Direction::Up => {
                if y == 0 {
                    continue;
                } else {
                    y -= 1
                }
            }
            Direction::Down => {
                if y == N - 1 {
                    continue;
                } else {
                    y += 1
                }
            }
            Direction::Left => {
                if x == 0 {
                    continue;
                } else {
                    x -= 1
                }
            }
            Direction::Right => {
                if x == N - 1 {
                    continue;
                } else {
                    x += 1
                }
            }
        }

        let heat_loss = state.heat_loss + heat_loss_map.blocks[y][x];

        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if state.direction == direction {
                if state.steps_in_direction < 3 {
                    heap.push(PathState {
                        heat_loss,
                        location: (x, y),
                        direction,
                        steps_in_direction: state.steps_in_direction + 1,
                    })
                }
            } else if state.direction != direction.opposite() {
                heap.push(PathState {
                    heat_loss,
                    location: (x, y),
                    direction,
                    steps_in_direction: 1,
                })
            }
        }
        seen.insert((state.location, state.direction, state.steps_in_direction));
    }

    minimum_heat_loss
}

struct HeatLossMap<const N: usize> {
    blocks: [[usize; N]; N],
}

impl<const N: usize> From<&str> for HeatLossMap<N> {
    fn from(s: &str) -> Self {
        let mut lines = s
            .lines()
            .map(|l| l.chars().map(|char| char.to_digit(10).unwrap() as usize));

        Self {
            blocks: std::array::from_fn(|_| {
                let mut line = lines.next().unwrap();
                std::array::from_fn(|_| line.next().unwrap())
            }),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Eq, PartialEq)]
struct PathState {
    heat_loss: usize,
    location: (usize, usize),
    direction: Direction,
    steps_in_direction: usize,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
