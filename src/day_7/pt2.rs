use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn card_strength(card: char) -> u64 {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => panic!("invalid label: {}", card),
    }
}

#[derive(Eq)]
struct Hand {
    cards: [char; 5],
    bid: u64,
    card_counts: HashMap<char, i32>,
    joker_count: i32,
}

impl Hand {
    fn from(s: &str) -> Hand {
        let (cards, bid) = s.split_once(" ").unwrap();
        let cards: Vec<_> = cards.chars().collect();
        let mut card_counts = HashMap::new();
        let mut joker_count = 0;
        for card in &cards {
            if *card == 'J' {
                joker_count += 1;
            } else {
                let count = card_counts.entry(*card).or_insert(0);
                *count += 1
            }
        }
        let bid: u64 = bid.parse().unwrap();
        Hand {
            bid,
            cards: cards.try_into().unwrap(),
            card_counts,
            joker_count,
        }
    }

    fn strength(&self) -> usize {
        let mut count_values = self.card_counts.values().collect::<Vec<_>>();
        count_values.sort_unstable();
        count_values.reverse();

        let highest_count = if count_values.len() > 0 {
            count_values[0] + self.joker_count
        } else {
            self.joker_count
        };

        let second_highest_count = if count_values.len() > 1 {
            *count_values[1]
        } else {
            0
        };

        if highest_count == 5 {
            return 6;
        }

        if highest_count == 4 {
            return 5;
        }

        if highest_count == 3 {
            if second_highest_count == 2 {
                return 4;
            }
            return 3;
        }

        if highest_count == 2 {
            if second_highest_count == 2 {
                return 2;
            }
            return 1;
        }

        0
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.strength();
        let s2 = other.strength();
        let mut result = s1.cmp(&s2);
        let mut card_index = 0;
        while result == Ordering::Equal {
            let card1 = card_strength(self.cards[card_index]);
            let card2 = card_strength(other.cards[card_index]);
            result = card1.cmp(&card2);
            card_index += 1;
        }
        result
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).unwrap();
    let mut hands: Vec<_> = contents.lines().map(Hand::from).collect();
    hands.sort_unstable();
    let winnings = hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        let rank = i + 1;

        acc + (hand.bid * rank as u64)
    });

    winnings
}

#[cfg(test)]
mod hand_strength_tests {
    use super::*;

    #[test]
    fn test_high_card_no_jacks() {
        let s = "35219 30";

        let hand = Hand::from(s);

        assert_eq!(0, hand.strength());
    }

    #[test]
    fn test_one_pair_no_jacks() {
        let s = "35229 30";

        let hand = Hand::from(s);

        assert_eq!(1, hand.strength());
    }

    #[test]
    fn test_one_pair_incl_one_jack() {
        let s = "35J29 30";

        let hand = Hand::from(s);

        assert_eq!(1, hand.strength());
    }

    #[test]
    fn test_two_pair_no_jacks() {
        let s = "35325 30";

        let hand = Hand::from(s);

        assert_eq!(2, hand.strength());
    }

    #[test]
    fn test_three_of_a_kind_no_jacks() {
        let s = "35323 30";

        let hand = Hand::from(s);

        assert_eq!(3, hand.strength());
    }

    #[test]
    fn test_three_of_a_kind_incl_one_jack() {
        let s = "3532J 30";

        let hand = Hand::from(s);

        assert_eq!(3, hand.strength());
    }

    #[test]
    fn test_three_of_a_kind_incl_two_jacks() {
        let s = "12J3J 30";

        let hand = Hand::from(s);

        assert_eq!(3, hand.strength());
    }

    #[test]
    fn test_full_house_no_jacks() {
        let s = "11133 30";

        let hand = Hand::from(s);

        assert_eq!(4, hand.strength());
    }

    #[test]
    fn test_full_house_incl_one_jack() {
        let s = "J1122 30";

        let hand = Hand::from(s);

        assert_eq!(4, hand.strength());
    }

    #[test]
    fn test_four_of_a_kind_no_jacks() {
        let s = "11115 30";

        let hand = Hand::from(s);

        assert_eq!(5, hand.strength());
    }

    #[test]
    fn test_four_of_a_kind_incl_one_jack() {
        let s = "111J5 30";

        let hand = Hand::from(s);

        assert_eq!(5, hand.strength());
    }

    #[test]
    fn test_four_of_a_kind_incl_two_jacks() {
        let s = "11JJ5 30";

        let hand = Hand::from(s);

        assert_eq!(5, hand.strength());
    }

    #[test]
    fn test_four_of_a_kind_incl_three_jacks() {
        let s = "1JJJ5 30";

        let hand = Hand::from(s);

        assert_eq!(5, hand.strength());
    }

    #[test]
    fn test_five_of_a_kind_no_jacks() {
        let s = "KKKKK 30";

        let hand = Hand::from(s);

        assert_eq!(6, hand.strength());
    }

    #[test]
    fn test_five_of_a_kind_incl_one_jack() {
        let s = "KKKKJ 30";

        let hand = Hand::from(s);

        assert_eq!(6, hand.strength());
    }

    #[test]
    fn test_five_of_a_kind_incl_two_jacks() {
        let s = "QQQJJ 30";

        let hand = Hand::from(s);

        assert_eq!(6, hand.strength());
    }

    #[test]
    fn test_five_of_a_kind_incl_three_jacks() {
        let s = "TTJJJ 30";

        let hand = Hand::from(s);

        assert_eq!(6, hand.strength());
    }

    #[test]
    fn test_five_of_a_kind_incl_four_jacks() {
        let s = "TJJJJ 30";

        let hand = Hand::from(s);

        assert_eq!(6, hand.strength());
    }

    #[test]
    fn test_five_of_a_kind_all_jacks() {
        let s = "JJJJJ 30";

        let hand = Hand::from(s);

        assert_eq!(6, hand.strength());
    }
}
