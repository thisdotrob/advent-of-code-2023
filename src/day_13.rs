use std::fs;

pub fn run() {
    let contents = fs::read_to_string("13.txt").unwrap();
    println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
}

fn points_of_reflection(pattern: &Vec<Vec<char>>) -> (Option<usize>, Option<usize>) {
    let horizontal_point_of_reflection = point_of_reflection_index(&pattern);

    let transposed_pattern = transpose_pattern(&pattern);

    let vertical_point_of_reflection = point_of_reflection_index(&transposed_pattern);

    (horizontal_point_of_reflection, vertical_point_of_reflection)
}

fn pt1(contents: &str) -> usize {
    let mut answer = 0;

    let patterns = parse_patterns(contents);

    for pattern in patterns {
        let (horizontal_point, vertical_point) = points_of_reflection(&pattern);

        if let Some(row_index) = horizontal_point {
            answer += 100 * (row_index + 1);
            continue;
        }

        if let Some(col_index) = vertical_point {
            answer += col_index + 1;
        }
    }

    answer
}

fn pt2(_contents: &str) -> usize {
    let answer = 0;
    answer
}

fn parse_patterns(contents: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns = vec![];

    let mut pattern: Vec<Vec<char>> = vec![];

    for line in contents.lines() {
        if line == "" {
            patterns.push(pattern);
            pattern = vec![];
        } else {
            let line = line.chars().collect();
            pattern.push(line);
        }
    }

    patterns.push(pattern);

    patterns
}

fn row_reflection_key(pattern: &Vec<Vec<char>>, row_index: usize) -> String {
    let mut key = String::new();

    for char in &pattern[row_index] {
        key.push(*char);
    }

    key
}

fn row_reflection_key_lookup(pattern: &Vec<Vec<char>>) -> Vec<String> {
    let mut lookup = vec![];
    for row_index in 0..pattern.len() {
        lookup.push(row_reflection_key(&pattern, row_index));
    }
    lookup
}

fn point_of_reflection_index(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let lookup = row_reflection_key_lookup(&pattern);

    let mut reflection_index = None;

    for row_index in 0..lookup.len() - 1 {
        let mut i0 = row_index;
        let mut i1 = row_index + 1;

        while lookup[i0] == lookup[i1] {
            if i0 == 0 || i1 == lookup.len() - 1 {
                reflection_index = Some(row_index);
                break;
            } else {
                i0 -= 1;
                i1 += 1;
            }
        }
    }

    reflection_index
}

fn transpose_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let pattern_height = pattern.len();
    let pattern_width = pattern[0].len();

    let mut transposed: Vec<Vec<char>> = vec![];

    for i in 0..pattern_width {
        let mut row = vec![];
        for j in 0..pattern_height {
            row.push(pattern[j][i]);
        }
        transposed.push(row);
    }

    transposed
}

#[cfg(test)]
mod day_13_pt1_tests {
    use super::*;

    #[test]
    fn test_row_reflection_key() {
        let pattern = vec![
            vec!['#', '.', '#'],
            vec!['.', '.', '#'],
            vec!['#', '#', '.'],
        ];

        assert_eq!("#.#", row_reflection_key(&pattern, 0));
        assert_eq!("..#", row_reflection_key(&pattern, 1));
        assert_eq!("##.", row_reflection_key(&pattern, 2));
    }

    #[test]
    fn test_row_reflection_key_lookup() {
        let pattern = vec![
            vec!['#', '.', '#'],
            vec!['.', '.', '#'],
            vec!['#', '#', '.'],
        ];

        let expected_lookup = vec!["#.#", "..#", "##."];

        assert_eq!(expected_lookup, row_reflection_key_lookup(&pattern));
    }

    #[test]
    fn test_parse_patterns() {
        let contents = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let patterns = parse_patterns(contents);

        assert_eq!(2, patterns.len());
    }

    #[test]
    fn test_pt1() {
        let contents = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(405, pt1(contents));
    }
}
