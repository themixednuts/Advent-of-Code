use day_05::{solution1, solution2};
fn main() {
    let input = include_str!("../../../inputs/day5.txt");
    let part1 = solution1(input).unwrap();

    println!("{part1}");
    let sanitized = input.replace("\r\n", "\n");
    if let Some(part2) = solution2(&sanitized) {
        println!("{part2}");
    };
}
