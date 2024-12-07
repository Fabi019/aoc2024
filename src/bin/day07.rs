aoc2024::main!("../../assets/day07.txt");

fn part1(input: &str) -> u64 {
    let input = input.lines().map(|l| {
        let (e, ns) = l.split_once(": ").unwrap();
        (
            e.parse::<u64>().unwrap(),
            ns.split(" ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    input
        .into_iter()
        .filter(|(e, ns)| check(ns, 0, 0, *e, false))
        .map(|(e, _)| e)
        .sum::<u64>()
}

fn check(nums: &[u64], index: usize, sum: u64, exp: u64, p2: bool) -> bool {
    if index >= nums.len() {
        return sum == exp;
    }

    if sum > exp {
        return false;
    }

    let cur = nums[index];

    let comb = if p2 && sum > 0 {
        let mut offset = 10;
        while cur >= offset {
            offset *= 10;
        }
        sum * offset + cur
    } else {
        0
    };

    check(nums, index + 1, sum + cur, exp, p2)
        || check(nums, index + 1, sum * cur, exp, p2)
        || p2 && sum > 0 && check(nums, index + 1, comb, exp, p2)
}

fn part2(input: &str) -> u64 {
    let input = input.lines().map(|l| {
        let (e, ns) = l.split_once(": ").unwrap();
        (
            e.parse::<u64>().unwrap(),
            ns.split(" ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    input
        .into_iter()
        .filter(|(e, ns)| check(ns, 0, 0, *e, true))
        .map(|(e, _)| e)
        .sum::<u64>()
}

aoc2024::test!(
    "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
    3749,
    11387
);
