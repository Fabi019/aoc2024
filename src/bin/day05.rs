use std::cmp::Ordering;

aoc2024::main!("../../assets/day05.txt");

fn part1(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    'next: for update in updates {
        let mut previous = Vec::new();

        for (index, i) in update.iter().enumerate() {
            for (_, b) in rules.iter().filter(|(a, _)| a == i) {
                if previous.contains(b) {
                    continue 'next;
                }
            }

            for (a, _) in rules.iter().filter(|(_, b)| b == i) {
                if let Some(pos) = update.iter().position(|i| i == a) {
                    if pos > index {
                        continue 'next;
                    }
                }
            }

            previous.push(i);
        }

        sum += previous[previous.len() / 2].parse::<u32>().unwrap();
    }

    sum
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    for update in updates {
        let mut new = update.clone();
        let mut incorrect = false;

        'next: for (index, i) in update.iter().enumerate() {
            for (_, b) in rules.iter().filter(|(a, _)| a == i) {
                if let Some(pos) = update.iter().position(|i| i == b) {
                    if pos < index {
                        incorrect = true;
                        break 'next;
                    }
                }
            }

            for (a, _) in rules.iter().filter(|(_, b)| b == i) {
                if let Some(pos) = update.iter().position(|i| i == a) {
                    if pos > index {
                        incorrect = true;
                        break 'next;
                    }
                }
            }
        }

        if incorrect {
            new.sort_by(|first, second| {
                if rules.iter().any(|(a, b)| a == first && b == second) {
                    return Ordering::Less;
                } else if rules.iter().any(|(a, b)| a == second && b == first) {
                    return Ordering::Greater;
                }

                Ordering::Equal
            });
        }

        if incorrect {
            sum += new[new.len() / 2].parse::<u32>().unwrap();
        }
    }

    sum
}

aoc2024::test!(
    "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
    143,
    123
);
