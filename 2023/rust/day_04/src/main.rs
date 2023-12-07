use day_04::{solution1, solution2, solution2_append};

fn main() {
    let input = include_str!("../../../inputs/day4.txt");

    let part1 = solution1(input);
    let part2 = solution2(input);
    let part3 = solution2_append(input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("Part 3: {part3}");
}
