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
        .filter(|(e, ns)| check(ns, *e, false))
        .map(|(e, _)| e)
        .sum::<u64>()
}

fn check(nums: &[u64], sum: u64, p2: bool) -> bool {
    if nums.is_empty() {
        return sum == 0;
    }

    let (cur, nums) = nums.split_last().unwrap();

    sum % cur == 0 && check(nums, sum / cur, p2)
        || sum >= *cur && check(nums, sum - cur, p2)
        || p2 && {
            let mut offset = 10;
            while cur >= &offset {
                offset *= 10;
            }
            sum % offset == *cur && check(nums, sum / offset, p2)
        }
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
        .filter(|(e, ns)| check(ns, *e, true))
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
