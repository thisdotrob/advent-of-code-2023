use std::fs;

pub fn run() {
    let contents = fs::read_to_string("3.txt").unwrap();
}

fn sum_of_part_numbers() {}

fn is_part_number(i: usize, s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    let length = chars.len();

    assert_eq!(s.len(), length);

    let is_first_in_row = i % 10 == 0;
    let is_last_in_row = if i > 8 { (i - 9) % 10 == 0 } else { false };

    let char = chars[i];

    if !char.is_numeric() {
        return false;
    }

    let i = i as i32;

    let mut adjacent_indexes: Vec<i32> = vec![
        i - 1,  // left
        i + 1,  // right
        i - 10, // above
        i + 10, // below
        i - 11, // diagonal above left
        i - 9,  // diagonal above right
        i + 9,  // diagonal below left
        i + 11, // diagonal below right
    ];

    if !is_first_in_row {
        adjacent_indexes.push(i - 11); // diagonal above left
        adjacent_indexes.push(i + 9); // diagonal below left
    }

    if !is_last_in_row {
        adjacent_indexes.push(i - 9); // diagonal above right
        adjacent_indexes.push(i + 11); // diagonal below right
    }

    // remove non-existant adjacent indexes
    adjacent_indexes.retain(|i| *i >= 0);
    adjacent_indexes.retain(|i| *i < length as i32);

    for i in adjacent_indexes {
        let char = chars[i as usize];
        if char != '.' && char.is_ascii_punctuation() {
            println!("{}", i);
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMATIC: &str = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";

    #[test]
    fn is_part_number_01() {
        // first element in string
        // is number but no adjacent symbols
        assert!(!is_part_number(0, SCHEMATIC));
    }

    #[test]
    fn is_part_number_2() {
        // is number but no adjacent symbols
        assert!(!is_part_number(1, SCHEMATIC));
    }

    #[test]
    fn is_part_number_3() {
        // is number, has adjacent symbol below right diagonal (at index 13)
        assert!(is_part_number(2, SCHEMATIC));
    }

    #[test]
    fn is_part_number_4() {
        // is a full stop
        assert!(!is_part_number(3, SCHEMATIC));
    }

    #[test]
    fn is_part_number_5() {
        // is number but no adjacent symbols
        assert!(!is_part_number(5, SCHEMATIC));
    }

    #[test]
    fn is_part_number_6() {
        // is number but no adjacent symbols
        assert!(!is_part_number(6, SCHEMATIC));
    }

    #[test]
    fn is_part_number_7() {
        // is number but no adjacent symbols
        assert!(!is_part_number(7, SCHEMATIC));
    }

    #[test]
    fn is_part_number_8() {
        // is a full stop
        assert!(!is_part_number(9, SCHEMATIC));
    }

    #[test]
    fn is_part_number_9() {
        // is a symbol
        assert!(!is_part_number(13, SCHEMATIC));
    }

    #[test]
    fn is_part_number_10() {
        // is number, has adjacent symbol above right diagonal (at index 13)
        assert!(is_part_number(22, SCHEMATIC));
    }

    #[test]
    fn is_part_number_11() {
        // is number, has adjacent symbol above (at index 13)
        assert!(is_part_number(23, SCHEMATIC));
    }

    #[test]
    fn is_part_number_12() {
        // is number, has adjacent symbol below (at index 36)
        assert!(is_part_number(26, SCHEMATIC));
    }

    #[test]
    fn is_part_number_13() {
        // is number, has adjacent symbol below left diagonal (at index 36)
        assert!(is_part_number(27, SCHEMATIC));
    }

    #[test]
    fn is_part_number_14() {
        // is number but no adjacent symbols
        assert!(!is_part_number(28, SCHEMATIC));
    }

    #[test]
    fn is_part_number_15() {
        // is number but no adjacent symbols
        assert!(!is_part_number(41, SCHEMATIC));
    }

    #[test]
    fn is_part_number_16() {
        // is number, has adjacent symbol to right (at index 43)
        assert!(is_part_number(42, SCHEMATIC));
    }

    #[test]
    fn is_part_number_17() {
        // is number, has adjacent symbol above left (at index 85)
        assert!(is_part_number(96, SCHEMATIC));
    }

    #[test]
    fn is_part_number_18() {
        // is number, has adjacent symbol to left
        let schematic = "..........$6";
        assert!(is_part_number(11, schematic));
    }
}
