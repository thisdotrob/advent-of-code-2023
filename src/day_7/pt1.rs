use std::cmp::Ordering;
use std::fmt;
use std::fs;

trait Strength {
    fn strength(&self) -> usize;
}

#[derive(Eq)]
struct Card {
    label: char,
}

impl Card {
    fn from(c: char) -> Card {
        Card { label: c }
    }
}

impl Strength for Card {
    fn strength(&self) -> usize {
        match self.label {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => panic!("invalid label: {}", self.label),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn from(s: &str) -> Hand {
        let (cards, bid) = s.split_once(" ").unwrap();
        let bid: u64 = bid.parse().unwrap();
        let mut cards = cards.chars();
        let cards = [
            Card::from(cards.next().unwrap()),
            Card::from(cards.next().unwrap()),
            Card::from(cards.next().unwrap()),
            Card::from(cards.next().unwrap()),
            Card::from(cards.next().unwrap()),
        ];
        Hand { bid, cards }
    }
}

impl Strength for Hand {
    fn strength(&self) -> usize {
        let mut card_counts = [0; 13];

        for card in &self.cards {
            card_counts[card.strength()] += 1;
        }

        if card_counts.contains(&5) {
            return 6;
        }

        if card_counts.contains(&4) {
            return 5;
        }

        if card_counts.contains(&3) && card_counts.contains(&2) {
            return 4;
        }

        if card_counts.contains(&3) {
            return 3;
        }

        let pairs_count = card_counts
            .iter()
            .fold(0, |acc, card_count| match card_count {
                2 => acc + 1,
                _ => acc,
            });

        if pairs_count == 2 {
            return 2;
        }

        if pairs_count == 1 {
            return 1;
        }

        return 0;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.strength();
        let s2 = other.strength();
        let mut result = s1.cmp(&s2);
        let mut cards1 = self.cards.iter();
        let mut cards2 = other.cards.iter();
        while result == Ordering::Equal {
            let card1 = cards1.next().unwrap();
            let card2 = cards2.next().unwrap();
            result = card1.cmp(&card2);
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

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ cards: {:?}, bid: {:?} }}", self.cards, self.bid)
    }
}

pub fn run(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    let mut hands: Vec<_> = lines.map(|l| Hand::from(l)).collect();

    hands.sort_unstable();

    let winnings = hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        let rank = i + 1;

        acc + (hand.bid * rank as u64)
    });

    winnings
}
