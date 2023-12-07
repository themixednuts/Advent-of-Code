extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

use day_04::{solution1, solution2, solution2_append};

fn bench_parser1(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day4.txt");
    c.bench_function("day4_1", |b| b.iter(|| solution1(str)));
}
fn bench_parser2(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day4.txt");
    c.bench_function("day4_2", |b| b.iter(|| solution2(str)));
}
fn bench_parser3(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day4.txt");
    c.bench_function("day4_2_add", |b| b.iter(|| solution2_append(str)));
}

criterion_group!(benches, bench_parser1, bench_parser2, bench_parser3);
criterion_main!(benches);
