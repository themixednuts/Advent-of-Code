// #![feature(test)]
// extern crate test;

pub mod hand;
pub mod hand2;

use hand::Hand;
use hand2::Hand2;

pub fn solution1(input: &str) -> usize {
    let mut s = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            (Hand::new(hand), bid)
        })
        .collect::<Vec<_>>();

    s.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    s.iter().enumerate().fold(0, |acc, (idx, (_hand, bid))| {
        let rank = idx + 1;
        let winnings = rank * bid.parse::<usize>().unwrap();

        acc + winnings
    })
}
pub fn solution2(input: &str) -> usize {
    let mut s = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            (Hand2::new(hand), bid)
        })
        .collect::<Vec<_>>();

    s.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    s.iter().enumerate().fold(0, |acc, (idx, (_hand, bid))| {
        let rank = idx + 1;
        let winnings = rank * bid.parse::<usize>().unwrap();

        acc + winnings
    })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[cfg(test)]
mod tests {

    use super::*;
    // use test::Bencher;

    #[test]
    fn test1() {
        let ex = include_str!("../../../inputs/ex7.txt").replace("\r\n", "\n");

        let winnings = solution1(&ex);
        dbg!(&winnings);
        assert_eq!(6440, winnings);
    }

    #[test]
    fn test2() {
        let ex = include_str!("../../../inputs/ex7.txt").replace("\r\n", "\n");

        let winnings = solution2(&ex);
        dbg!(&winnings);
        assert_eq!(5905, winnings);
    }

    // #[bench]
    // fn bench_solution1(b: &mut Bencher) {
    //     let input = include_str!("../../../inputs/day7.txt").replace("\r\n", "\n");
    //     b.iter(|| solution1(&input));
    // }
    // #[bench]
    // fn bench_solution2(b: &mut Bencher) {
    //     let input = include_str!("../../../inputs/day7.txt").replace("\r\n", "\n");
    //     b.iter(|| solution2(&input));
    // }
}
