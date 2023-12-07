pub fn solution(str: &str) -> usize {
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
