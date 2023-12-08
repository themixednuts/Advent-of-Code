use std::collections::HashMap;

pub fn solution1(input: &str) -> usize {
    let mut rotation: Vec<char> = vec![];
    let mut next_idx = 0;
    let mut search_key = "AAA".to_string();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut count = 0;

    input.lines().enumerate().for_each(|(idx, l)| {
        if idx == 0 {
            rotation = l.chars().collect::<Vec<_>>();
            return;
        }
        if idx == 1 {
            return;
        }

        let (set, dir) = l.split_once(" = ").unwrap();
        let (left, right) = dir.split_once(",").unwrap();

        let left = left.replace("(", "").replace(" ", "");
        let right = right.replace(")", "").replace(" ", "");
        map.insert(set.to_owned(), (left.to_owned(), right.to_owned()));
    });

    // dbg!(&last);

    loop {
        if search_key == "ZZZ" {
            return count;
        }
        count += 1;
        match rotation[next_idx] {
            'R' => {
                search_key = map.get(&search_key).unwrap().1.to_owned();
            }
            'L' => {
                search_key = map.get(&search_key).unwrap().0.to_owned();
            }
            _ => unreachable!(),
        }

        if rotation.get(next_idx + 1).is_some() {
            next_idx += 1;
        } else {
            next_idx = 0;
        }
    }
}

pub fn solution2(input: &str) -> usize {
    let mut rotation: Vec<char> = vec![];
    let mut next_idx = 0;
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    input.lines().enumerate().for_each(|(idx, l)| {
        if idx == 0 {
            rotation = l.chars().collect::<Vec<_>>();
            return;
        }
        if idx == 1 {
            return;
        }

        let (set, dir) = l.split_once(" = ").unwrap();
        let (left, right) = dir.split_once(",").unwrap();

        let left = left.replace("(", "").replace(" ", "");
        let right = right.replace(")", "").replace(" ", "");
        map.insert(set.to_owned(), (left.to_owned(), right.to_owned()));
    });

    let mut search_keys: Vec<String> = map
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('A'))
        .collect();

    let mut cycles: Vec<usize> = map
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('A'))
        .map(|_| 0)
        .collect();

    loop {
        if search_keys.iter().all(|s| s.ends_with('Z')) {
            return cycles.into_iter().fold(1, num::integer::lcm);
        }
        for i in 0..search_keys.len() {
            if search_keys[i].ends_with('Z') {
                continue;
            }
            match rotation[next_idx] {
                'R' => {
                    search_keys[i] = map.get(&search_keys[i]).unwrap().1.to_owned();
                    cycles[i] += 1;
                }
                'L' => {
                    search_keys[i] = map.get(&search_keys[i]).unwrap().0.to_owned();
                    cycles[i] += 1;
                }
                _ => unreachable!(),
            }
        }

        if rotation.get(next_idx + 1).is_some() {
            next_idx += 1;
        } else {
            next_idx = 0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../../../inputs/ex8.txt").replace("\r\n", "\n");

        let res = solution1(&input);
        assert_eq!(6, res);
    }
    #[test]
    fn test2() {
        let ex = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let res = solution2(ex);
        assert_eq!(6, res);
    }
}
