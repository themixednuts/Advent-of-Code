pub fn solution1(input: &str) -> Option<usize> {
    if let Some((seeds, maps)) = input.lines().collect::<Vec<_>>().split_first() {
        let ids = seeds.split_at(7).1.split_whitespace();
        let mut arr: Vec<usize> = ids
            .clone()
            .into_iter()
            .map(|ele| ele.parse::<usize>().unwrap())
            .collect();

        maps.split(|l| l.is_empty())
            .filter(|m| !m.is_empty())
            .for_each(|m| {
                // println!("");
                let mut solved = vec![false; ids.clone().count()];
                m.split_first().unwrap().1.iter().for_each(|l| {
                    let mapper: Vec<&str> = l.splitn(3, " ").collect();
                    let source: usize = mapper[1].parse().unwrap();
                    let destination: usize = mapper[0].parse().unwrap();
                    let range: usize = mapper[2].parse().unwrap();

                    arr.iter_mut().enumerate().for_each(|(idx, ele)| {
                        let within_range = &source <= ele && ele <= &mut (source + range);

                        if within_range && solved[idx] == false {
                            let distance = *ele - source;
                            *ele = destination + distance;
                            solved[idx] = true;
                        }
                        // print!("{ele} ");
                        ()
                    });
                    // println!("");
                    ()
                });
            });
        // println!("{:?}", arr);

        Some(*arr.iter().min().unwrap())
    } else {
        None
    }
}

pub fn solution2(input: &str) -> Option<usize> {
    if let Some((seeds, maps)) = input.split_once("\n\n") {
        let ids = seeds
            .split_ascii_whitespace()
            .filter_map(|id| id.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let maps = maps
            .split("\n\n")
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|l| {
                        l.split_ascii_whitespace()
                            .filter_map(|num| num.parse::<usize>().ok())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut arr = ids
            .chunks_exact(2)
            .map(|ele| ele[0]..(ele[0] + ele[1]))
            .collect::<Vec<_>>();

        for mut map in maps {
            map.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

            let mut idx = 0;
            loop {
                if let None = arr.get_mut(idx) {
                    break;
                }

                let current_range = arr[idx].clone();

                for m in map.iter() {
                    let destination = m[0];
                    let source = m[1];
                    let length = m[2];
                    let range = source..(source + length);

                    let current_start = current_range.start;
                    let current_end = current_range.end - 1;

                    let start_distance = current_start.saturating_sub(source);
                    let end_distance = current_end.saturating_sub(source);

                    if range.contains(&current_start) && range.contains(&current_end) {
                        arr[idx] = (destination + start_distance)..(destination + end_distance);

                        // println!("Range Contained -> {:?}", current_range);
                        // println!("Output Range -> {:?}", arr[idx]);

                        break;
                    } else if range.contains(&current_start) && !range.contains(&current_end) {
                        arr[idx] = (destination + start_distance)..(destination + length);
                        let next_range = (source + length)..current_end + 1;

                        // println!(
                        //     "Start Included, End Excluded -> Split Range: {:?} -> {:?}",
                        //     current_start..source + length,
                        //     next_range
                        // );
                        // println!("Output Range -> {:?}", arr[idx]);

                        arr.insert(idx + 1, next_range);
                        // solved.insert(idx + 1, false);
                        break;
                    } else if !range.contains(&current_start) && range.contains(&current_end) {
                        arr[idx] = (destination)..(destination + end_distance);
                        let next_range = (current_start)..(source);

                        // println!(
                        //     "Start Excluded, End Included -> Split Range: {:?} -> {:?}",
                        //     source..source + end_distance,
                        //     next_range
                        // );
                        // println!("Output Range -> {:?}", arr[idx]);

                        arr.insert(idx + 1, next_range);
                        // solved.insert(idx + 1, false);
                        break;
                    }
                    ()
                }
                idx += 1;
            }
        }

        let result = arr.iter().map(|r| r.start).min().unwrap();
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let example = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";
        let result = solution1(example).unwrap();
        assert_eq!(35, result);
    }

    #[test]
    fn test2() {
        let example = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        if let Some(result) = solution2(example) {
            assert_eq!(46, result);
        } else {
            assert!(false)
        };
    }
}
