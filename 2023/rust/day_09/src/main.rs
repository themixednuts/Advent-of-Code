use day_09::{solution1, solution2};
fn main() {
    const INPUT: &str = include_str!("../../../inputs/day9.txt");
    let part1 = solution1(INPUT);
    let part2 = solution2(INPUT);
    println!("{part1}");
    println!("{part2}");
}
