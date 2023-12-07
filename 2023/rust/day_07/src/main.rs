use day_07::solution1;

fn main() {
    let input = include_str!("../../../inputs/day7.txt").replace("\r\n", "\n");
    let part1 = solution1(&input);

    println!("{part1}");
}
