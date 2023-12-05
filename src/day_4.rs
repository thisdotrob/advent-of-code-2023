use std::collections::HashSet;
use std::fs;

pub fn run() {
    let contents = fs::read_to_string("4.txt").unwrap();
    let lines: Vec<_> = contents.lines().collect();
    let pt1 = points_total(&lines);
    println!("pt1: {}", pt1);
    let mut memo: [i32; 200] = [-1; 200];
    let pt2 = scratchcards_total(&lines[..], lines.len(), &mut memo);
    println!("pt2: {}", pt2);
}

fn points_total(lines: &Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|s| Card::from(s))
        .map(|card| card.points)
        .sum()
}

fn scratchcards_total(lines: &[&str], count: usize, memo: &mut [i32; 200]) -> u32 {
    let mut total = 0_u32;
    for i in 0..count {
        let card = Card::from(lines[i]);
        let memo_total = memo[card.card_number as usize];
        if memo_total == -1 {
            if card.match_count > 0 {
                let copies = scratchcards_total(&lines[i + 1..], card.match_count as usize, memo);
                total += 1 + copies;
                memo[card.card_number as usize] = 1 + copies as i32;
            } else {
                total += 1;
                memo[card.card_number as usize] = 1;
            }
        } else {
            total += memo_total as u32;
        }
    }
    total
}

struct Card {
    card_number: u32,
    match_count: u32,
    points: u32,
}

impl Card {
    fn from(s: &str) -> Card {
        let (card_number, rest) = s.split_once(":").unwrap();
        let (_, card_number) = card_number.split_once(" ").unwrap();
        let card_number: u32 = card_number.trim().parse().unwrap();
        let (winning_numbers, numbers) = rest.split_once("|").unwrap();
        let (winning_numbers, numbers) = (winning_numbers.trim(), numbers.trim());
        let winning_numbers: HashSet<_> = winning_numbers
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let numbers: Vec<_> = numbers
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let mut match_count = 0;
        for n in numbers {
            if winning_numbers.contains(&n) {
                match_count += 1;
            }
        }
        let points = if match_count > 0 {
            2_u32.pow(match_count - 1)
        } else {
            0
        };

        Card {
            card_number,
            points,
            match_count,
        }
    }
}

#[cfg(test)]
mod day_4_pt1_test {
    use super::*;

    #[test]
    fn card_from_line_01() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from(line);
        assert_eq!(8, card.points);
        assert_eq!(4, card.match_count);
    }

    #[test]
    fn card_from_line_02() {
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = Card::from(line);
        assert_eq!(2, card.points);
        assert_eq!(2, card.match_count);
    }

    #[test]
    fn card_from_line_03() {
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let card = Card::from(line);
        assert_eq!(2, card.points);
        assert_eq!(2, card.match_count);
    }

    #[test]
    fn card_from_line_04() {
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let card = Card::from(line);
        assert_eq!(1, card.points);
        assert_eq!(1, card.match_count);
    }

    #[test]
    fn card_from_line_05() {
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let card = Card::from(line);
        assert_eq!(0, card.points);
        assert_eq!(0, card.match_count);
    }

    #[test]
    fn card_from_line_06() {
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let card = Card::from(line);
        assert_eq!(0, card.points);
        assert_eq!(0, card.match_count);
    }

    #[test]
    fn points_total_example_input() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = 13;

        assert_eq!(expected, points_total(&lines));
    }

    #[test]
    fn points_total_first_two_lines_of_input() {
        let lines = vec![
"Card   1: 17 15  5 75 36 13 16 66 92 39 | 13 92 16  5 87 78 15 94 21 48 30 62 70 41  3 39 22 17 77 58 75 52 83 34 24",
"Card   2: 72 64 18  5 58 94 25 59 75 51 | 34 17 48 75 25  8  2 94 64 29 33 92 73 12 51 38 27  4  1 60 31 85 59 18  5",
        ];

        let card_1_match_count = 8;
        let card_1_points = 2_u32.pow(card_1_match_count - 1);
        let card_2_match_count = 8;
        let card_2_points = 2_u32.pow(card_2_match_count - 1);

        let expected = card_1_points + card_2_points;
        assert_eq!(expected, points_total(&lines));
    }

    #[test]
    fn scratchcards_total_example_input() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = 30;

        let mut memo: [i32; 200] = [-1; 200];

        assert_eq!(expected, scratchcards_total(&lines, lines.len(), &mut memo));
    }
}
