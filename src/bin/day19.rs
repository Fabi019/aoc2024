use std::collections::HashMap;

aoc2024::main!("../../assets/day19.txt");

fn part1(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();

    designs
        .lines()
        .map(|d| {
            let towel_map: HashMap<u8, Vec<&str>> =
                towels.iter().fold(HashMap::new(), |mut acc, t| {
                    if d.contains(t) {
                        acc.entry(t.as_bytes()[0]).or_default().push(*t)
                    }
                    acc
                });
            check_recursive(&towel_map, d, &mut vec![usize::MAX; d.len()], true)
        })
        .sum()
}

fn check_recursive(
    towels: &HashMap<u8, Vec<&str>>,
    design: &str,
    cache: &mut [usize],
    first: bool,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    let value = cache[design.len() - 1];
    if value != usize::MAX {
        return value;
    }

    let mut ways = 0;
    if let Some(ts) = towels.get(&design.as_bytes()[0]) {
        for towel in ts {
            if let Some(design) = design.strip_prefix(towel) {
                ways += check_recursive(towels, design, cache, first);
                if first && ways > 0 {
                    return 1;
                }
            }
        }
    }

    cache[design.len() - 1] = ways;
    ways
}

fn part2(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();

    designs
        .lines()
        .map(|d| {
            let towel_map: HashMap<u8, Vec<&str>> =
                towels.iter().fold(HashMap::new(), |mut acc, t| {
                    if d.contains(t) {
                        acc.entry(t.as_bytes()[0]).or_default().push(*t)
                    }
                    acc
                });
            check_recursive(&towel_map, d, &mut vec![usize::MAX; d.len()], false)
        })
        .sum()
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
