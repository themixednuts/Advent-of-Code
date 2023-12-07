pub fn parser_part_1(str: &str) -> usize {
    let mut sum: usize = 0;
    for line in str.lines() {
        let mut num_array: Vec<usize> = vec![];
        for char in line.trim().chars() {
            if char.is_ascii_digit() {
                num_array.push(char.to_digit(10).unwrap() as usize);
                // println!("{char}");
            }
        }
        if let Some(first_num) = num_array.first() {
            sum += first_num * 10;
        };
        if let Some(last_num) = num_array.last() {
            sum += last_num;
        }
        // let inner_num = inner_sum.join();
    }
    sum
}
