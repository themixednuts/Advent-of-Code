use std::{cmp::Ordering, collections::HashMap};

use crate::HandType;

#[derive(Debug)]
pub struct Hand2<'a>(&'a str);

impl<'a> Hand2<'a> {
    pub fn new(cards: &'a str) -> Hand2 {
        Hand2(cards)
    }

    fn hand_type(&self) -> HandType {
        let mut char_count = HashMap::new();

        self.0.chars().for_each(|c| {
            char_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });

        let j_value = char_count.remove(&'J');
        let mut sorted = char_count.into_values().collect::<Vec<_>>();

        sorted.sort_unstable();
        sorted.reverse();
        match (sorted.get(0), sorted.get(1)) {
            (Some(&5), _) => HandType::FiveKind,
            (Some(num), _) if j_value.is_some_and(|j| num + j > 4) => HandType::FiveKind,
            (_, _) if j_value.is_some_and(|j| j == 5) => HandType::FiveKind,
            (Some(&4), _) => HandType::FourKind,
            (Some(num), _) if j_value.is_some_and(|j| num + j > 3) => HandType::FourKind,
            (Some(&3), Some(&2)) => HandType::FullHouse,
            (Some(num), Some(s)) if j_value.is_some_and(|j| num + j + s > 4) => HandType::FullHouse,
            (Some(&3), _) => HandType::ThreeKind,
            (Some(num), Some(s)) if j_value.is_some_and(|j| num + j + s > 3) => HandType::ThreeKind,
            (Some(&2), Some(&2)) => HandType::TwoPair,
            (_, _) if j_value.is_some_and(|j| j > 1) => HandType::TwoPair,
            (Some(&2), _) => HandType::OnePair,
            (Some(num), Some(s)) if j_value.is_some_and(|j| num + j + s > 2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
    fn get_card_value(&self) -> [usize; 5] {
        let mut values = [0usize; 5];
        for (idx, card) in self.0.chars().enumerate() {
            match card {
                'J' => values[idx] = 0,
                '2' => values[idx] = 1,
                '3' => values[idx] = 2,
                '4' => values[idx] = 3,
                '5' => values[idx] = 4,
                '6' => values[idx] = 5,
                '7' => values[idx] = 6,
                '8' => values[idx] = 7,
                '9' => values[idx] = 8,
                'T' => values[idx] = 9,
                'Q' => values[idx] = 10,
                'K' => values[idx] = 11,
                'A' => values[idx] = 12,
                _ => unreachable!(),
            }
        }
        values
    }
}

impl<'a> PartialEq for Hand2<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type()
    }
}

impl<'a> Eq for Hand2<'a> {}

impl<'a> PartialOrd for Hand2<'a> {
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

impl<'a> Ord for Hand2<'a> {
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
