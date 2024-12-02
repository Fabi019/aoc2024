aoc2024::main!("../../assets/day02.txt");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|r| is_safe(r, usize::MAX))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let reports = input.lines().map(|l| {
        l.split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut saves = 0;

    for report in reports {
        if is_safe(&report, usize::MAX) {
            saves += 1;
            continue;
        }

        for i in 0..report.len() {
            if is_safe(&report, i) {
                saves += 1;
                break;
            }
        }
    }

    saves
}

fn is_safe(report: &[u32], skip: usize) -> bool {
    let start = if skip == 0 { 1 } else { 0 };

    let mut inc = false;
    let mut last = report[start];
    let mut index = start;

    for &level in &report[start + 1..] {
        index += 1;
        if skip == index {
            continue;
        }

        let diff = last as i32 - level as i32;

        if index == start + 1 {
            inc = diff > 0
        } else if inc && diff < 0 || !inc && diff > 0 {
            return false;
        }

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        last = level;
    }

    true
}

aoc2024::test!(
    "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
    2,
    4
);
