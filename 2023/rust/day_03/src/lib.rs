pub fn solution1(str: &str) -> usize {
    let mut part_numbers: Vec<usize> = vec![];
    let lines: Vec<&str> = str.trim().split("\n").collect();
    let lines_max = lines.len() - 1;

    for (l_idx, line) in lines.iter().enumerate() {
        let max = line.len().saturating_sub(1);
        let mut current_number: (Vec<usize>, usize, usize) = (vec![], 0, 0);
        for (c_idx, char) in line.char_indices() {
            match char.is_ascii_digit() {
                true => {
                    current_number.0.push(char.to_digit(10).unwrap() as usize);
                    if current_number.1 == 0 {
                        current_number.1 = c_idx;
                    }
                }
                false => {
                    if !current_number.0.is_empty() {
                        current_number.2 = c_idx - 1;
                        let start = current_number.1.saturating_sub(1);
                        let end = current_number.2.saturating_add(1).min(max);
                        let line_start = l_idx.saturating_sub(1);
                        let line_end = l_idx.saturating_add(1).min(lines_max);

                        let relavent_slice = &lines[line_start..=line_end]
                            .iter()
                            .map(|&l| &l[start..=end])
                            .collect::<Vec<_>>()
                            .join("");

                        if relavent_slice
                            .chars()
                            .any(|c| c.is_ascii_punctuation() && c != '.')
                        {
                            let num = current_number.0.iter().fold(0, |acc, &n| acc * 10 + n);
                            part_numbers.push(num);
                        }
                    }
                    current_number.1 = 0;
                    current_number.0.clear();
                }
            }
        }
    }
    let sum: usize = part_numbers.iter().sum();
    sum
}
pub fn solution2(str: &str) -> usize {
    let mut part_numbers: Vec<usize> = vec![];
    let lines: Vec<&str> = str.trim().split("\n").collect();

    for (l_idx, line) in lines.iter().enumerate() {
        let line_start = l_idx.saturating_sub(1);
        let line_end = l_idx.saturating_add(1);
        for (c_idx, char) in line.char_indices() {
            if char == '*' {
                let start = c_idx.saturating_sub(1);
                let end = c_idx.saturating_add(1);
                let search_box = &lines[line_start..=line_end]
                    .iter()
                    .flat_map(|&l| {
                        let mut start = start;
                        let mut end = end;

                        let l_chars: Vec<char> = l.chars().collect();

                        if l_chars[start].is_ascii_digit() {
                            while start > 0 && l_chars[start.saturating_sub(1)].is_ascii_digit() {
                                start = start.saturating_sub(1);
                            }
                        }

                        let l_end = l.len() - 1;
                        if l_chars[end].is_ascii_digit() {
                            while end < l_end && l_chars[end].is_ascii_digit() {
                                end = end.saturating_add(1);
                            }
                        }

                        l[start..end]
                            .split(|c: char| c.is_ascii_punctuation())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                if search_box.len() == 2 {
                    part_numbers.push(search_box.iter().product())
                }
            }
        }
    }
    part_numbers.iter().sum()
}
