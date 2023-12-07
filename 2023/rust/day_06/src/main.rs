use day_06::{solution1, solution2};

fn main() {
    let input = include_str!("../../../inputs/day6.txt");

    let part1 = solution1(input);
    let part2 = solution2(input);
    dbg!(part1);
    dbg!(part2);
}
