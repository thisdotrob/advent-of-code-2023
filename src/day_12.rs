use std::fs;
use std::collections::HashMap;

pub fn run() {
    let contents = fs::read_to_string("12.txt").unwrap();
    println!("pt1: {}", pt1(&contents));
    println!("pt2: {}", pt2(&contents));
}

fn valid(pattern: &Vec<usize>, pattern_index: usize, broken_count: usize) -> bool {
    if pattern_index == pattern.len() && broken_count == 0 {
        return true
    }

    if pattern_index == pattern.len() - 1 {
        return pattern[pattern_index] == broken_count
    }

    false
}

fn score(springs: &Vec<char>, pattern: &Vec<usize>, memo_key: [usize; 3], memo: &mut HashMap<[usize; 3], usize>) -> usize {
    if let Some(answer) = memo.get(&memo_key) {
        return *answer
    }

    let [springs_index, pattern_index, broken_count] = memo_key;

    if springs_index == springs.len() {
        if valid(pattern, pattern_index, broken_count) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut answer = 0;

    let spring = springs[springs_index];

    if spring == '.' || spring == '?' {
        if broken_count == 0 {
            let next_memo_key = [springs_index + 1, pattern_index, broken_count];
            answer += score(springs, pattern, next_memo_key, memo);
        } else if pattern_index < pattern.len() && pattern[pattern_index] == broken_count {
            let next_memo_key = [springs_index + 1, pattern_index + 1, 0];
            answer += score(springs, pattern, next_memo_key, memo);
        }
    }

    if spring == '#' || spring == '?' {
        let next_memo_key = [springs_index + 1, pattern_index, broken_count + 1];
        answer += score(springs, pattern, next_memo_key, memo);
    }

    memo.insert(memo_key, answer);

    answer
}

fn pt1(contents: &str) -> usize {
    let mut answer = 0;

    for line in contents.lines() {
        let (springs, pattern) = line.split_once(' ').unwrap();
        let springs: Vec<_> = springs.chars().collect();
        let pattern = pattern.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        let mut memo = HashMap::new();
        
        answer += score(&springs, &pattern, [0, 0, 0], &mut memo);
    }

    answer
}

fn pt2(contents: &str) -> usize {
    let mut answer = 0;

    for line in contents.lines() {
        let (springs, pattern) = line.split_once(' ').unwrap();

        let springs = [springs; 5].join("?");
        let springs: Vec<_> = springs.chars().collect();

        let pattern = [pattern; 5].join(",");
        let pattern = pattern.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        let mut memo = HashMap::new();

        answer += score(&springs, &pattern, [0, 0, 0], &mut memo);
    }

    answer
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
}

#[cfg(test)]
mod day_12_pt_2_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(525152, pt2(example_input));
    }
}
