use std::collections::HashMap;

aoc2024::main!("../../assets/day11.txt");

fn part1(input: &str) -> usize {
    let mut stones = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut index = 0;
        loop {
            let stone = stones[index];

            if stone == 0 {
                stones[index] = 1;
            } else {
                let mut digits = 1;
                let mut offset = 10;
                while stone >= offset {
                    digits += 1;
                    offset *= 10;
                }
    
                if digits % 2 == 0 {
                    let divisor = 10u64.pow((digits / 2) as u32);
                    stones[index] = stone / divisor;
                    stones.insert(index + 1, stone % divisor);
                    index += 1;
                } else {
                    stones[index] = stone * 2024;
                }
            }

            index += 1;
            if index >= stones.len() {
                break;
            }
        }
    }

    stones.len()
}

fn part2(input: &str) -> u64 {
    let mut stones = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<_, _>>();

    for _ in 0..75 {
        let mut new = HashMap::new();

        for (stone, count) in &stones {
            if stone == &0 {
                *new.entry(1).or_default() += count;
            } else {
                let mut digits = 1;
                let mut offset = 10;
                while stone >= &offset {
                    digits += 1;
                    offset *= 10;
                }
    
                if digits % 2 == 0 {
                    let divisor = 10u64.pow((digits / 2) as u32);
                    //stones[index] = stone / divisor;
                    *new.entry(stone / divisor).or_default() += count;
                    //stones.insert(index + 1, stone % divisor);
                    *new.entry(stone % divisor).or_default() += count;
                    //index += 1;
                } else {
                    //stones[index] = stone * 2024;
                    *new.entry(stone * 2024).or_default() += count;
                }
            }
        }

        stones = new;
    }

    stones.into_iter().fold(0, |acc, (_, count)| {
        acc + count
    })
}

aoc2024::test!(
    "\
125 17
",
    55312,
    "\
",
    0
);
