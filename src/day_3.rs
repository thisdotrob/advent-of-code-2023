use std::collections::HashSet;
use std::fs;

pub fn run() {
    let contents = fs::read_to_string("3.txt").unwrap();
    let lines: Vec<_> = contents.lines().collect();
    let pt1 = sum_of_part_numbers(&lines);
    println!("pt1: {}", pt1);
    let pt2 = sum_of_gear_ratios(&lines);
    println!("pt2: {}", pt2);
}

fn sum_of_part_numbers(lines: &Vec<&str>) -> u32 {
    let mut sum = 0;
    let mut checked_indexes = HashSet::new();

    let max_i = lines.len() - 1;

    let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    for i in 0..=max_i {
        let max_j = lines[i].len() - 1;

        for j in 0..=max_j {
            let char = lines[i][j];

            if !char.is_digit(10) && char != '.' {
                checked_indexes.insert((i, j));

                let i = i as i32;
                let j = j as i32;

                let mut adjacent_indexes = vec![
                    (i, j - 1),     // left
                    (i, j + 1),     // right
                    (i - 1, j),     // above
                    (i + 1, j),     // below
                    (i - 1, j - 1), // top left
                    (i - 1, j + 1), // top right
                    (i + 1, j - 1), // bottom left
                    (i + 1, j + 1), // bottom right
                ];

                adjacent_indexes.retain(|(i, j)| {
                    0 <= *i && max_i as i32 >= *i && 0 <= *j && max_j as i32 >= *j
                });

                let adjacent_indexes: Vec<(usize, usize)> = adjacent_indexes
                    .iter()
                    .map(|(i, j)| (*i as usize, *j as usize))
                    .collect();

                for (i, j) in adjacent_indexes {
                    if checked_indexes.contains(&(i, j)) {
                        continue;
                    }
                    let mut char = lines[i][j];
                    if char.is_digit(10) {
                        checked_indexes.insert((i, j));
                        let mut part_number = vec![];
                        let mut offset = 0;
                        while char.is_digit(10) {
                            checked_indexes.insert((i, (j - offset)));
                            part_number.push(char);
                            if j - offset > 0 {
                                offset += 1;
                                char = lines[i][j - offset];
                            } else {
                                break;
                            }
                        }
                        part_number.reverse();
                        offset = 1;
                        char = lines[i][j + offset];
                        checked_indexes.insert((i, j + offset));
                        while char.is_digit(10) {
                            part_number.push(char);
                            checked_indexes.insert((i, (j + offset)));
                            if j + offset < max_j {
                                offset += 1;
                                char = lines[i][j + offset];
                            } else {
                                break;
                            }
                        }

                        let part_number: String = part_number.into_iter().collect();
                        sum += part_number.parse::<u32>().unwrap();
                    }
                }
            }
        }
    }

    sum
}

