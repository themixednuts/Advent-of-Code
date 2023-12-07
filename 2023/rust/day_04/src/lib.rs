pub fn solution1(str: &str) -> usize {
    str.trim().lines().fold(0, |acc, l| {
        let (winning, draw) = l.split_once(":").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers: Vec<_> = winning.split_whitespace().collect();
        let count = draw
            .split_whitespace()
            .filter(|draw_num| winning_numbers.contains(&draw_num))
            .count();

        if count > 0 {
            acc + 2usize.pow(count as u32 - 1u32)
        } else {
            acc
        }
    })
}

pub fn solution2(str: &str) -> usize {
    let n = str.trim().lines().count();
    str.trim()
        .lines()
        .enumerate()
        .fold(vec![1; n], |mut acc, (idx, l)| {
            let (winning, draw) = l.split_once(":").unwrap().1.split_once(" | ").unwrap();
            let winning_numbers: Vec<_> = winning.split_whitespace().collect();
            let count = draw
                .split_whitespace()
                .filter(|draw_num| winning_numbers.contains(&draw_num))
                .count();

            if count > 0 {
                (1..=count).for_each(|v| {
                    acc[idx + v] += 1 * acc[idx];
                });
                acc
            } else {
                acc
            }
        })
        .into_iter()
        .sum()
}

pub fn solution2_append(str: &str) -> usize {
    str.trim()
        .lines()
        .enumerate()
        .fold(vec![1], |mut acc, (idx, l)| {
            let (winning, draw) = l.split_once(":").unwrap().1.split_once(" | ").unwrap();
            let winning_numbers: Vec<_> = winning.split_whitespace().collect();
            let count = draw
                .split_whitespace()
                .filter(|draw_num| winning_numbers.contains(&draw_num))
                .count();

            if count > 0 {
                (1..=count).for_each(|v| {
                    if (v + idx) >= acc.len() {
                        acc.resize(v + idx + 1, 1);
                    }
                    acc[idx + v] += 1 * acc[idx];
                });
                acc
            } else {
                acc
            }
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        let str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        assert_eq!(13, solution1(str));
    }

    #[test]
    fn test2() {
        let str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        assert_eq!(30, solution2(str));
    }
}
