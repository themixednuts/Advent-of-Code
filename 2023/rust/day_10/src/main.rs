use day_10::solution1;
fn main() {
    let input = include_str!("../../../inputs/day10.txt").replace("\r\n", "\n");

    let part1 = solution1(&input);
    println!("{part1}");
}
