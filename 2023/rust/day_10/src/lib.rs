use std::collections::HashMap;

pub fn solution1(input: &str) -> usize {
    let mut map: HashMap<
        (usize, usize),
        ((usize, usize, usize, usize), (usize, usize, usize, usize)),
    > = HashMap::new();
    let mut start: (usize, usize) = (0, 0);
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(c_idx, c)| {
            match c {
                'L' => {
                    map.insert((i, c_idx), ((0, 0, 1, 1), (1, 1, 0, 0)));
                }
                '|' => {
                    map.insert((i, c_idx), ((1, 0, 1, 0), (1, 0, 1, 0)));
                }
                '7' => {
                    map.insert((i, c_idx), ((1, 1, 0, 0), (0, 0, 1, 1)));
                }
                'F' => {
                    map.insert((i, c_idx), ((1, 0, 0, 1), (0, 1, 1, 0)));
                }
                'J' => {
                    map.insert((i, c_idx), ((0, 1, 1, 0), (1, 0, 0, 1)));
                }
                '-' => {
                    map.insert((i, c_idx), ((0, 1, 0, 1), (0, 1, 0, 1)));
                }
                'S' => {
                    start = (i, c_idx);
                    map.insert((i, c_idx), ((1, 1, 1, 1), (1, 1, 1, 1)));
                }
                _ => {}
            };
        });
    });

    let mut seeker = (start.clone(), 0);

    loop {
        if seeker.0 == start && seeker.1 != 0 {
            break;
        }
        match seeker.0 {
            p if visited.get(&(p.0.saturating_sub(1), p.1)).is_none()
                && map
                    .get(&(p.0.saturating_sub(1), p.1))
                    .is_some_and(|pos| pos.0 .0 == 1)
                && map.get(&p).is_some_and(|pos| pos.1 .0 == 1) =>
            {
                seeker.0 = (p.0 - 1, p.1);
                seeker.1 += 1;
                visited.insert(seeker.0, true);
            }
            p if visited.get(&(p.0, p.1.saturating_add(1))).is_none()
                && map
                    .get(&(p.0, p.1.saturating_add(1)))
                    .is_some_and(|pos| pos.0 .1 == 1)
                && map.get(&p).is_some_and(|pos| pos.1 .1 == 1) =>
            {
                seeker.0 = (p.0, p.1 + 1);
                seeker.1 += 1;
                visited.insert(seeker.0, true);
            }
            p if visited.get(&(p.0.saturating_add(1), p.1)).is_none()
                && map
                    .get(&(p.0.saturating_add(1), p.1))
                    .is_some_and(|pos| pos.0 .2 == 1)
                && map.get(&p).is_some_and(|pos| pos.1 .2 == 1) =>
            {
                seeker.0 = (p.0 + 1, p.1);
                seeker.1 += 1;
                visited.insert(seeker.0, true);
            }
            p if visited.get(&(p.0, p.1.saturating_sub(1))).is_none()
                && map
                    .get(&(p.0, p.1.saturating_sub(1)))
                    .is_some_and(|pos| pos.0 .3 == 1)
                && map.get(&p).is_some_and(|pos| pos.1 .3 == 1) =>
            {
                seeker.0 = (p.0, p.1 - 1);
                seeker.1 += 1;
                visited.insert(seeker.0, true);
            }
            _ => {}
        }
    }
    return seeker.1;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let ex = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let res = solution1(ex);
        assert_eq!(8, res);
    }
}
