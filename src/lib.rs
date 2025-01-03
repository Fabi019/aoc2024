use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[macro_export]
macro_rules! main {
    ($input:expr) => {
        static INPUT: &str = include_str!($input);

        fn main() {
            let now = std::time::Instant::now();
            println!("Part 1: {:?} ({:?})", part1(INPUT), now.elapsed());

            let now = std::time::Instant::now();
            println!("Part 2: {:?} ({:?})", part2(INPUT), now.elapsed());
        }
    };
}

#[macro_export]
macro_rules! test {
    ($input:expr, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input), $part2);
            }
        }
    };
    ($input1:expr, $part1:expr, $input2:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input1), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input2), $part2);
            }
        }
    };
}

#[macro_export]
macro_rules! benchs {
    ($($day:ident),*) => {
        $(
            aoc2024::bench!($day);
        )*

        criterion_group!(benchmarks, $($day::bench,)*);
    };
}

#[macro_export]
macro_rules! bench {
    ($day:ident) => {
        mod $day {
            use criterion::Criterion;

            include!(concat!("../src/bin/", stringify!($day), ".rs"));

            pub fn bench(c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!($day));
                group.bench_function("1", |b| b.iter(|| part1(INPUT)));
                group.bench_function("2", |b| b.iter(|| part2(INPUT)));
                group.finish();
            }
        }
    };
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
