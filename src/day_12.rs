use std::fs;

pub fn run() {
    let contents = fs::read_to_string("12.txt").unwrap();
    println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
}

fn valid(springs: &Vec<char>, pattern: &Vec<usize>) -> bool {
    let mut broken_count = 0;
    let mut pattern_index = 0;

    for char in springs {
        let char = *char;
        if char == '#' {
            if pattern_index == pattern.len() {
                return false
            }
            broken_count += 1;
        } else if broken_count > 0 { 
            if broken_count != pattern[pattern_index] {
                return false
            }

            broken_count = 0;
            pattern_index += 1;
        }
    }

    if broken_count > 0 && pattern_index < pattern.len() {
        if broken_count != pattern[pattern_index] {
            return false
        }

        broken_count = 0;
        pattern_index += 1;
    }

    broken_count == 0 && pattern_index == pattern.len()
}

fn score(springs: &Vec<char>, pattern: &Vec<usize>, index: usize) -> usize {
    if index == springs.len() {
        if valid(springs, pattern) {
            return 1
        } else {
            return 0
        }
    } else {
        let char = springs[index];

        if char == '?' {
            let mut answer = 0;

            let mut springs1 = springs.clone();
            springs1[index] = '#';

            answer += score(&mut springs1, &pattern, index + 1);

            let mut springs2 = springs.clone();
            springs2[index] = '.';

            answer += score(&mut springs2, &pattern, index + 1);

            return answer
        } else {
            return score(springs, &pattern, index + 1);
        }
    }
}

fn pt1(contents: &str) -> usize {
    let mut answer = 0;

    for line in contents.lines() {
        let (springs, pattern) = line.split_once(' ').unwrap();
        let mut springs: Vec<_> = springs.chars().collect();
        let pattern = pattern.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let s = score(&mut springs, &pattern, 0);
        // println!("{} = {}", line, s);
        answer += s;
    }

    answer
}

fn pt2(contents: &str) -> usize {
    0
}

#[cfg(test)]
mod day_12_pt_1_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(21, pt1(example_input));
    }

    #[test]
    fn test_score_for_last_line_of_example_input() {
        let springs = vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'];
        let pattern = vec![3, 2, 1];

        assert_eq!(10, score(&springs, &pattern, 0));
    }

    #[test]
    fn test_valid() {
        let pattern = vec![3, 2, 1];

        let invalid_permutations = vec![
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '#', '.', '#', '#'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '#', '.', '.', '#'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '.', '#'],
            vec!['.', '#', '#', '#', '.', '.', '#', '#', '.', '#', '.', '#'],
            vec!['.', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#'],
        ];

        let valid_permutations = vec![
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '#', '.', '.', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '.', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '#', '#', '.', '.', '#', '#', '.', '#', '.', '.'],
            vec!['.', '#', '#', '#', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '.', '#', '#', '.', '.', '.', '#'],
            vec!['.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '#'],
            vec!['.', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '#'],

        ];

        for permutation in invalid_permutations {
            assert!(!valid(&permutation, &pattern));
        }

        for permutation in valid_permutations {
            assert!(valid(&permutation, &pattern));
        }
    }
}

