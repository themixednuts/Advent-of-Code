extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use day_06::{solution1, solution2};

fn bench_parser1(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day6.txt");
    c.bench_function("day6_1", |b| b.iter(|| solution1(str)));
}
fn bench_parser2(c: &mut Criterion) {
    let str = include_str!("../../../inputs/day6.txt");
    c.bench_function("day6_2", |b| b.iter(|| solution2(str)));
}

criterion_group!(benches, bench_parser1, bench_parser2);
criterion_main!(benches);
