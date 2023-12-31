use std::{cmp::Ordering, collections::HashMap};

use crate::HandType;

#[derive(Debug)]
pub(crate) struct Hand<'a>(&'a str);

impl<'a> Hand<'a> {
    pub fn new(cards: &'a str) -> Hand {
        Hand(cards)
    }

    fn hand_type(&self) -> HandType {
        let mut char_count = HashMap::new();

        self.0.chars().for_each(|c| {
            char_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });

        let mut sorted = char_count.into_values().collect::<Vec<_>>();

        sorted.sort_unstable();
        sorted.reverse();
        match (sorted.get(0), sorted.get(1)) {
            (Some(&5), _) => HandType::FiveKind,
            (Some(&4), _) => HandType::FourKind,
            (Some(&3), Some(&2)) => HandType::FullHouse,
            (Some(&3), _) => HandType::ThreeKind,
            (Some(&2), Some(&2)) => HandType::TwoPair,
            (Some(&2), _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
    fn get_card_value(&self) -> [usize; 5] {
        let mut values = [0usize; 5];
        for (idx, card) in self.0.chars().enumerate() {
            match card {
                '2' => values[idx] = 0,
                '3' => values[idx] = 1,
                '4' => values[idx] = 2,
                '5' => values[idx] = 3,
                '6' => values[idx] = 4,
                '7' => values[idx] = 5,
                '8' => values[idx] = 6,
                '9' => values[idx] = 7,
                'T' => values[idx] = 8,
                'J' => values[idx] = 9,
                'Q' => values[idx] = 10,
                'K' => values[idx] = 11,
                'A' => values[idx] = 12,
                _ => unreachable!(),
            }
        }
        values
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type()
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(Ordering::Equal) => {
                for i in 0..self.0.len() {
                    let self_card = self.get_card_value()[i];
                    let other_card = other.get_card_value()[i];

                    match self_card.cmp(&other_card) {
                        Ordering::Equal => continue,
                        result => return Some(result),
                    }
                }
                Some(Ordering::Equal)
            }
            result => result,
        }
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for i in 0..self.0.len() {
                    let self_card = self.get_card_value()[i];
                    let other_card = other.get_card_value()[i];

                    match self_card.cmp(&other_card) {
                        Ordering::Equal => continue,
                        result => return result,
                    }
                }
                Ordering::Equal
            }
            result => result,
        }
    }
}
