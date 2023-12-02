use std::fs;

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let contents = fs::read_to_string("1.txt").unwrap();

    let mut pt1_sum = 0;
    let mut pt2_sum = 0;

    for line in contents.lines() {
        pt1_sum += calibration_value_pt1(line);
        pt2_sum += calibration_value_pt2(line);
    }

    println!("pt1: {}", pt1_sum);
    println!("pt2: {}", pt2_sum);
}

fn calibration_value_pt1(line: &str) -> u32 {
    let first_int = get_first_int(line);

    let line_reversed: String = line.chars().rev().collect();

    let last_int = get_first_int(&line_reversed);

    last_int + first_int * 10
}

fn calibration_value_pt2(line: &str) -> u32 {
    let num_strs: Vec<String> = NUM_STRS.into_iter().map(|s| String::from(s)).collect();

    let first_int = get_first_int_pt2(line, &num_strs);

    let line_reversed: String = line.chars().rev().collect();

    let num_strs_rev: Vec<String> = num_strs.iter().map(|s| s.chars().rev().collect()).collect();

    let last_int = get_first_int_pt2(&line_reversed, &num_strs_rev);

    last_int + first_int * 10
}

const RADIX: u32 = 10;

fn get_first_int(s: &str) -> u32 {
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(RADIX).unwrap();
        }
    }

    panic!("No int found in {}", s);
}

fn get_first_int_pt2(s: &str, num_strs: &Vec<String>) -> u32 {
    let mut checked_chars = String::from("");

    for c in s.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(RADIX).unwrap();
        }

        checked_chars.push(c);

        for (i, num_str) in num_strs.iter().enumerate() {
            if checked_chars.contains(num_str) {
                return (1 + i).try_into().unwrap();
            }
        }
    }

    panic!("No int found in {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = "1abc2";
        let result = calibration_value_pt1(s);
        assert_eq!(result, 12);
    }

    #[test]
    fn pt1_2() {
        let s = "pqr3stu8vwx";
        let result = calibration_value_pt1(s);
        assert_eq!(result, 38);
    }

    #[test]
    fn pt1_3() {
        let s = "a1b2c3d4e5f";
        let result = calibration_value_pt1(s);
        assert_eq!(result, 15);
    }

    #[test]
    fn pt1_4() {
        let s = "treb7uchet";
        let result = calibration_value_pt1(s);
        assert_eq!(result, 77);
    }

    #[test]
    fn pt2_1() {
        let s = "two1nine";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 29);
    }

    #[test]
    fn pt2_2() {
        let s = "eightwothree";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 83);
    }

    #[test]
    fn pt2_3() {
        let s = "abcone2threexyz";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 13);
    }

    #[test]
    fn pt2_4() {
        let s = "xtwone3four";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 24);
    }

    #[test]
    fn pt2_5() {
        let s = "4nineeightseven2";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 42);
    }

    #[test]
    fn pt2_6() {
        let s = "zoneight234";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 14);
    }

    #[test]
    fn pt2_7() {
        let s = "7pqrstsixteen";
        let result = calibration_value_pt2(s);
        assert_eq!(result, 76);
    }
}
