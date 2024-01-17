use std::fs;

pub fn run() {
    let contents = fs::read_to_string("14.txt").unwrap();
    println!("pt1: {}", pt1(&contents));

    let cycles = 1_000_000_000;

    println!("pt2: {}", pt2(&contents, cycles));
}

fn pt1(contents: &str) -> usize {
    let mut answer = 0;

    let mut parsed_platform = parse(contents);

    tilt_north(&mut parsed_platform);

    for col_index in 0..parsed_platform[0].len() {
        answer += col_load(col_index, &parsed_platform);
    }

    answer
}

fn pt2(contents: &str, cycles: usize) -> usize {
    let mut answer = 0;

    let mut parsed_platform = parse(contents);

    for _ in 0..cycles {
        let original_platform = parsed_platform.clone();
        tilt_north(&mut parsed_platform);
        tilt_west(&mut parsed_platform);
        tilt_south(&mut parsed_platform);
        tilt_east(&mut parsed_platform);
        if original_platform == parsed_platform {
            break
        }
    }

    println!("{:?}", parsed_platform);

    for col_index in 0..parsed_platform[0].len() {
        answer += col_load(col_index, &parsed_platform);
    }

    answer
}

fn parse(contents: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in contents.lines() {
        let line = line.chars().collect();
        grid.push(line);
    }

    grid
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut transposed: Vec<Vec<char>> = vec![];

    for i in 0..width {
        let mut row = vec![];
        for j in 0..height {
            row.push(grid[height - j - 1][i]);
        }
        transposed.push(row);
    }

    transposed
}

fn transpose_anti_clockwise(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let transposed = transpose(grid);
    let transposed = transpose(&transposed);
    transpose(&transposed)
}

fn tilt_east(platform: &mut Vec<Vec<char>>) -> () {
    for i in 0..platform.len() {
        tilt_row_east(&mut platform[i]);
    }
}

fn tilt_west(platform: &mut Vec<Vec<char>>) -> () {
    for i in 0..platform.len() {
        tilt_row_west(&mut platform[i]);
    }
}

fn tilt_north(platform: &mut Vec<Vec<char>>) -> () {
    for col_index in 0..platform[0].len() {
        tilt_col_north(col_index, platform);
    }
}

fn tilt_south(platform: &mut Vec<Vec<char>>) -> () {
    for col_index in 0..platform[0].len() {
        for row_index in 0..platform.len() {
            let char = platform[row_index][col_index];
            if char != '.' && char != '#' {
                let mut dot_position = None;
                let mut search_index = row_index + 1;
                while search_index < platform.len() && platform[search_index][col_index] != '#' {
                    if platform[search_index][col_index] == '.' {
                        dot_position = Some(search_index);
                    }
                    search_index += 1;
                }
                if let Some(search_index) = dot_position {
                    platform[search_index][col_index] = 'O';
                    platform[row_index][col_index] = '.';
                }
            }
        }
    }
}

fn tilt_col_north(col_index: usize, platform: &mut Vec<Vec<char>>) -> () {
    for i in (1..platform.len()).rev() {
        let char = platform[i][col_index];
        if char != '.' && char != '#' {
            let mut dot_position = None;
            let mut j = i;
            while j > 0 && platform[j][col_index] != '#' {
                j -= 1;
                if platform[j][col_index] == '.' {
                    dot_position = Some(j);
                }
            }
            if let Some(j) = dot_position {
                platform[i][col_index] = '.';
                platform[j][col_index] = 'O';
            }
        }
    }
}

fn tilt_row_east(row: &mut Vec<char>) -> () {
    for i in 0..row.len() {
        let char = row[i];
        if char != '.' && char != '#' {
            let mut dot_position = None;
            let mut j = i + 1;
            while j < row.len() && row[j] != '#' {
                if row[j] == '.' {
                    dot_position = Some(j);
                }
                j += 1;
            }
            if let Some(j) = dot_position {
                row[i] = '.';
                row[j] = 'O';
            }
        }
    }
}

fn tilt_row_west(row: &mut Vec<char>) -> () {
    for i in 0..row.len() {
        let char = row[row.len() - 1 - i];
        if char != '.' && char != '#' {
            let mut dot_position = None;
            let mut j = i;
            while j > 0 && row[j] != '#' {
                j -= 1;
                if row[j] == '.' {
                    dot_position = Some(j);
                }
            }
            if let Some(j) = dot_position {
                row[i] = '.';
                row[j] = 'O';
            }
        }
    }
}

fn row_load(row: &Vec<char>) -> usize {
    let mut answer = 0;
    for i in 0..row.len() {
        if row[i] == 'O' {
            answer += i + 1;
        }
    }
    answer
}

fn col_load(col_index: usize, platform: &Vec<Vec<char>>) -> usize {
    let mut answer = 0;
    for i in 0..platform.len() {
        let row = &platform[i];
        if row[col_index] == 'O' {
            answer += platform.len() - i;
        }
    }
    answer
}

