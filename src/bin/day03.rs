use regex::Regex;

aoc2024::main!("../../assets/day03.txt");

fn part1(input: &str) -> u32 {
    input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            if let Some(colon) = &input[i..].find(",") {
                if let Ok(a) = &input[i + 4..i + colon].parse::<u32>() {
                    if let Some(par) = &input[i + colon..].find(")") {
                        if let Ok(b) = &input[i + colon + 1..i + colon + par].parse::<u32>() {
                            return Some(a * b);
                        }
                    }
                }
            }

            None
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap();

    re.captures_iter(input)
        .fold((true, 0), |(enabled, acc), c| {
            let op = c.get(1).unwrap();

            match op.as_str() {
                "mul" if enabled => {
                    let a = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    let b = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
                    (enabled, acc + a * b)
                }
                "do" => (true, acc),
                "don't" => (false, acc),
                _ => (enabled, acc),
            }
        })
        .1
}

aoc2024::test!(
    "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
",
    161,
    "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
",
    48
);
