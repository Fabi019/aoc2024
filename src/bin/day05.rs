use std::cmp::Ordering;

aoc2024::main!("../../assets/day05.txt");

fn part1(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .collect::<Vec<_>>();

    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .fold(0, |acc, u| {
            if is_correct(&u, &rules) {
                return acc + u[u.len() / 2].parse::<u32>().unwrap();
            }
            acc
        })
}

fn is_correct(update: &[&str], rules: &[(&str, &str)]) -> bool {
    for (index, i) in update.iter().enumerate() {
        let rules = rules.iter().filter_map(|(a, b)| {
            if a == i {
                return Some((b, true));
            } else if b == i {
                return Some((a, false));
            }
            None
        });

        for (ab, lg) in rules {
            if let Some(pos) = update.iter().position(|i| i == ab) {
                if pos < index && lg || pos > index && !lg {
                    return false;
                }
            }
        }
    }

    true
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .collect::<Vec<_>>();

    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .fold(0, |acc, mut u| {
            if !is_correct(&u, &rules) {
                u.sort_by(|first, second| {
                    if rules.iter().any(|(a, b)| a == first && b == second) {
                        return Ordering::Less;
                    } else if rules.iter().any(|(a, b)| a == second && b == first) {
                        return Ordering::Greater;
                    }

                    Ordering::Equal
                });

                return acc + u[u.len() / 2].parse::<u32>().unwrap();
            }
            acc
        })
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
