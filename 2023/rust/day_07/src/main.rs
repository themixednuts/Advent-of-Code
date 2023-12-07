use day_07::{solution1, solution2};

fn main() {
    let input = include_str!("../../../inputs/day7.txt").replace("\r\n", "\n");
    let part1 = solution1(&input);
    let part2 = solution2(&input);

    println!("{part1}");
    println!("{part2}");
}
