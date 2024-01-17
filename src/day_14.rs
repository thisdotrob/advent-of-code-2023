use std::fs;

pub fn run() {
    let contents = fs::read_to_string("14.txt").unwrap();
    println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
}

fn pt1(contents: &str) -> usize {
    let mut answer = 0;

    let parsed_platform = parse(contents);

    let mut transposed_platform = transpose(&parsed_platform);

    tilt_east(&mut transposed_platform);

    for row in &transposed_platform {
        answer += row_load(&row);
    }

    answer
}

fn pt2(_contents: &str) -> usize {
    let answer = 0;

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

fn tilt_east(platform: &mut Vec<Vec<char>>) -> () {
    for i in 0..platform.len() {
        tilt_row_east(&mut platform[i]);
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

fn row_load(row: &Vec<char>) -> usize {
    let mut answer = 0;
    for i in 0..row.len() {
        if row[i] == 'O' {
            answer += i + 1;
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
    fn test_tilt_row_east() {
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
}
