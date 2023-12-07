use libs::{part1::parser1, part2::parser2};

mod libs {
    pub mod part1;
    pub mod part2;
}

// struct GameRecord {
//     pub id: usize,
//     pub red: usize,
//     pub blue: usize,
//     pub green: usize,
// }

fn main() {
    let input_str = include_str!("../../../inputs/day2.txt");

    let part1 = parser1(input_str);
    println!("{part1}");

    let part2 = parser2(input_str);
    println!("{part2}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1() {
        let str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let sum = parser1(str);
        assert_eq!(8, sum);
    }

    #[test]
    fn part2() {
        let str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let sum = parser2(str);
        assert_eq!(2286, sum);
    }
}
