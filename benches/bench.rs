use criterion::{criterion_group, criterion_main};

// To run individual benchmarks use:
// $ cargo bench --bench bench -- <name>
// where <name> can be like: day07, 07, 07/1, 7/2

aoc2024::bench!(day01);
aoc2024::bench!(day02);

criterion_group!(benches, day01::bench, day02::bench);

criterion_main!(benches);
