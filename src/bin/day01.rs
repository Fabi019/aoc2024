use std::collections::HashMap;

aoc2024::main!("../../assets/day01.txt");

fn part1(input: &str) -> u32 {
    let both = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect::<Vec<_>>();

    let mut left = both
        .iter()
        .map(|e| e.0.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    left.sort_unstable();

    let mut right = both
        .into_iter()
        .map(|e| e.1.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

fn part2(input: &str) -> u32 {
    let both = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect::<Vec<_>>();

    let left = both
        .iter()
        .map(|e| e.0.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let right =
        both.into_iter()
            .map(|e| e.1.parse::<u32>().unwrap())
            .fold(HashMap::new(), |mut acc, i| {
                *acc.entry(i).or_insert(0) += 1;
                acc
            });

    left.into_iter()
        .fold(0, |acc, i| acc + i * right.get(&i).unwrap_or(&0))
}

aoc2024::test!(
    "\
3   4
4   3
2   5
1   3
3   9
3   3
",
    11,
    31
);
