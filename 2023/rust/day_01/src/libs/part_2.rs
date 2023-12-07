pub fn parser_part_2(str: &str) -> usize {
    let mut sum = 0usize;
    for line in str.lines() {
        let mut numeric: Vec<usize> = vec![];
        let line = line.trim();
        for (idx, _char) in line.char_indices() {
            let cursor = &line[idx..];
            if let Some(num) = parse_start(cursor) {
                numeric.push(num);
            };
        }
        // print!("{line} - ");
        if let Some(first_num) = numeric.first() {
            // print!("{first_num}");
            sum += first_num * 10;
        };
        if let Some(last_num) = numeric.last() {
            // print!("{last_num}\n");
            sum += last_num;
        };
    }
    sum
}

fn parse_start(str: &str) -> Option<usize> {
    match str {
        // s if s.starts_with("zero") || s.starts_with("0") => Some(0),
        s if s.starts_with("one") || s.starts_with("1") => Some(1),
        s if s.starts_with("two") || s.starts_with("2") => Some(2),
        s if s.starts_with("three") || s.starts_with("3") => Some(3),
        s if s.starts_with("four") || s.starts_with("4") => Some(4),
        s if s.starts_with("five") || s.starts_with("5") => Some(5),
        s if s.starts_with("six") || s.starts_with("6") => Some(6),
        s if s.starts_with("seven") || s.starts_with("7") => Some(7),
        s if s.starts_with("eight") || s.starts_with("8") => Some(8),
        s if s.starts_with("nine") || s.starts_with("9") => Some(9),
        _ => None,
    }
}
