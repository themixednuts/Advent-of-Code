use day_03::{solution1, solution2};

// pub mod libs {
//     pub mod part1;
//     pub mod part2;
// }

fn main() {
    let str = include_str!("../../../inputs/day3.txt");

    let part_1 = solution1(str);
    println!("{part_1}");
    let part_2 = solution2(str);
    println!("{part_2}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1() {
        let str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let sum = solution1(str);
        println!("Sum?: {sum}");
        assert_eq!(4361, sum);
    }
}