#[cfg(test)]
mod day_14_pt1_tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let platform = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let parsed_platform = parse(platform);

        let transposed_platform = transpose(&parsed_platform);

        let expected = vec![
            vec!['#', '#', '.', '.', 'O', '.', 'O', '.', 'O', 'O'],
            vec!['O', '.', '.', '.', '.', 'O', 'O', '.', '.', '.'],
            vec!['O', '.', '.', 'O', '#', '.', '.', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'O', '.', '#', '.'],
            vec!['#', '#', '.', '#', 'O', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', 'O', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', 'O', '.', '#', 'O', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'O', '#', '.', 'O', '.', '#', '.'],
        ];

        assert_eq!(expected, transposed_platform);
    }

    #[test]
    fn test_tilt_row() {
        let mut rows = vec![
            vec!['#', '#', '.', '.', 'O', '.', 'O', '.', 'O', 'O'],
            vec!['O', '.', '.', '.', '.', 'O', 'O', '.', '.', '.'],
            vec!['O', '.', '.', 'O', '#', '.', '.', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'O', '.', '#', '.'],
            vec!['#', '#', '.', '#', 'O', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', 'O', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', 'O', '.', '#', 'O', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'O', '#', '.', 'O', '.', '#', '.'],
        ];

        let expected_rows = vec![
            vec!['#', '#', '.', '.', '.', '.', 'O', 'O', 'O', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', 'O', 'O'],
            vec!['.', '.', 'O', 'O', '#', '.', '.', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', '#', '.'],
            vec!['#', '#', '.', '#', '.', '.', 'O', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', 'O', '#', '.', '.'],
            vec!['.', '#', '.', 'O', '#', '.', '.', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'O', '#', '.', '.', 'O', '#', '.'],
        ];

        for i in 0..rows.len() {
            tilt_row_east(&mut rows[i]);
        }

        assert_eq!(expected_rows, rows);
    }

    #[test]
    fn test_row_load() {
        let row = vec!['#', '#', '.', '.', '.', '.', 'O', 'O', 'O', 'O'];
        assert_eq!(34, row_load(&row));
    }

    #[test]
    fn test_pt1() {
        let platform = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(136, pt1(platform));
    }

    #[test]
    fn test_pt1_b() {
        let platform = "O.
O.
..
OO
.O
O.
..
..
#.
#O";

        let expected = 10 + 9 + 8 + 7 + 10 + 9 + 8;

        assert_eq!(expected, pt1(platform));
    }

    #[test]
    fn test_pt1_c() {
        let platform = "O
O
.
O
.
O
.
.
#
#";

        let expected = 10 + 9 + 8 + 7;

        assert_eq!(expected, pt1(platform));
    }

    #[test]
    fn test_pt1_d() {
        let platform = ".
.
.
O
O
.
.
.
.
O";

        let expected = 10 + 9 + 8;

        assert_eq!(expected, pt1(platform));
    }
}

#[cfg(test)]
mod day_14_pt2_tests {
    use super::*;

    #[test]
    fn test_pt2() {
        let platform = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(64, pt2(platform, 1_000_000_000));
    }

//     #[test]
//     fn test_pt2_b() {
//         let platform = "O..
// O.O
// ...
// OO.
// .O.
// O.#
// ..O
// ...
// #..
// #OO";
//         let expected = 10 + 9 + 8 + 7 + 10 + 9 + 8 + 10 + 4 + 3;
//
//         assert_eq!(expected, pt2(platform));
//     }
//
//     #[test]
//     fn test_pt2_c() {
//         let platform = "O
// O
// .
// O
// .
// O
// .
// .
// #
// #";
//
//         let expected = 10 + 9 + 8 + 7;
//         assert_eq!(expected, pt2(platform));
//     }
//
//     #[test]
//     fn test_pt2_d() {
//         let platform = "O";
//         let expected = 1;
//         assert_eq!(expected, pt2(platform));
//     }

    #[test]
    fn test_tilt_north() {
        let mut platform = vec![
            vec!['O', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            vec!['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O'],
            vec!['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.'],
            vec!['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.'],
        ];

        let expected = vec![
            vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
            vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
            vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
            vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        ];

        tilt_north(&mut platform);

        assert_eq!(expected, platform);
    }

    #[test]
    fn test_col_north() {
        let mut platform = vec![
            vec!['O', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            vec!['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O'],
            vec!['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.'],
            vec!['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.'],
        ];

        let expected = vec![
            vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
            vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
            vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
            vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        ];

        for i in 0..platform[0].len() {
            tilt_col_north(i, &mut platform);
            let expected: Vec<_> = expected.iter().map(|row| row[i]).collect();
            let actual: Vec<_> = platform.iter().map(|row| row[i]).collect();
            assert_eq!(expected, actual);
        }
    }
}
