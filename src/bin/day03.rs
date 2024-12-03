use regex::Regex;

aoc2024::main!("../../assets/day03.txt");

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for c in re.captures_iter(input) {
        let first = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let second = c.get(2).unwrap().as_str().parse::<u32>().unwrap();

        sum += first * second;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for c in re.captures_iter(input) {
        let op = c.get(1).unwrap();

        match op.as_str() {
            "mul" if enabled => {
                let a = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let b = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
                sum += a * b;
            }
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {}
        }
    }

    sum
}

aoc2024::test!(
    "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
", 161,
    "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
", 48
);
