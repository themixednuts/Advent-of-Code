pub fn solution1(input: &str) -> usize {
    input
        .split_once("\n")
        .map(|(times, distances)| {
            let times = times
                .split_ascii_whitespace()
                .filter_map(|str| str.parse::<usize>().ok())
                .into_iter();
            let distances = distances
                .split_ascii_whitespace()
                .filter_map(|str| str.parse::<usize>().ok())
                .into_iter();

            times
                .zip(distances)
                .map(|(time, distance)| ((time.pow(2) - 4 * distance) as f64).sqrt() as usize)
                .product()
        })
        .unwrap()
}
pub fn solution2(input: &str) -> usize {
    input
        .split_once("\n")
        .map(|(times, distances)| {
            let time = times
                .split_ascii_whitespace()
                .skip(1)
                .fold("".to_string(), |acc, t| acc + t)
                .parse::<usize>()
                .unwrap();

            let distance = distances
                .split_ascii_whitespace()
                .skip(1)
                .fold("".to_string(), |acc, d| acc + d)
                .parse::<usize>()
                .unwrap();

            ((time.pow(2) - 4 * distance) as f64).sqrt() as usize
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let ex = "Time:      7  15   30
Distance:  9  40  200";

        let res = solution1(ex);
        assert_eq!(288, res);
    }

    #[test]
    fn test2() {
        let ex = "Time:      7  15   30
Distance:  9  40  200";

        let res = solution2(ex);
        assert_eq!(71503, res);
    }
}
