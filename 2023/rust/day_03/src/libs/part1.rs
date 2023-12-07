pub fn solution(str: &str) -> usize {
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
