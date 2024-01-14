use std::fs;

pub fn run() {
    let contents = fs::read_to_string("13.txt").unwrap();
    println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
}

fn pt1(contents: &str) -> usize {
    let mut answer = 0;

    let patterns = parse_patterns(contents);

    for pattern in patterns {
        let lookup = col_reflection_key_lookup(&pattern);

        let num_cols = pattern[0].len(); // TODO replace with lookup length

        let mut reflected_col_index = None;

        for col_index in 0..num_cols - 1 {
            let mut i0 = col_index;
            let mut i1 = col_index + 1;

            while lookup[i0] == lookup[i1] {
                if i0 == 0 || i1 == num_cols - 1 {
                    reflected_col_index = Some(col_index);
                    break
                } else {
                    i0 -= 1;
                    i1 += 1;
                }
            }
        }

        if let Some(col_index) = reflected_col_index {
            answer += col_index + 1;
            continue
        }

        let lookup = row_reflection_key_lookup(&pattern);

        let num_rows = lookup.len();

        let mut reflected_row_index = None;

        for row_index in 0..num_rows - 1 {
            let mut i0 = row_index;
            let mut i1 = row_index + 1;

            while lookup[i0] == lookup[i1] {
                if i0 == 0 || i1 == num_rows - 1 {
                    reflected_row_index = Some(row_index);
                    break
                } else {
                    i0 -= 1;
                    i1 += 1;
                }
            }
        }

        if let Some(row_index) = reflected_row_index {
            answer += 100 * (row_index + 1);
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

fn col_reflection_key(pattern: &Vec<Vec<char>>, col_index: usize) -> String {
    let mut key = String::new();

    for line in pattern {
        key.push(line[col_index]);
    }

    key
}

fn row_reflection_key(pattern: &Vec<Vec<char>>, row_index: usize) -> String {
    let mut key = String::new();

    for char in &pattern[row_index] {
        key.push(*char);
    }

    key
}

fn col_reflection_key_lookup(pattern: &Vec<Vec<char>>) -> Vec<String> {
    let mut lookup = vec![];
    for col_index in 0..pattern[0].len() {
        lookup.push(col_reflection_key(&pattern, col_index));
    }
    lookup
}

fn row_reflection_key_lookup(pattern: &Vec<Vec<char>>) -> Vec<String> {
    let mut lookup = vec![];
    for row_index in 0..pattern.len() {
        lookup.push(row_reflection_key(&pattern, row_index));
    }
    lookup
}

#[cfg(test)]
mod day_13_pt1_tests {
    use super::*;

    #[test]
    fn test_col_reflection_key() {
        let pattern = vec![
            vec!['#', '.', '#'], 
            vec!['.', '.', '#'], 
            vec!['#', '#', '.'], 
            vec!['#', '#', '.'], 
            vec!['.', '.', '#'], 
            vec!['.', '.', '#'], 
            vec!['#', '.', '#']
        ];

        assert_eq!("#.##..#", col_reflection_key(&pattern, 0));
        assert_eq!("..##...", col_reflection_key(&pattern, 1));
        assert_eq!("##..###", col_reflection_key(&pattern, 2));
    }

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
    fn test_col_reflection_key_lookup() {
        let pattern = vec![
            vec!['#', '.', '#'], 
            vec!['.', '.', '#'], 
            vec!['#', '#', '.'], 
            vec!['#', '#', '.'], 
            vec!['.', '.', '#'], 
            vec!['.', '.', '#'], 
            vec!['#', '.', '#']
        ];

        let expected_lookup = vec![
            "#.##..#",
            "..##...",
            "##..###",
        ];

        assert_eq!(expected_lookup, col_reflection_key_lookup(&pattern));
    }

    #[test]
    fn test_row_reflection_key_lookup() {
        let pattern = vec![
            vec!['#', '.', '#'], 
            vec!['.', '.', '#'], 
            vec!['#', '#', '.'], 
        ];

        let expected_lookup = vec![
            "#.#",
            "..#",
            "##.",
        ];

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
