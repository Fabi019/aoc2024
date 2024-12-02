aoc2024::main!("../../assets/day02.txt");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|r| is_save(r))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let reports = input.lines().map(|l| {
        l.split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut saves = 0;

    'outer: for report in reports {
        if is_save(&report) {
            saves += 1;
            continue;
        }

        for i in 0..report.len() {
            let new_report = [&report[0..i], &report[i + 1..report.len()]].concat();
            if is_save(&new_report) {
                saves += 1;
                continue 'outer;
            }
        }
    }

    saves
}

fn is_save(report: &[u32]) -> bool {
    let mut inc = None;
    let mut last = report.first().unwrap();

    for level in report.iter().skip(1) {
        if let Some(inc) = inc {
            if inc && last > level || !inc && last < level {
                return false;
            }
        } else {
            inc = Some(last < level);
        }

        let diff = last.abs_diff(*level);
        if diff < 1 || diff > 3 {
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
