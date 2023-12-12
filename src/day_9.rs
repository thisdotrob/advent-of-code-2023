use std::fs;

pub fn run() {
    let contents = fs::read_to_string("9.txt").unwrap();

    let histories = contents.lines().map(|line| {
        let value_history: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        value_history
    });

    let answers = histories.fold([0, 0], |acc, value_history| {
        let next_value = predict_next_value(value_history.clone());
        let prev_value = predict_previous_value(value_history);
        [acc[0] + next_value, acc[1] + prev_value]
    });

    println!("pt1: {:?}", answers[0]);
    println!("pt2: {:?}", answers[1]);
}

fn predict_previous_value(mut value_history: Vec<i64>) -> i64 {
    value_history.reverse();
    predict_next_value(value_history)
}

fn predict_next_value(value_history: Vec<i64>) -> i64 {
    let mut differences = vec![value_history];

    loop {
        let windows: Vec<_> = differences.last().unwrap().windows(2).collect();

        let diffs: Vec<_> = windows.iter().map(|pair| pair[1] - pair[0]).collect();

        let all_zeroes = diffs.iter().all(|x| *x == 0);

        differences.push(diffs);

        if all_zeroes {
            break;
        }
    }

    while differences.len() > 1 {
        let val = differences.pop().unwrap().pop().unwrap();
        let prev = differences.last_mut().unwrap();
        prev.push(val + prev.last().unwrap());
    }

    *differences.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod predict_next_value_tests {
    use super::*;

    #[test]
    fn first_value_from_example_input() {
        let value_history = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(18, predict_next_value(value_history));
    }

    #[test]
    fn second_value_from_example_input() {
        let value_history = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(28, predict_next_value(value_history));
    }

    #[test]
    fn third_value_from_example_input() {
        let value_history = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(68, predict_next_value(value_history));
    }
}

#[cfg(test)]
mod predict_previous_value_tests {
    use super::*;

    #[test]
    fn first_value_from_example_input() {
        let mut value_history = vec![0, 3, 6, 9, 12, 15];
        value_history.reverse();
        assert_eq!(-3, predict_next_value(value_history));
    }

    #[test]
    fn second_value_from_example_input() {
        let mut value_history = vec![1, 3, 6, 10, 15, 21];
        value_history.reverse();
        assert_eq!(0, predict_next_value(value_history));
    }

    #[test]
    fn third_value_from_example_input() {
        let mut value_history = vec![10, 13, 16, 21, 30, 45];
        value_history.reverse();
        assert_eq!(5, predict_next_value(value_history));
    }
}
