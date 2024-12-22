use std::{
    arch::x86_64::{
        __m256i, _mm256_and_si256, _mm256_loadu_si256, _mm256_set1_epi32, _mm256_slli_epi32,
        _mm256_srli_epi32, _mm256_storeu_si256, _mm256_xor_si256,
    },
    collections::{HashMap, HashSet},
};

aoc2024::main!("../../assets/day22.txt");

fn part1(input: &str) -> usize {
    let mut numbers = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    numbers
        .as_mut_slice()
        .chunks_mut(8)
        .fold(0, |mut acc, chunk| {
            let ptr = chunk.as_mut_ptr() as *const __m256i;
            let mut v = unsafe { _mm256_loadu_si256(ptr) };
            let prune = unsafe { _mm256_set1_epi32((1 << 24) - 1) };

            for _ in 0..2000 {
                v = unsafe { _mm256_xor_si256(v, _mm256_slli_epi32(v, 6)) }; // n ^= n << 6
                v = unsafe { _mm256_and_si256(v, prune) }; // n &= (2 << 23) - 1
                v = unsafe { _mm256_xor_si256(v, _mm256_srli_epi32(v, 5)) }; // n ^= n >> 5
                v = unsafe { _mm256_xor_si256(v, _mm256_slli_epi32(v, 11)) }; // n ^= n << 11
                v = unsafe { _mm256_and_si256(v, prune) }; // n &= (2 << 23) - 1
            }

            unsafe {
                _mm256_storeu_si256(ptr as *mut __m256i, v);
            }

            for i in chunk {
                acc += *i as usize;
            }
            acc
        })
}

fn part2(input: &str) -> usize {
    let numbers = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(|mut n| {
            let mut prices = Vec::with_capacity(2000 + 1);
            prices.push(n % 10);

            for _ in 1..=2000 {
                n ^= n << 6;
                n &= (2 << 23) - 1;

                n ^= n >> 5;

                n ^= n << 11;
                n &= (2 << 23) - 1;

                prices.push(n % 10);
            }

            prices
        });

    let mut sums = HashMap::new();

    for prices in numbers {
        let changes = prices
            .windows(2)
            .map(|ab| ab[1] as isize - ab[0] as isize)
            .collect::<Vec<_>>();
        let mut visited = HashSet::new();
        for (i, seq) in changes.windows(4).enumerate() {
            let hash = (seq[0] << 24) + (seq[1] << 16) + (seq[2] << 8) + seq[3];

            // only count first occurence of sequence
            if !visited.insert(hash as u32) {
                continue;
            }

            *sums.entry(hash as u32).or_default() += prices[i + 4];
        }
    }

    *sums.values().max().unwrap()
}

aoc2024::test!(
    "\
1
10
100
2024
",
    37327623,
    "\
1
2
3
2024
",
    23
);
