#[macro_use]
extern crate criterion;
extern crate txdo;

use criterion::{Criterion, black_box};
use txdo::TodoItem;

const SAMPLE_FILE: &str = include_str!("sample.txt");

fn parse_file() -> () {
    let lines: Vec<&str> = SAMPLE_FILE
        .split("\n")
        .collect::<Vec<_>>();

    let byte_lines = lines
        .into_iter()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let parsed = byte_lines.iter()
        .map(|line| TodoItem::parse(&line))
        .collect::<Vec<_>>();

    black_box(parsed);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_file", |b| b.iter(|| parse_file()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
