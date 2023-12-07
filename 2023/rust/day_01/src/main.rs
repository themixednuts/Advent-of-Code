use crate::libs::{part_1::parser_part_1, part_2::parser_part_2};

mod libs {
    pub mod part_1;
    pub mod part_2;
}
fn main() {
    let input_str = include_str!("../../../inputs/day_01.txt");
    let part_1 = parser_part_1(input_str);
    let part_2 = parser_part_2(input_str);
    println!("Day 1 - Part 1: {part_1}");
    println!("Day 1 - Part 2: {part_2}");
}

#[test]
fn test_1() {
    const SAMPLE_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    let sum = parser_part_1(SAMPLE_1);
    assert_eq!(142, sum);
}
#[test]
fn test_2() {
    const SAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let sum = parser_part_2(SAMPLE_2);
    assert_eq!(281, sum);
}
#[test]
fn custom_test() {
    let sum = parser_part_2("1fivesix");
    assert_eq!(16, sum);
}
