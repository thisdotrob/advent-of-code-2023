use std::fs;

pub fn run() {
    let sketch = fs::read_to_string("10.txt").unwrap();
    println!("pt1: {:?}", steps_to_farthest_point(&sketch));
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn peek(direction: Direction, sketch: &Vec<Vec<char>>, pos: [usize; 2]) -> Option<char> {
    let mut i: i64 = pos[0].try_into().unwrap();
    let mut j: i64 = pos[1].try_into().unwrap();

    match direction {
        Direction::Up => i -= 1,
        Direction::Down => i += 1,
        Direction::Left => j -= 1,
        Direction::Right => j += 1,
    }

    let max_i = sketch.len() as i64;
    let max_j = sketch.first().unwrap().len() as i64;

    if i == max_i || i < 0 || j == max_j || j < 0 {
        None
    } else {
        Some(sketch[i as usize][j as usize])
    }
}

fn steps_to_farthest_point(sketch: &str) -> u64 {
    let sketch: Vec<Vec<char>> = sketch.lines().map(|l| l.chars().collect()).collect();
    let mut s_pos = None;

    'a: for i in 0..sketch.len() {
        let row = &sketch[i];
        for j in 0..row.len() {
            let tile = row[j];
            if tile == 'S' {
                s_pos = Some([i, j]);
                break 'a;
            }
        }
    }

    let s_pos = s_pos.unwrap();

    let mut direction = Direction::Down;

    let [mut i, mut j] = s_pos;

    let symbol_to_right = peek(Direction::Right, &sketch, s_pos);
    let symbol_to_left = peek(Direction::Left, &sketch, s_pos);
    let symbol_above = peek(Direction::Up, &sketch, s_pos);
    let _symbol_below = peek(Direction::Down, &sketch, s_pos);

    if let Some(symbol) = symbol_to_right {
        if ['-', '7', 'J']
            .iter()
            .any(|valid_symbol| *valid_symbol == symbol)
        {
            direction = Direction::Right;
        }
    }

    if let Some(symbol) = symbol_to_left {
        if ['-', 'L', 'F']
            .iter()
            .any(|valid_symbol| *valid_symbol == symbol)
        {
            direction = Direction::Left;
        }
    }

    if let Some(symbol) = symbol_above {
        if ['|', 'F', '7']
            .iter()
            .any(|valid_symbol| *valid_symbol == symbol)
        {
            direction = Direction::Up;
        }
    }

    let mut steps = 0;

    loop {
        steps += 1;

        match direction {
            Direction::Right => j += 1,
            Direction::Left => j -= 1,
            Direction::Up => i -= 1,
            Direction::Down => i += 1,
        }

        if i == s_pos[0] && j == s_pos[1] {
            break;
        }

        let symbol = sketch[i][j];

        direction = match (symbol, &direction) {
            ('-', _) => direction,
            ('|', _) => direction,
            ('7', Direction::Right) => Direction::Down,
            ('7', Direction::Up) => Direction::Left,
            ('J', Direction::Right) => Direction::Up,
            ('J', Direction::Down) => Direction::Left,
            ('L', Direction::Left) => Direction::Up,
            ('L', Direction::Down) => Direction::Right,
            ('F', Direction::Left) => Direction::Down,
            ('F', Direction::Up) => Direction::Right,
            _ => panic!(
                "Unrecognised symbol/direction: {:?}/{:?}",
                symbol, direction
            ), // TODO is there a way to make this exhaustive without using the
               // catch all?
        };
    }

    steps / 2
}

#[cfg(test)]
mod steps_to_farthest_point_tests {
    use super::*;

    #[test]
    fn square_loop_example() {
        let sketch = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, steps_to_farthest_point(sketch));
    }

    #[test]
    fn more_complex_loop_example() {
        let sketch = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, steps_to_farthest_point(sketch));
    }

    #[test]
    fn more_complex_loop_with_non_main_tiles_example() {
        let sketch = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(8, steps_to_farthest_point(sketch));
    }
}
