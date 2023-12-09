pub fn solution1(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let mut vec = l
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<isize>().ok())
                .collect::<Vec<_>>();

            differences_push(&mut vec).last().unwrap().to_owned()
        })
        .sum()
}

pub fn solution2(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let mut vec = l
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<isize>().ok())
                .collect::<Vec<_>>();

            differences_prepend(&mut vec).first().unwrap().to_owned()
        })
        .sum()
}

fn differences_push(v: &mut Vec<isize>) -> &mut Vec<isize> {
    if v.iter().all(|&x| x == 0) {
        return v;
    }

    let mut diffs: Vec<isize> = v.windows(2).map(|w| w[1] - w[0]).collect();
    let result = differences_push(&mut diffs);
    v.push(result.last().unwrap() + v.last().unwrap());
    v
}
fn differences_prepend(v: &mut Vec<isize>) -> &mut Vec<isize> {
    if v.iter().all(|&x| x == 0) {
        return v;
    }

    let mut diffs: Vec<isize> = v.windows(2).map(|w| w[1] - w[0]).collect();
    let result = differences_prepend(&mut diffs);
    v.insert(0, v.first().unwrap() - result.first().unwrap());
    v
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let ex = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let res = solution1(ex);
        // dbg!(&res);
        assert_eq!(114, res);
    }

    #[test]
    fn test2() {
        let ex = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let res = solution2(ex);
        // dbg!(&res);
        assert_eq!(2, res);
    }
}
