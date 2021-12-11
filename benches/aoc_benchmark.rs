use advent_of_code::solve;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

pub fn criterion_benchmark(c: &mut Criterion) {
    let end_day = 11;
    for day in 1..=end_day {
        let input_path = format!("src/year2021/day{:02}_input.txt", day);
        let input = read_to_string(input_path).unwrap();

        for part in 1..=2 {
            let benchmark_name = format!("2021_{:02}_{}", day, part);
            c.bench_function(&benchmark_name, |b| {
                b.iter(|| solve(black_box(day), black_box(part), black_box(&input)));
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
