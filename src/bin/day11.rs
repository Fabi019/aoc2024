use std::collections::HashMap;

aoc2024::main!("../../assets/day11.txt");

fn part1(input: &str) -> u64 {
    let mut stones = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<_, _>>();

    blink(&mut stones, 25);

    stones.into_iter().fold(0, |acc, (_, count)| acc + count)
}

fn blink(stones: &mut HashMap<u64, u64>, times: usize) {
    for _ in 0..times {
        for (stone, count) in stones.drain().collect::<Vec<_>>() {
            if stone == 0 {
                *stones.entry(1).or_default() += count;
            } else {
                let mut digits = 1;
                let mut offset = 10;
                while stone >= offset {
                    digits += 1;
                    offset *= 10;
                }

                if digits % 2 == 0 {
                    let divisor = 10u64.pow(digits / 2);
                    *stones.entry(stone / divisor).or_default() += count;
                    *stones.entry(stone % divisor).or_default() += count;
                } else {
                    *stones.entry(stone * 2024).or_default() += count;
                }
            }
        }
    }
}

fn part2(input: &str) -> u64 {
    let mut stones = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<_, _>>();

    blink(&mut stones, 75);

    stones.into_iter().fold(0, |acc, (_, count)| acc + count)
}

aoc2024::test!(
    "\
125 17
",
    55312,
    65601038650482
);
