use std::fs;

fn main() {
    let contents = fs::read_to_string("1.txt").unwrap();

    let mut pt1_sum = 0;
    let mut pt2_sum = 0;

    for line in contents.lines() {
        pt1_sum += calibration_value(line);
        pt2_sum += calibration_value_pt2(line);
    }

    println!("pt1: {}", pt1_sum);
    println!("pt2: {}", pt2_sum);
}

const RADIX: u32 = 10;

fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}

fn first_int(s: &str) -> u32 {
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(RADIX).unwrap();
        }
    }

    panic!("No num found in {}", s);
}

fn calibration_value(s: &str) -> u32 {
    let first_num = first_int(s);

    let s_reversed = reverse_str(s);

    let last_num = first_int(&s_reversed);

    last_num + first_num * 10
}

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_int_pt2(s: &str, num_strs: impl Iterator<Item = String>) -> u32 {
    let mut comparison = String::from("");

    for c in s.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(RADIX).unwrap();
        }

        comparison.push(c);

        for (i, x) in num_strs.enumerate() {
            if comparison.contains(&x) {
                return (1 + i).try_into().unwrap();
            }
        }
    }

    panic!("No num found in {}", s);
}

fn calibration_value_pt2(s: &str) -> u32 {
    let num_strs: Vec<String> = NUM_STRS.into_iter().map(|s| {
        String::from(s)
    });

    let first_num = first_int_pt2(s, num_strs);

    let s_reversed = reverse_str(s);

    let num_strs_rev: Vec<String> = num_strs.iter().map(|s| {
        reverse_str(s)
    }).collect();

    let last_num = first_int_pt2(&s_reversed, &num_strs_rev);

    last_num + first_num * 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = "1abc2";
        let result = calibration_value(s);
        assert_eq!(result, 12);
    }

    #[test]
    fn pt1_2() {
        let s = "pqr3stu8vwx";
        let result = calibration_value(s);
        assert_eq!(result, 38);
    }

    #[test]
    fn pt1_3() {
        let s = "a1b2c3d4e5f";
        let result = calibration_value(s);
        assert_eq!(result, 15);
    }

    #[test]
    fn pt1_4() {
        let s = "treb7uchet";
        let result = calibration_value(s);
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
