pub fn parser2(str: &str) -> usize {
    let mut sum = 0usize;
    for line in str.lines() {
        let line = line.replace("Game ", "");
        let (_id, ledger) = line.split_once(':').unwrap();

        // let mut id: usize = id.parse().unwrap();
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;

        for records in ledger.split(";").into_iter() {
            for record in records.split(",").into_iter() {
                for (amount, color) in record.trim().split_once(' ').into_iter() {
                    let amount = amount.trim();
                    if let Ok(amt) = amount.parse::<usize>() {
                        match color {
                            "red" if amt > red => red = amt,
                            "green" if amt > green => green = amt,
                            "blue" if amt > blue => blue = amt,
                            _ => {}
                        };
                    }
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}
