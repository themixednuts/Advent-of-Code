extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day_05::{solution1, solution2};

fn bench_parser1(c: &mut Criterion) {
    const INPUT: &str = include_str!("../../../inputs/day5.txt");
    c.bench_function("day5_1", |b| b.iter(|| solution1(INPUT)));
}
fn bench_parser2(c: &mut Criterion) {
    let input = include_str!("../../../inputs/day5.txt");
    let sanitized = input.replace("\r\n", "\n");
    c.bench_function("day5_2", |b| b.iter(|| solution2(&sanitized)));
}
fn bench_parser3(c: &mut Criterion) {
    let input = include_str!("../../../inputs/day5.txt");
    let sanitized = input.replace("\r\n", "\n");
    c.bench_function("day5_2_black_box", |b| {
        b.iter(|| solution2(black_box(&sanitized)))
    });
}
fn bench_parser4(c: &mut Criterion) {
    c.bench_function("day5_2_reading_content", |b| {
        b.iter(|| {
            let input = include_str!("../../../inputs/day5.txt");
            let sanitized = input.replace("\r\n", "\n");
            solution2(black_box(&sanitized));
        })
    });
}
fn bench_parser5(c: &mut Criterion) {
    let example = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    c.bench_function("day5_2_example", |b| {
        b.iter(|| solution2(black_box(example)))
    });
}

criterion_group!(
    benches,
    bench_parser1,
    bench_parser2,
    bench_parser3,
    bench_parser4,
    bench_parser5
);
criterion_main!(benches);
