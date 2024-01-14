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
        let col_reflection_counts = col_reflection_counts(&pattern);
        let row_reflection_counts = row_reflection_counts(&pattern);

        if col_reflection_counts.iter().sum::<usize>() + row_reflection_counts.iter().sum::<usize>() == 0 {
            println!("YES");
            continue
        }

        let max_reflection_count_col_index = index_of_max(&col_reflection_counts);
        let max_reflection_count_row_index = index_of_max(&row_reflection_counts);

        if max_reflection_count_row_index > max_reflection_count_col_index {
            answer += 100 * (max_reflection_count_row_index + 1);
        } else {
            answer += max_reflection_count_col_index + 1;
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

fn col_reflection_counts(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let lookup = col_reflection_key_lookup(&pattern);

    let num_cols = pattern[0].len();

    let mut counts = vec![];

    for col_index in 0..num_cols {
        if !lookup[col_index].contains("#") {
            counts.push(0);
            continue
        }

        let mut count = 0;

        let mut i0 = col_index;
        let mut i1 = col_index + 1;

        while i1 < num_cols && lookup[i0] == lookup[i1] {
            if lookup[i0].contains("#") {
                count = i1 - col_index;
            }
            if i0 == 0 {
                break
            } else {
                i0 -= 1;
                i1 += 1;
            }
        }

        counts.push(count);
    }

    counts
}

fn row_reflection_counts(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let lookup = row_reflection_key_lookup(&pattern);

    let num_rows = pattern.len();

    let mut counts = vec![];

    for row_index in 0..num_rows {
        if !lookup[row_index].contains("#") {
            counts.push(0);
            continue
        }

        let mut count = 0;

        let mut i0 = row_index;
        let mut i1 = row_index + 1;

        while i1 < num_rows && lookup[i0] == lookup[i1] {
            if lookup[i0].contains("#") {
                count = i1 - row_index;
            }
            if i0 == 0 {
                break
            } else {
                i0 -= 1;
                i1 += 1;
            }
        }

        counts.push(count);
    }

    counts
}

fn index_of_max(v: &Vec<usize>) -> usize {
    let mut max = 0;

    for i in 1..v.len() {
        if v[i] > v[max] {
            max = i;
        }
    }

    max
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
    fn test_col_reflection_counts() {
        let pattern = vec![
            vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'], 
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'], 
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'], 
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'], 
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'], 
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'], 
            vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'], 
        ];

        let col_index_to_check = 4;
        let expected_reflection_count = 4;

        let reflection_counts_lookup = col_reflection_counts(&pattern);

        assert_eq!(expected_reflection_count, reflection_counts_lookup[col_index_to_check]);
    }

    #[test]
    fn test_reflection_counts_no_mirrors() {
        let pattern = vec![
            vec!['.', '.', '.'], 
            vec!['.', '.', '.'], 
            vec!['.', '.', '.'], 
        ];

        let reflection_counts_lookup = col_reflection_counts(&pattern);

        for i in 0..2 {
            assert_eq!(0, reflection_counts_lookup[i]);
        }

        let reflection_counts_lookup = row_reflection_counts(&pattern);

        for i in 0..2 {
            assert_eq!(0, reflection_counts_lookup[i]);
        }

    }

    #[test]
    fn test_reflection_counts_are_bounded_by_outer_mirrors() {
        let pattern = vec![
            vec!['.', '.', '.', '.'], 
            vec!['.', '#', '#', '.'], 
            vec!['.', '#', '#', '.'], 
            vec!['.', '.', '.', '.'], 
        ];

        let reflection_counts_lookup = col_reflection_counts(&pattern);

        assert_eq!(1, reflection_counts_lookup[1]);

        let reflection_counts_lookup = row_reflection_counts(&pattern);

        assert_eq!(1, reflection_counts_lookup[1]);
    }

    #[test]
    fn test_row_reflection_counts() {
        let pattern = vec![
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'], 
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'], 
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'], 
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'], 
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'], 
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'], 
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'], 
        ];

        let row_index_to_check = 3;
        let expected_reflection_count = 3;

        let reflection_counts_lookup = row_reflection_counts(&pattern);

        assert_eq!(expected_reflection_count, reflection_counts_lookup[row_index_to_check]);
    }

    #[test]
    fn test_index_of_max() {
        let v = vec![0, 4, 200, 3, 20];
        assert_eq!(2, index_of_max(&v));
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

    #[test]
    fn test_no_reflection() {
        let contents = "#.
..
##
..
..
#.

#.##..#
#..#..#";
        
        assert_eq!(0, pt1(contents));
    }
}