fn sum_of_gear_ratios(lines: &Vec<&str>) -> u32 {
    let mut sum = 0;

    let max_i = lines.len() - 1;

    let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    for i in 0..=max_i {
        let max_j = lines[i].len() - 1;

        for j in 0..=max_j {
            let char = lines[i][j];

            if char == '*' {
                let i = i as i32;
                let j = j as i32;

                let mut adjacent_indexes = vec![
                    (i, j - 1),     // left
                    (i, j + 1),     // right
                    (i - 1, j),     // above
                    (i + 1, j),     // below
                    (i - 1, j - 1), // top left
                    (i - 1, j + 1), // top right
                    (i + 1, j - 1), // bottom left
                    (i + 1, j + 1), // bottom right
                ];

                adjacent_indexes.retain(|(i, j)| {
                    0 <= *i && max_i as i32 >= *i && 0 <= *j && max_j as i32 >= *j
                });

                let adjacent_indexes: Vec<(usize, usize)> = adjacent_indexes
                    .iter()
                    .map(|(i, j)| (*i as usize, *j as usize))
                    .collect();

                let mut checked_indexes = HashSet::new();
                let mut part_numbers = vec![];

                for (i, j) in adjacent_indexes {
                    if checked_indexes.contains(&(i, j)) {
                        continue;
                    }
                    let mut char = lines[i][j];
                    if char.is_digit(10) {
                        checked_indexes.insert((i, j));
                        let mut part_number = vec![];
                        let mut offset = 0;
                        while char.is_digit(10) {
                            checked_indexes.insert((i, (j - offset)));
                            part_number.push(char);
                            if j - offset > 0 {
                                offset += 1;
                                char = lines[i][j - offset];
                            } else {
                                break;
                            }
                        }
                        part_number.reverse();
                        offset = 1;
                        char = lines[i][j + offset];
                        checked_indexes.insert((i, j + offset));
                        while char.is_digit(10) {
                            part_number.push(char);
                            checked_indexes.insert((i, (j + offset)));
                            if j + offset < max_j {
                                offset += 1;
                                char = lines[i][j + offset];
                            } else {
                                break;
                            }
                        }

                        let part_number: String = part_number.into_iter().collect();
                        part_numbers.push(part_number.parse::<u32>().unwrap());
                    }
                }

                if part_numbers.len() == 2 {
                    sum += part_numbers[0] * part_numbers[1];
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod day_3_pt2_test {
    use super::*;

    #[test]
    fn first_2_lines_of_input() {
        let lines = vec![
".......262....300...................507.....961..............668.....................189.906...........................624..................",
"..148.................805..130..880*...........*684.............*......*..............*..-......%.................$........17...65....91*...",
        ];
        let expected = (507 * 880) + (961 * 684);
        assert_eq!(expected, sum_of_gear_ratios(&lines));
    }

    #[test]
    fn last_5_lines_of_input() {
        let lines = vec![
"..........27.#498..*.....286..+490...............................966.....................262.......................#..542.237...............",
"72....251..#.......667.....................282..556..260...........%.$......................*....107..................*.....................",
".......*.......*..............................*..-...*...............64..#...=.....402..@....790.@.....................295.766...484..969...",
"......85....882.80.......184$................117........................454..583......*..351...............266....................&....*....",
".......................................................48...........................436........275...................869............258.....",
        ];
        let expected = (262 * 790)
            + (542 * 295)
            + (251 * 85)
            + (882 * 80)
            + (282 * 117)
            + (402 * 436)
            + (969 * 258);
        assert_eq!(expected, sum_of_gear_ratios(&lines));
    }
}

#[cfg(test)]
mod day_3_pt1_test {
    use super::*;

    #[test]
    fn no_numbers_or_symbols() {
        let lines = vec![".....", ".....", "....."];
        assert_eq!(0, sum_of_part_numbers(&lines));
    }

    #[test]
    fn symbols_and_no_numbers() {
        let lines = vec!["+.*.$", ".=..%", "/.@.#", "...-."];
        assert_eq!(0, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_left() {
        let lines = vec![".....", "$101.", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_right() {
        let lines = vec![".....", ".101%", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_top() {
        let lines = vec!["..@..", ".101.", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_bottom() {
        let lines = vec![".....", ".101.", "..*.."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_top_left() {
        let lines = vec!["-....", ".101.", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_top_right() {
        let lines = vec!["....+", ".101.", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_bottom_left() {
        let lines = vec![".....", ".101.", "/...."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_to_bottom_right() {
        let lines = vec![".....", ".101.", "....#"];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_top_inner() {
        let lines = vec![".=...", ".101.", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));

        let lines = vec!["...=.", ".101.", "....."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn single_part_number_symbol_bottom_inner() {
        let lines = vec![".....", ".101.", ".&..."];
        assert_eq!(101, sum_of_part_numbers(&lines));

        let lines = vec![".....", ".101.", "...&."];
        assert_eq!(101, sum_of_part_numbers(&lines));
    }

    #[test]
    fn first_2_lines_of_input() {
        let lines = vec![
".......262....300...................507.....961..............668.....................189.906...........................624..................",
"..148.................805..130..880*...........*684.............*......*..............*..-......%.................$........17...65....91*...",
        ];
        let expected = 507 + 961 + 668 + 189 + 906 + 880 + 684 + 91;
        assert_eq!(expected, sum_of_part_numbers(&lines));
    }
}
