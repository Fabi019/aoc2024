use std::collections::HashMap;

aoc2024::main!("../../assets/day01.txt");

fn part1(input: &str) -> u32 {
    let both = input.lines().map(|line| line.split_once("   ").unwrap());

    let mut left = Vec::new();
    let mut right = Vec::new();

    for (l, r) in both {
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

fn part2(input: &str) -> u32 {
    let both = input.lines().map(|line| line.split_once("   ").unwrap());

    let mut left = Vec::new();
    let mut right = HashMap::new();

    for (l, r) in both {
        left.push(l.parse::<u32>().unwrap());
        *right.entry(r.parse::<u32>().unwrap()).or_insert(0) += 1;
    }

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
