use std::collections::HashSet;
use std::fs;

pub fn run() {
    let sketch = fs::read_to_string("10.txt").unwrap();
    println!("pt1: {:?}", steps_to_farthest_point(&sketch));
    println!("pt2: {:?}", enclosed_tile_count(&sketch));
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn _remove_non_loop_pipes(
    sketch: &Vec<Vec<char>>,
    loop_positions: &HashSet<[usize; 2]>,
) -> Vec<Vec<char>> {
    sketch
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tile)| {
                    if loop_positions.contains(&[i, j]) {
                        *tile
                    } else {
                        '.'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn _debug_print_loop(sketch: &Vec<Vec<char>>, loop_positions: &HashSet<[usize; 2]>) {
    let _debug_sketch = _remove_non_loop_pipes(&sketch, &loop_positions);
    let _debug_sketch = _debug_sketch
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{_debug_sketch}");
}

fn steps_to_farthest_point(sketch: &str) -> usize {
    let mut sketch: Vec<Vec<char>> = sketch.lines().map(|l| l.chars().collect()).collect();
    let (s_pos, s_symbol) = find_start(&sketch);
    sketch[s_pos[0]][s_pos[1]] = s_symbol;
    let loop_positions = loop_positions(&sketch, s_pos);

    // _debug_print_loop(&sketch, &loop_positions);

    loop_positions.len() / 2
}

fn enclosed_tile_count(sketch: &str) -> u64 {
    let mut sketch: Vec<Vec<char>> = sketch.lines().map(|l| l.chars().collect()).collect();
    let (s_pos, s_symbol) = find_start(&sketch);
    sketch[s_pos[0]][s_pos[1]] = s_symbol;
    let loop_positions = loop_positions(&sketch, s_pos);

    // _debug_print_loop(&sketch, &loop_positions);

    let mut count = 0;
    for i in 0..sketch.len() {
        for j in 0..sketch[i].len() {
            if is_enclosed(&sketch, &loop_positions, [i, j]) {
                // println!("enclosed: {:?}", [i, j]);
                count += 1;
            }
        }
    }

    count
}

fn peek(direction: Dir, sketch: &Vec<Vec<char>>, pos: [usize; 2]) -> Option<char> {
    let mut i: i64 = pos[0].try_into().unwrap();
    let mut j: i64 = pos[1].try_into().unwrap();

    match direction {
        Dir::Up => i -= 1,
        Dir::Down => i += 1,
        Dir::Left => j -= 1,
        Dir::Right => j += 1,
    }

    let max_i = sketch.len() as i64;
    let max_j = sketch.first().unwrap().len() as i64;

    if i == max_i || i < 0 || j == max_j || j < 0 {
        return None;
    }

    let char = sketch[i as usize][j as usize];

    match (direction, char) {
        (Dir::Up, '|' | 'F' | '7') => Some(char),
        (Dir::Down, '|' | 'L' | 'J') => Some(char),
        (Dir::Left, '-' | 'F' | 'L') => Some(char),
        (Dir::Right, '-' | '7' | 'J') => Some(char),
        _ => None,
    }
}

fn find_start(sketch: &Vec<Vec<char>>) -> ([usize; 2], char) {
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

    let adjacent_symbols = [
        peek(Dir::Right, &sketch, s_pos),
        peek(Dir::Left, &sketch, s_pos),
        peek(Dir::Up, &sketch, s_pos),
        peek(Dir::Down, &sketch, s_pos),
    ];

    let char = match adjacent_symbols {
        [Some(_), None, Some(_), None] => 'L',
        [Some(_), None, None, Some(_)] => 'F',
        [None, Some(_), None, Some(_)] => '7',
        [None, Some(_), Some(_), None] => 'J',
        [None, None, Some(_), Some(_)] => '|',
        [Some(_), Some(_), None, None] => '-',
        _ => panic!(
            "Invalid adjacent tiles for start point: {:?}",
            adjacent_symbols
        ),
    };

    (s_pos, char)
}

fn loop_positions(sketch: &Vec<Vec<char>>, s_pos: [usize; 2]) -> HashSet<[usize; 2]> {
    let s_symbol = sketch[s_pos[0]][s_pos[1]];

    let mut direction = match s_symbol {
        '|' => Dir::Up,
        '-' => Dir::Right,
        '7' => Dir::Down,
        'J' => Dir::Left,
        'F' => Dir::Down,
        'L' => Dir::Up,
        _ => panic!("Unexpected start symbol: {:?}", s_symbol),
    };

    let [mut i, mut j] = s_pos;

    let mut positions = HashSet::new();

    loop {
        positions.insert([i, j]);

        match direction {
            Dir::Right => j += 1,
            Dir::Left => j -= 1,
            Dir::Up => i -= 1,
            Dir::Down => i += 1,
        }

        if i == s_pos[0] && j == s_pos[1] {
            break;
        }

        let symbol = sketch[i][j];

        direction = match (symbol, &direction) {
            ('-', _) => direction,
            ('|', _) => direction,
            ('7', Dir::Right) => Dir::Down,
            ('7', Dir::Up) => Dir::Left,
            ('J', Dir::Right) => Dir::Up,
            ('J', Dir::Down) => Dir::Left,
            ('L', Dir::Left) => Dir::Up,
            ('L', Dir::Down) => Dir::Right,
            ('F', Dir::Left) => Dir::Down,
            ('F', Dir::Up) => Dir::Right,
            _ => panic!(
                "Unrecognised symbol/direction: {:?}/{:?}",
                symbol, direction
            ),
        };
    }

    positions
}

fn is_enclosed(
    sketch: &Vec<Vec<char>>,
    loop_positions: &HashSet<[usize; 2]>,
    pos: [usize; 2],
) -> bool {
    if loop_positions.contains(&pos) {
        return false;
    }
    let i = pos[0];
    let mut j = 0;
    let mut intersect_count = 0;
    while j < pos[1] {
        let char = sketch[i][j];
        if char == '|' || char == 'F' || char == '7' || char == 'S' {
            if loop_positions.contains(&[i, j]) {
                intersect_count += 1;
            }
        }
        j += 1
    }
    intersect_count % 2 != 0
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

#[cfg(test)]
mod loop_positions_tests {
    use super::*;

    #[test]
    fn returns_positions_of_pipes_in_the_loop() {
        let sketch = "....
.S7.
.LJ.
....";

        let mut sketch: Vec<Vec<char>> = sketch.lines().map(|l| l.chars().collect()).collect();
        let (s_pos, s_symbol) = find_start(&sketch);
        sketch[s_pos[0]][s_pos[1]] = s_symbol;
        let positions = loop_positions(&sketch, s_pos);
        let mut expected = HashSet::new();
        expected.insert([1, 1]);
        expected.insert([1, 2]);
        expected.insert([2, 2]);
        expected.insert([2, 1]);
        assert_eq!(expected, positions);
    }
}

#[cfg(test)]
mod enclosed_tile_count_tests {
    use super::*;

    #[test]
    fn single_enclosed_area() {
        let sketch = "......
.S--7.
.|..|.
.|..|.
.L--J.
......";

        assert_eq!(4, enclosed_tile_count(&sketch));
    }

    #[test]
    fn two_enclosed_areas() {
        let sketch = "...........
.S-------7.
.|..F-7..|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(8, enclosed_tile_count(&sketch));
    }

    #[test]
    fn no_enclosed_area() {
        let sketch = "......
.S--7.
.|F7|.
.||||.
.LJLJ.
......";

        assert_eq!(0, enclosed_tile_count(&sketch));
    }

    #[test]
    fn example_input_1() {
        let sketch = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(4, enclosed_tile_count(&sketch));
    }

    #[test]
    fn example_input_2() {
        let sketch = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(4, enclosed_tile_count(&sketch));
    }

    #[test]
    fn example_input_3() {
        let sketch = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(8, enclosed_tile_count(&sketch));
    }

    #[test]
    fn example_input_4() {
        let sketch = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(10, enclosed_tile_count(&sketch));
    }

    #[test]
    fn example_input_5() {
        let sketch = ".F7FSF7F7F7F7F7F---7
.|LJ||||||||||||F--J
.L-7LJLJ||||||LJL-7.
F--JF--7||LJLJ.F7FJ.
L---JF-JLJ....FJLJ..
...F-JF---7...L7....
..FJF7L7F-JF7..L---7
..L-JL7||F7|L7F-7F7|
.....FJ|||||FJL7||LJ
.....L-JLJLJL--JLJ..";

        assert_eq!(10, enclosed_tile_count(&sketch));
    }
}
