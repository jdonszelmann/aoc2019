use aoc2019::day09::challenge1::main_func;
use criterion::{criterion_group, criterion_main, Criterion};

fn test_day09_challenge2(c: &mut Criterion) {
    c.bench_function("test_day09_challenge2", |b| {
        b.iter(|| {
            let input = include_str!("../src/day09/input");
            let result = main_func(input, 2);
            assert_eq!(result, vec![33343]);
        })
    });
}

fn test_day09_challenge2_victor(c: &mut Criterion) {
    c.bench_function("test_day09_challenge2_victor", |b| {
        b.iter(|| {
            let input = include_str!("victorsday9input");
            let result = main_func(input, 2);
            assert_eq!(result, vec![32869]);
        })
    });
}

criterion_group!(benches, test_day09_challenge2_victor, test_day09_challenge2);
criterion_main!(benches);
