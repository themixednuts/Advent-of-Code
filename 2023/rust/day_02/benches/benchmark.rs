#[macro_use]
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parser1(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day2.txt");
    c.bench_function("parser1", |b| b.iter(|| parser1(str)));
}
fn bench_parser2(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day2.txt");
    c.bench_function("parser2", |b| b.iter(|| parser2(str)));
}

criterion_group!(benches, bench_parser1, bench_parser2);
criterion_main!(benches);
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
