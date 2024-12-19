use std::collections::HashMap;

aoc2024::main!("../../assets/day19.txt");

fn part1(input: &str) -> u32 {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    let designs = designs.lines();

    let mut possible = 0;

    for design in designs {
        let towels = towels
            .iter()
            .cloned()
            .filter(|d| design.contains(d))
            .collect::<Vec<_>>();
        if check_recursive(&towels, design, &mut HashMap::new()) > 0 {
            possible += 1;
        }
    }

    possible
}

fn check_recursive(towels: &[&str], design: &str, cache: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(ways) = cache.get(design) {
        return *ways;
    }

    let mut ways = 0;
    for towel in towels {
        if design.starts_with(towel) {
            ways += check_recursive(towels, design.strip_prefix(towel).unwrap(), cache);
        }
    }

    cache.insert(design.to_string(), ways);
    ways
}

fn part2(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    let designs = designs.lines();

    let mut possible = 0;

    for design in designs {
        let towels = towels
            .iter()
            .cloned()
            .filter(|d| design.contains(d))
            .collect::<Vec<_>>();
        possible += check_recursive(&towels, design, &mut HashMap::new());
    }

    possible
}

aoc2024::test!(
    "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
    6,
    16
);
