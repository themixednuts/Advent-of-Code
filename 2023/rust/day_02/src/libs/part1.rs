pub fn parser1(str: &str) -> usize {
    let mut sum = 0usize;
    for line in str.lines() {
        let line = line.replace("Game ", "");
        let (id, ledger) = line.split_once(':').unwrap();

        let mut id: usize = id.parse().unwrap();
        'outer: for records in ledger.split(";").into_iter() {
            for record in records.split(",").into_iter() {
                for (amount, color) in record.trim().split_once(' ').into_iter() {
                    let amount = amount.trim();
                    if let Ok(amt) = amount.parse::<usize>() {
                        match color {
                            "red" => {
                                if amt > 12 {
                                    id = 0;
                                    break 'outer;
                                }
                            }
                            "green" => {
                                if amt > 13 {
                                    id = 0;
                                    break 'outer;
                                }
                            }
                            "blue" => {
                                if amt > 14 {
                                    id = 0;
                                    break 'outer;
                                }
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
        sum += id;
    }
    sum
}
